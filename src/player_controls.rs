use bevy::prelude::*;
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;

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

fn apply_controls(mut query: Query<(&ActionState<PlayerAction>, &mut TnuaController)>) {
    for (input, mut controller) in query.iter_mut() {
        let controller = controller.as_mut();

        let desired_velocity = input
            .clamped_axis_pair(&PlayerAction::Run)
            .map(|axis_pair| Vec3::new(axis_pair.x(), 0.0, -axis_pair.y()))
            .unwrap_or_default();

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: 10.0 * desired_velocity,
            desired_forward: desired_velocity.normalize_or_zero(),
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
