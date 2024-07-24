use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_tnua::{prelude::*, TnuaProximitySensor};
use bevy_turborand::prelude::*;
use bevy_yoetz::prelude::*;

use crate::{bed::Bed, dweeb::Dweeb, dweeb_effects::DweebEffect};

pub struct DweebBehaviorPlugin;

impl Plugin for DweebBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YoetzPlugin::<DweebBehavior>::new(FixedUpdate));
        app.observe(add_behavior_to_dweeb);
        app.add_systems(
            FixedUpdate,
            (
                suggest_aweken,
                suggest_idle,
                suggest_sleep,
                suggest_walk_to_bed,
            )
                .in_set(YoetzSystemSet::Suggest),
        );
        app.add_systems(
            FixedUpdate,
            (
                enact_awaken,
                enact_idle,
                enact_jump_on_bed,
                enact_sleep,
                enact_walk_to_bed,
            )
                .in_set(YoetzSystemSet::Act),
        );
        app.add_systems(FixedUpdate, modify_effect);
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
    Sleep {
        #[yoetz(key)]
        bed_entity: Entity,
        #[yoetz(state)]
        stage_is_rem: bool,
        #[yoetz(state)]
        stage_progress: f32,
    },
    Awaken {
        #[yoetz(state)]
        from_rem: bool,
        #[yoetz(state)]
        timer: Timer,
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

fn enact_idle(mut query: Query<&mut TnuaController, With<DweebBehaviorIdle>>) {
    for mut controller in query.iter_mut() {
        controller.basis(gen_walk(Vec3::ZERO));
    }
}

#[allow(clippy::type_complexity)]
fn suggest_walk_to_bed(
    mut query: Query<
        (Entity, &mut YoetzAdvisor<DweebBehavior>, &GlobalTransform),
        Or<(
            With<DweebBehaviorIdle>,
            With<DweebBehaviorWalkToBed>,
            With<DweebBehaviorJumpOnBed>,
        )>,
    >,
    beds_query: Query<(Entity, &Bed, &GlobalTransform)>,
    sleep_on_bed_query: Query<&DweebBehaviorSleep>,
) {
    #[derive(Debug)]
    struct BedStatus {
        position: Vec3,
        closest: Option<(Entity, f32)>,
        demand: f32,
    }
    let mut beds_statuses = HashMap::new();
    for (bed_entity, _, bed_transform) in beds_query.iter() {
        let mut bed_status = BedStatus {
            position: bed_transform.translation(),
            closest: None,
            demand: 0.0,
        };
        for (dweeb_entity, _, dweeb_transform) in query.iter() {
            let distance_sq = dweeb_transform
                .translation()
                .distance_squared(bed_status.position);
            bed_status.demand += 10.0f32.powi(2) / distance_sq;
            if match bed_status.closest {
                Some((_, currrent_distance_sq)) => distance_sq < currrent_distance_sq,
                None => true,
            } {
                bed_status.closest = Some((dweeb_entity, distance_sq));
            }
        }
        beds_statuses.insert(bed_entity, bed_status);
    }
    for DweebBehaviorSleep { bed_entity, .. } in sleep_on_bed_query.iter() {
        beds_statuses.remove(bed_entity);
    }

    for (dweeb_entity, mut advisor, dweeb_transform) in query.iter_mut() {
        for (&bed_entity, bed_status) in beds_statuses.iter() {
            if bed_status
                .closest
                .is_some_and(|(closest_entity, distance_sq)| {
                    closest_entity != dweeb_entity && distance_sq < 3.0f32.powi(2)
                })
            {
                continue;
            }
            let distance_to_bed_sq = bed_status
                .position
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

fn suggest_sleep(
    mut query: Query<(
        &mut YoetzAdvisor<DweebBehavior>,
        &TnuaController,
        &TnuaProximitySensor,
    )>,
    beds_query: Query<(), With<Bed>>,
) {
    for (mut advisor, controller, sensor) in query.iter_mut() {
        if controller.is_airborne().unwrap_or(true) {
            continue;
        }
        let Some(sensor_output) = sensor.output.as_ref() else {
            continue;
        };
        if !beds_query.contains(sensor_output.entity) {
            continue;
        }
        advisor.suggest(
            1000.0,
            DweebBehavior::Sleep {
                bed_entity: sensor_output.entity,
                stage_is_rem: false,
                stage_progress: 0.0,
            },
        );
    }
}

fn enact_sleep(
    mut query: Query<(
        &mut TnuaController,
        &GlobalTransform,
        &mut DweebBehaviorSleep,
    )>,
    beds_query: Query<&GlobalTransform>,
    time: Res<Time>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (mut controller, dweeb_transform, mut sleep) in query.iter_mut() {
        let DweebBehaviorSleep {
            bed_entity,
            stage_is_rem,
            stage_progress,
        } = sleep.as_mut();
        let Ok(bed_transform) = beds_query.get(*bed_entity) else {
            continue;
        };
        let vector = bed_transform.translation() - dweeb_transform.translation();
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: vector.with_y(0.0).clamp_length_max(2.0),
            desired_forward: bed_transform.forward().with_y(0.0).normalize_or_zero(),
            float_height: 1.0,
            ..Default::default()
        });
        *stage_progress += time.delta_seconds()
            * if *stage_is_rem {
                0.3 + 0.1 * global_rng.f32_normalized()
            } else {
                0.15 + 0.05 * global_rng.f32_normalized()
            };
        if 1.0 <= *stage_progress {
            *stage_progress %= 1.0;
            *stage_is_rem = !*stage_is_rem;
        }
    }
}

#[allow(clippy::type_complexity)]
fn modify_effect(
    mut query: Query<
        (
            &mut DweebEffect,
            // NOTE: don't use AnyOf here because we need it to happen not matter what the current
            // strategy is (if it's not one of the following, we'll just remove the effect)
            Option<&DweebBehaviorSleep>,
            Option<&DweebBehaviorAwaken>,
        ),
        With<Dweeb>,
    >,
) {
    for (mut effect, sleep, awaken) in query.iter_mut() {
        *effect = if let Some(sleep) = sleep {
            DweebEffect::Zs {
                is_rem: sleep.stage_is_rem,
            }
        } else if let Some(awaken) = awaken {
            if awaken.from_rem {
                DweebEffect::Lightbulb
            } else {
                DweebEffect::Confusion
            }
        } else {
            DweebEffect::None
        };
    }
}

#[allow(clippy::type_complexity)]
fn suggest_aweken(
    mut query: Query<(
        &mut YoetzAdvisor<DweebBehavior>,
        AnyOf<(&DweebBehaviorSleep, &DweebBehaviorAwaken)>,
    )>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (mut advisor, (sleep, awaken)) in query.iter_mut() {
        if let Some(sleep) = sleep {
            let wait_secs = if sleep.stage_is_rem {
                2.0 + 1.0 * global_rng.f32()
            } else {
                4.0 + 2.0 * global_rng.f32()
            };
            advisor.suggest(
                // Make it less than Sleep's score, so that if we can sleep it'd override it
                900.0,
                DweebBehavior::Awaken {
                    from_rem: sleep.stage_is_rem,
                    timer: Timer::new(Duration::from_secs_f32(wait_secs), TimerMode::Once),
                },
            )
        } else if let Some(awaken) = awaken {
            if !awaken.timer.finished() {
                advisor.suggest(
                    // Make it more than Sleep's score because we are already awake
                    1100.0,
                    DweebBehavior::Awaken {
                        // These fields don't matter because they are both state fields
                        from_rem: Default::default(),
                        timer: Default::default(),
                    },
                )
            }
        }
    }
}

fn enact_awaken(
    mut query: Query<(&mut TnuaController, &mut DweebBehaviorAwaken)>,
    time: Res<Time>,
) {
    for (mut controller, mut awaken) in query.iter_mut() {
        awaken.timer.tick(time.delta());
        controller.basis(gen_walk(Vec3::ZERO));
    }
}
