use bevy::prelude::*;
use bevy_yoleck::vpeol::prelude::*;
use dolly::prelude::*;

use crate::player::IsPlayer;
use crate::During;

pub struct SwiftDreamsAreMadeForDweebsCameraPlugin;

impl Plugin for SwiftDreamsAreMadeForDweebsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, apply_dolly_camera_controls.in_set(During::Gameplay));
    }
}

#[derive(Component)]
struct CameraController(CameraRig);

fn setup_camera(mut commands: Commands) {
    let mut cmd = commands.spawn_empty();
    cmd.insert(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 50.0)
            .looking_to(Vec3::new(0.0, -3.0, -10.0), Vec3::Y),
        ..Default::default()
    });
    cmd.insert(VpeolCameraState::default());
    cmd.insert(Vpeol3dCameraControl::topdown());
    cmd.insert(CameraController(
        CameraRig::builder()
            .with(Position::default())
            .with(Arm::new([0.0, 10.0, 30.0]))
            .with(Smooth::new_position(1.0))
            .with(LookAt::new([0.0, 0.0, 0.0]).tracking_smoothness(0.5))
            .build(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 1000.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn apply_dolly_camera_controls(
    time: Res<Time>,
    mut camera_query: Query<(&mut CameraController, &mut Transform)>,
    player_query: Query<&GlobalTransform, With<IsPlayer>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_position = player_transform.translation();
    for (mut camera_controller, mut camera_transform) in camera_query.iter_mut() {
        camera_controller.0.driver_mut::<Position>().position = player_position.to_array().into();
        camera_controller.0.driver_mut::<LookAt>().target =
            (player_position + 3.0 * Vec3::Y).to_array().into();
        camera_controller.0.update(time.delta_seconds());
        camera_transform.translation =
            Vec3::from_slice(camera_controller.0.final_transform.position.as_ref());
        camera_transform.rotation =
            Quat::from_slice(camera_controller.0.final_transform.rotation.as_ref());
    }
}
