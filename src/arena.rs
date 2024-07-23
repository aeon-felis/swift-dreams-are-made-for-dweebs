use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_arena);
    }
}

fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const SIZE: Vec3 = Vec3::new(200.0, 1.0, 200.0);
    let mut cmd = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy::math::prelude::Cuboid {
            half_size: 0.5 * SIZE,
        })),
        material: materials.add(StandardMaterial::from_color(css::SLATE_GRAY)),
        ..Default::default()
    });
    cmd.insert(RigidBody::Static);
    cmd.insert(Collider::cuboid(SIZE.x, SIZE.y, SIZE.z));
}
