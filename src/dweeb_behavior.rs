use std::time::Duration;

use bevy::{
    ecs::query::{QueryData, WorldQuery},
    prelude::*,
    utils::HashMap,
};
use bevy_tnua::{prelude::*, TnuaProximitySensor};
use bevy_turborand::prelude::*;
use bevy_yoetz::prelude::*;

use crate::{bed::Bed, desk::Desk, dweeb::Dweeb, dweeb_effects::DweebEffect, score::IncreaseScore};

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
                suggest_walk_to::<Bed>,
                suggest_walk_to::<Desk>,
                suggest_scribe,
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
                enact_walk_to::<Bed>,
                enact_walk_to::<Desk>,
                enact_scribe,
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
    Startled {
        #[yoetz(state)]
        from_rem: bool,
        #[yoetz(state)]
        timer: Timer,
    },
    WalkToDesk {
        #[yoetz(key)]
        desk_entity: Entity,
    },
    Scribe {
        #[yoetz(key)]
        desk_entity: Entity,
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
        advisor.suggest(f32::NEG_INFINITY, DweebBehavior::Idle);
    }
}

fn enact_idle(mut query: Query<&mut TnuaController, With<DweebBehaviorIdle>>) {
    for mut controller in query.iter_mut() {
        controller.basis(gen_walk(Vec3::ZERO));
    }
}

trait WalkTo: Component {
    type SuggestableFrom: QueryData;

    fn check_suggestable_from(
        _suggestable_from: <Self::SuggestableFrom as WorldQuery>::Item<'_>,
    ) -> bool {
        true
    }

    fn walk_to_position(transform: &GlobalTransform) -> Vec3 {
        transform.translation()
    }

    const TARGET_DISTANCE: f32;

    type DweebUsesDestinationIndicator: Component;

    fn extract_entity_in_use(indicator: &Self::DweebUsesDestinationIndicator) -> Entity;
    fn suggest_walk_to(destination: Entity) -> DweebBehavior;
    fn suggest_use(destination: Entity) -> DweebBehavior;

    type Behavior: Component;

    fn extract_entity_from_behavior(behavior: &Self::Behavior) -> Entity;
}

impl WalkTo for Bed {
    type SuggestableFrom = AnyOf<(
        &'static DweebBehaviorIdle,
        &'static DweebBehaviorWalkToBed,
        &'static DweebBehaviorJumpOnBed,
    )>;

    const TARGET_DISTANCE: f32 = 2.0;

    type DweebUsesDestinationIndicator = DweebBehaviorSleep;

    fn extract_entity_in_use(indicator: &Self::DweebUsesDestinationIndicator) -> Entity {
        indicator.bed_entity
    }

    fn suggest_walk_to(destination: Entity) -> DweebBehavior {
        DweebBehavior::WalkToBed {
            bed_entity: destination,
        }
    }

    fn suggest_use(destination: Entity) -> DweebBehavior {
        DweebBehavior::JumpOnBed {
            bed_entity: destination,
        }
    }

    type Behavior = DweebBehaviorWalkToBed;

    fn extract_entity_from_behavior(behavior: &Self::Behavior) -> Entity {
        behavior.bed_entity
    }
}

impl WalkTo for Desk {
    type SuggestableFrom = AnyOf<(
        &'static DweebBehaviorStartled,
        &'static DweebBehaviorWalkToDesk,
    )>;

    fn check_suggestable_from(
        (startled, _): <Self::SuggestableFrom as WorldQuery>::Item<'_>,
    ) -> bool {
        if let Some(startled) = startled {
            startled.from_rem
        } else {
            true
        }
    }

    const TARGET_DISTANCE: f32 = 0.5;

    fn walk_to_position(transform: &GlobalTransform) -> Vec3 {
        transform.transform_point(1.0 * Vec3::Z)
    }

    type DweebUsesDestinationIndicator = DweebBehaviorScribe;

    fn extract_entity_in_use(indicator: &Self::DweebUsesDestinationIndicator) -> Entity {
        indicator.desk_entity
    }

    fn suggest_walk_to(destination: Entity) -> DweebBehavior {
        DweebBehavior::WalkToDesk {
            desk_entity: destination,
        }
    }

    fn suggest_use(destination: Entity) -> DweebBehavior {
        DweebBehavior::Scribe {
            desk_entity: destination,
            timer: Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once),
        }
    }

    type Behavior = DweebBehaviorWalkToDesk;

    fn extract_entity_from_behavior(behavior: &Self::Behavior) -> Entity {
        behavior.desk_entity
    }
}

#[allow(clippy::type_complexity)]
fn suggest_walk_to<D: WalkTo>(
    mut query: Query<(
        Entity,
        &mut YoetzAdvisor<DweebBehavior>,
        D::SuggestableFrom,
        &GlobalTransform,
    )>,
    destinations_query: Query<(Entity, &D, &GlobalTransform)>,
    uses_destination_query: Query<&D::DweebUsesDestinationIndicator>,
) {
    #[derive(Debug)]
    struct DestinationStatus {
        position: Vec3,
        closest: Option<(Entity, f32)>,
        demand: f32,
    }
    let mut destinations_statuses = HashMap::new();
    for (destination_entity, _, destination_transform) in destinations_query.iter() {
        let mut destination_status = DestinationStatus {
            position: D::walk_to_position(destination_transform),
            closest: None,
            demand: 0.0,
        };
        for (dweeb_entity, _, _, dweeb_transform) in query.iter() {
            let distance_sq = dweeb_transform
                .translation()
                .distance_squared(destination_status.position);
            destination_status.demand += 10.0f32.powi(2) / distance_sq;
            if match destination_status.closest {
                Some((_, currrent_distance_sq)) => distance_sq < currrent_distance_sq,
                None => true,
            } {
                destination_status.closest = Some((dweeb_entity, distance_sq));
            }
        }
        destinations_statuses.insert(destination_entity, destination_status);
    }
    for in_use_indicator in uses_destination_query.iter() {
        destinations_statuses.remove(&D::extract_entity_in_use(in_use_indicator));
    }

    for (dweeb_entity, mut advisor, suggestable_from, dweeb_transform) in query.iter_mut() {
        if !D::check_suggestable_from(suggestable_from) {
            continue;
        }
        for (&destination_entity, destination_status) in destinations_statuses.iter() {
            if destination_status
                .closest
                .is_some_and(|(closest_entity, distance_sq)| {
                    closest_entity != dweeb_entity && distance_sq < 3.0f32.powi(2)
                })
            {
                continue;
            }
            let distance_to_destination_sq = destination_status
                .position
                .xz()
                .distance_squared(dweeb_transform.translation().xz());
            if D::TARGET_DISTANCE.powi(2) < distance_to_destination_sq {
                advisor.suggest(
                    40.0f32.powi(2) / distance_to_destination_sq,
                    D::suggest_walk_to(destination_entity),
                );
            } else {
                advisor.suggest(100.0, D::suggest_use(destination_entity));
            }
        }
    }
}

fn enact_walk_to<D: WalkTo>(
    mut query: Query<(&mut TnuaController, &GlobalTransform, &D::Behavior)>,
    destination_query: Query<&GlobalTransform>,
) {
    for (mut controller, dweeb_transform, walk_to) in query.iter_mut() {
        let Ok(destination_transform) =
            destination_query.get(D::extract_entity_from_behavior(walk_to))
        else {
            continue;
        };
        let vector = D::walk_to_position(destination_transform) - dweeb_transform.translation();
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
            Option<&DweebBehaviorStartled>,
            Has<DweebBehaviorWalkToDesk>,
            Has<DweebBehaviorScribe>,
        ),
        With<Dweeb>,
    >,
) {
    for (mut effect, sleep, startled, walk_to_desk, scribe) in query.iter_mut() {
        *effect = if let Some(sleep) = sleep {
            DweebEffect::Zs {
                is_rem: sleep.stage_is_rem,
            }
        } else if let Some(startled) = startled {
            if startled.from_rem {
                DweebEffect::Lightbulb
            } else {
                DweebEffect::Confusion
            }
        } else if walk_to_desk || scribe {
            DweebEffect::Lightbulb
        } else {
            DweebEffect::None
        };
    }
}

#[allow(clippy::type_complexity)]
fn suggest_aweken(
    mut query: Query<(
        &mut YoetzAdvisor<DweebBehavior>,
        AnyOf<(&DweebBehaviorSleep, &DweebBehaviorStartled)>,
    )>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (mut advisor, (sleep, startled)) in query.iter_mut() {
        if let Some(sleep) = sleep {
            let wait_secs = if sleep.stage_is_rem {
                2.0 + 1.0 * global_rng.f32()
            } else {
                4.0 + 2.0 * global_rng.f32()
            };
            advisor.suggest(
                // Make it less than Sleep's score, so that if we can sleep it'd override it
                900.0,
                DweebBehavior::Startled {
                    from_rem: sleep.stage_is_rem,
                    timer: Timer::new(Duration::from_secs_f32(wait_secs), TimerMode::Once),
                },
            )
        } else if let Some(startled) = startled {
            if !startled.timer.finished() {
                advisor.suggest(
                    // Make it more than Sleep's score because we are already awake
                    1100.0,
                    DweebBehavior::Startled {
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
    mut query: Query<(&mut TnuaController, &mut DweebBehaviorStartled)>,
    time: Res<Time>,
) {
    for (mut controller, mut startled) in query.iter_mut() {
        startled.timer.tick(time.delta());
        controller.basis(gen_walk(Vec3::ZERO));
    }
}

fn suggest_scribe(
    mut query: Query<(
        &mut YoetzAdvisor<DweebBehavior>,
        &DweebBehaviorScribe,
        &GlobalTransform,
    )>,
    desks_query: Query<&GlobalTransform>,
) {
    for (mut advisor, scribe, dweeb_transform) in query.iter_mut() {
        if scribe.timer.finished() {
            continue;
        }
        let Ok(desk_transform) = desks_query.get(scribe.desk_entity) else {
            continue;
        };
        let target_position = Desk::walk_to_position(desk_transform);
        let distance_sq = dweeb_transform
            .translation()
            .xz()
            .distance_squared(target_position.xz());
        if distance_sq <= Desk::TARGET_DISTANCE.powi(2) {
            advisor.suggest(
                1000.0,
                DweebBehavior::Scribe {
                    desk_entity: scribe.desk_entity,
                    timer: Default::default(),
                },
            );
        }
    }
}

fn enact_scribe(
    mut query: Query<(
        &mut TnuaController,
        &GlobalTransform,
        &mut DweebBehaviorScribe,
    )>,
    desks_query: Query<&GlobalTransform>,
    time: Res<Time>,
    mut score_writer: EventWriter<IncreaseScore>,
) {
    for (mut controller, dweeb_transform, mut scribe) in query.iter_mut() {
        let DweebBehaviorScribe { desk_entity, timer } = scribe.as_mut();
        if timer.tick(time.delta()).finished() {
            score_writer.send(IncreaseScore);
            continue;
        }
        let Ok(desk_transform) = desks_query.get(*desk_entity) else {
            continue;
        };
        let vector = Desk::walk_to_position(desk_transform) - dweeb_transform.translation();
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: vector.with_y(0.0).clamp_length_max(2.0),
            desired_forward: desk_transform.forward().with_y(0.0).normalize_or_zero(),
            float_height: 1.5,
            ..Default::default()
        });
    }
}
