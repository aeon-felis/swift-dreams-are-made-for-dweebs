use std::{
    mem::{self, Discriminant},
    time::Duration,
};

use bevy::prelude::*;
use bevy_turborand::prelude::*;

use crate::dweeb::Dweeb;

pub struct DweebEffectsPlugin;

impl Plugin for DweebEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(add_effect_to_dweeb);
        app.add_systems(
            Update,
            (handle_effect_discriminant_changes, handle_effect_particles),
        );
    }
}

fn add_effect_to_dweeb(trigger: Trigger<OnInsert, Dweeb>, mut commands: Commands) {
    commands.entity(trigger.entity()).insert((
        DweebEffect::None,
        OldEffect(mem::discriminant(&DweebEffect::None)),
    ));
}

#[derive(Component)]
pub enum DweebEffect {
    None,
    Zs { is_rem: bool },
    Confusion,
    Lightbulb,
}

#[derive(Component)]
struct OldEffect(Discriminant<DweebEffect>);

fn handle_effect_discriminant_changes(
    mut query: Query<(Entity, &DweebEffect, &mut OldEffect)>,
    particles_query: Query<(Entity, &EffectParticle)>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    asset_server: Res<AssetServer>,
) {
    for (entity, effect, mut old) in query.iter_mut() {
        let new_discriminant = mem::discriminant(effect);
        if new_discriminant == old.0 {
            continue;
        }
        old.0 = new_discriminant;

        for (particle_entity, particle) in particles_query.iter() {
            if particle.owner == entity {
                commands.entity(particle_entity).despawn_recursive();
            }
        }

        match effect {
            DweebEffect::None => {}
            DweebEffect::Zs { .. } => {
                commands.spawn((
                    SceneBundle {
                        scene: asset_server.load("Z.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, -4.0, 0.0),
                        ..Default::default()
                    },
                    EffectParticle {
                        owner: entity,
                        timer: {
                            let mut timer = Timer::new(
                                Duration::from_secs_f32(2.0 + 0.5 * global_rng.f32_normalized()),
                                TimerMode::Repeating,
                            );
                            timer.set_elapsed(timer.duration().mul_f32(global_rng.f32()));
                            timer
                        },
                    },
                ));
            }
            DweebEffect::Confusion => {
                commands.spawn((
                    SceneBundle {
                        scene: asset_server.load("Confusion.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, -4.0, 0.0),
                        ..Default::default()
                    },
                    EffectParticle {
                        owner: entity,
                        timer: Default::default(),
                    },
                ));
            }
            DweebEffect::Lightbulb => {
                commands.spawn((
                    SceneBundle {
                        scene: asset_server.load("Lightbulb.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, -4.0, 0.0),
                        ..Default::default()
                    },
                    EffectParticle {
                        owner: entity,
                        timer: Default::default(),
                    },
                ));
            }
        }
    }
}

#[derive(Component)]
struct EffectParticle {
    owner: Entity,
    timer: Timer,
}

fn handle_effect_particles(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EffectParticle, &mut Transform)>,
    owners_query: Query<(&DweebEffect, &GlobalTransform)>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (particle_entity, mut particle, mut particle_transform) in query.iter_mut() {
        let Ok((effect, owner_transform)) = owners_query.get(particle.owner) else {
            commands.entity(particle_entity).despawn_recursive();
            continue;
        };
        match effect {
            DweebEffect::None => {
                commands.entity(particle_entity).despawn_recursive();
            }
            DweebEffect::Zs { is_rem } => {
                let time_multiplier = if *is_rem {
                    10.0 + 2.0 * global_rng.f32_normalized()
                } else {
                    1.0
                };
                let timer_progress = particle
                    .timer
                    .tick(time.delta().mul_f32(time_multiplier))
                    .elapsed_secs()
                    / particle.timer.duration().as_secs_f32();
                let animation_progress = 2.0
                    * if timer_progress < 0.5 {
                        timer_progress
                    } else {
                        1.0 - timer_progress
                    };
                particle_transform.translation =
                    owner_transform.translation() + (1.0 + 0.2 * animation_progress) * Vec3::Y;
            }
            DweebEffect::Confusion | DweebEffect::Lightbulb => {
                particle_transform.translation = owner_transform.translation() + 1.0 * Vec3::Y;
            }
        }
    }
}
