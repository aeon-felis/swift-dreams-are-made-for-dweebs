use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinDash, prelude::*};
use leafwing_input_manager::prelude::*;
use ordered_float::OrderedFloat;

use crate::player::IsPlayer;

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.observe(add_controls_to_player);
        app.add_systems(Update, apply_controls);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    Run,
    Jump,
}

#[derive(Component)]
pub struct PotentialAttackTarget {
    pub offset: Vec3,
}

fn add_controls_to_player(trigger: Trigger<OnInsert, IsPlayer>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(InputManagerBundle::<PlayerAction> {
            action_state: Default::default(),
            input_map: {
                let mut input_map = InputMap::default();

                input_map.insert(PlayerAction::Run, VirtualDPad::arrow_keys());
                input_map.insert(PlayerAction::Run, VirtualDPad::wasd());
                input_map.insert(PlayerAction::Run, VirtualDPad::dpad());
                input_map.insert(PlayerAction::Run, DualAxis::left_stick());

                input_map.insert(PlayerAction::Jump, KeyCode::Space);
                input_map.insert(PlayerAction::Jump, KeyCode::KeyJ);
                input_map.insert(PlayerAction::Jump, GamepadButtonType::South);

                input_map
            },
        });
}

fn apply_controls(
    mut query: Query<(
        &ActionState<PlayerAction>,
        &mut TnuaController,
        &GlobalTransform,
    )>,
    attack_targets_query: Query<(&GlobalTransform, &PotentialAttackTarget)>,
) {
    for (input, mut controller, player_transform) in query.iter_mut() {
        let controller = controller.as_mut();

        let desired_velocity = input
            .clamped_axis_pair(&PlayerAction::Run)
            .map(|axis_pair| Vec3::new(axis_pair.x(), 0.0, -axis_pair.y()))
            .unwrap_or_default();
        let desired_direction = Dir3::new(desired_velocity).ok();

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: 10.0 * desired_velocity,
            desired_forward: desired_direction
                .map(|direction| *direction)
                .unwrap_or_default(),
            float_height: 2.0,
            // cling_distance: todo!(),
            // spring_strengh: todo!(),
            // spring_dampening: todo!(),
            // acceleration: todo!(),
            // air_acceleration: todo!(),
            // coyote_time: todo!(),
            // free_fall_extra_gravity: todo!(),
            // tilt_offset_angvel: todo!(),
            // tilt_offset_angacl: todo!(),
            // turning_angvel: todo!(),
            // max_slope: todo!(),
            ..Default::default()
        });

        let attack_target = 'attack_target: {
            if !input.just_pressed(&PlayerAction::Jump) {
                break 'attack_target None;
            }
            if controller.action_name() != Some(TnuaBuiltinJump::NAME) {
                break 'attack_target None;
            }
            if !controller.is_airborne().is_ok_and(|airborne| airborne) {
                break 'attack_target None;
            }
            let player_position = player_transform.translation();
            let attack_direction = desired_direction.unwrap_or_else(|| player_transform.forward());
            attack_targets_query
                .iter()
                .filter_map(|(target_translation, PotentialAttackTarget { offset })| {
                    let vec_to_target = target_translation.translation() - player_position;
                    if vec_to_target == Vec3::ZERO {
                        return None;
                    }
                    let angle = attack_direction.angle_between(vec_to_target).abs();
                    if std::f32::consts::FRAC_PI_2 < angle {
                        return None;
                    }
                    Some((
                        vec_to_target.length_squared() * angle,
                        vec_to_target + *offset,
                    ))
                })
                .min_by_key(|(score, _)| OrderedFloat(*score))
                .map(|(_, v)| (attack_direction, v))
        };

        if let Some((attack_direction, vec_to_target)) = attack_target {
            controller.action(TnuaBuiltinDash {
                displacement: vec_to_target,
                desired_forward: vec_to_target.with_y(0.0).normalize_or(*attack_direction),
                allow_in_air: true,
                speed: 200.0,
                // brake_to_speed: todo!(),
                // acceleration: todo!(),
                // brake_acceleration: todo!(),
                // input_buffer_time: todo!(),
                ..Default::default()
            });
        } else {
            let jump = input.clamped_value(&PlayerAction::Jump);
            if 0.0 < jump {
                controller.action(TnuaBuiltinJump {
                    height: 4.0 * jump,
                    // allow_in_air: todo!(),
                    // upslope_extra_gravity: todo!(),
                    // takeoff_extra_gravity: todo!(),
                    // takeoff_above_velocity: todo!(),
                    // fall_extra_gravity: todo!(),
                    // shorten_extra_gravity: todo!(),
                    // peak_prevention_at_upward_velocity: todo!(),
                    // peak_prevention_extra_gravity: todo!(),
                    // reschedule_cooldown: todo!(),
                    // input_buffer_time: todo!(),
                    ..Default::default()
                });
            }
        }
    }
}
