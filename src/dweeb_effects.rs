use std::mem::{self, Discriminant};

use bevy::prelude::*;

use crate::dweeb::Dweeb;

pub struct DweebEffectsPlugin;

impl Plugin for DweebEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(add_effect_to_dweeb);
        app.add_systems(Update, (handle_effect_discriminant_changes, handle_effect_particles));
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
    Zs,
}

#[derive(Component)]
struct OldEffect(Discriminant<DweebEffect>);

fn handle_effect_discriminant_changes(
    mut query: Query<(Entity, &DweebEffect, &mut OldEffect)>,
    particles_query: Query<(Entity, &EffectParticle)>,
    mut commands: Commands,
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
            DweebEffect::Zs => {
                commands.spawn((
                    SceneBundle {
                        scene: asset_server.load("Z.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, -4.0, 0.0),
                        ..Default::default()
                    },
                    EffectParticle { owner: entity },
                ));
            }
        }
    }
}

#[derive(Component)]
struct EffectParticle {
    owner: Entity,
}

fn handle_effect_particles(
    mut query: Query<(Entity, &EffectParticle, &mut Transform)>,
    owners_query: Query<(&DweebEffect, &GlobalTransform)>,
    mut commands: Commands,
) {
    for (particle_entity, particle, mut particle_transform) in query.iter_mut() {
        let Ok((effect, owner_transform)) = owners_query.get(particle.owner) else {
            commands.entity(particle_entity).despawn_recursive();
            continue;
        };
        match effect {
            DweebEffect::None => {
                commands.entity(particle_entity).despawn_recursive();
            }
            DweebEffect::Zs => {
                particle_transform.translation = owner_transform.translation() + 1.0 * Vec3::Y;
            }
        }
    }
}
