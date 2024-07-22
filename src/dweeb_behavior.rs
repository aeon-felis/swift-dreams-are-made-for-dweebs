use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_yoetz::prelude::*;

use crate::{bed::Bed, dweeb::Dweeb};

pub struct DweebBehaviorPlugin;

impl Plugin for DweebBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YoetzPlugin::<DweebBehavior>::new(FixedUpdate));
        app.observe(add_behavior_to_dweeb);
        app.add_systems(
            FixedUpdate,
            (suggest_idle, suggest_walk_to_bed).in_set(YoetzSystemSet::Suggest),
        );
        app.add_systems(
            FixedUpdate,
            (enact_idle, enact_walk_to_bed, enact_jump_on_bed).in_set(YoetzSystemSet::Act),
        );
    }
}

#[derive(YoetzSuggestion)]
enum DweebBehavior {
    Idle, // TODO: maybe remove this in the future?
    WalkToBed {
        #[yoetz(key)]
        bed_entity: Entity,
    },
    JumpOnBed {
        #[yoetz(key)]
        bed_entity: Entity,
    },
}

fn gen_walk(direction: Vec3) -> TnuaBuiltinWalk {
    TnuaBuiltinWalk {
        desired_velocity: 2.5 * direction,
        desired_forward: direction.normalize_or_zero(),
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
    }
}

fn add_behavior_to_dweeb(trigger: Trigger<OnInsert, Dweeb>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(YoetzAdvisor::<DweebBehavior>::new(10.0));
}

fn suggest_idle(mut query: Query<&mut YoetzAdvisor<DweebBehavior>>) {
    for mut advisor in query.iter_mut() {
        advisor.suggest(0.0, DweebBehavior::Idle);
    }
}

#[allow(clippy::type_complexity)]
fn suggest_walk_to_bed(
    mut query: Query<
        (&mut YoetzAdvisor<DweebBehavior>, &GlobalTransform),
        Or<(
            With<DweebBehaviorIdle>,
            With<DweebBehaviorWalkToBed>,
            With<DweebBehaviorJumpOnBed>,
        )>,
    >,
    beds_query: Query<(Entity, &Bed, &GlobalTransform)>,
) {
    for (mut advisor, dweeb_transform) in query.iter_mut() {
        for (bed_entity, bed, bed_transform) in beds_query.iter() {
            if bed.occupied_by.is_some() {
                continue;
            }
            let distance_to_bed_sq = bed_transform
                .translation()
                .distance_squared(dweeb_transform.translation());
            if 2.0f32.powi(2) < distance_to_bed_sq {
                advisor.suggest(
                    40.0f32.powi(2) / distance_to_bed_sq,
                    DweebBehavior::WalkToBed { bed_entity },
                );
            } else {
                advisor.suggest(100.0, DweebBehavior::JumpOnBed { bed_entity });
            }
        }
    }
}

fn enact_idle(mut query: Query<&mut TnuaController, With<DweebBehaviorIdle>>) {
    for mut controller in query.iter_mut() {
        controller.basis(gen_walk(Vec3::ZERO));
    }
}

fn enact_walk_to_bed(
    mut query: Query<(
        &mut TnuaController,
        &GlobalTransform,
        &DweebBehaviorWalkToBed,
    )>,
    beds_query: Query<&GlobalTransform>,
) {
    for (mut controller, dweeb_transform, walk_to_bed) in query.iter_mut() {
        let Ok(bed_transform) = beds_query.get(walk_to_bed.bed_entity) else {
            continue;
        };
        let vector = bed_transform.translation() - dweeb_transform.translation();
        let direction = vector.with_y(0.0).normalize_or_zero();
        controller.basis(gen_walk(direction));
    }
}

fn enact_jump_on_bed(
    mut query: Query<(
        &mut TnuaController,
        &GlobalTransform,
        &DweebBehaviorJumpOnBed,
    )>,
    beds_query: Query<&GlobalTransform>,
) {
    for (mut controller, dweeb_transform, walk_to_bed) in query.iter_mut() {
        let Ok(bed_transform) = beds_query.get(walk_to_bed.bed_entity) else {
            continue;
        };
        let vector = bed_transform.translation() - dweeb_transform.translation();
        let direction = vector.with_y(0.0).normalize_or_zero();
        controller.basis(TnuaBuiltinWalk {
            // To ensure we get to the correct velocity even from a stop
            acceleration: f32::INFINITY,
            air_acceleration: f32::INFINITY,
            ..gen_walk(direction)
        });
        controller.action(TnuaBuiltinJump {
            height: 2.0,
            //allow_in_air: todo!(),
            //upslope_extra_gravity: todo!(),
            //takeoff_extra_gravity: todo!(),
            //takeoff_above_velocity: todo!(),
            //fall_extra_gravity: todo!(),
            shorten_extra_gravity: 0.0,
            //peak_prevention_at_upward_velocity: todo!(),
            //peak_prevention_extra_gravity: todo!(),
            //reschedule_cooldown: todo!(),
            //input_buffer_time: todo!(),
            ..Default::default()
        });
    }
}
