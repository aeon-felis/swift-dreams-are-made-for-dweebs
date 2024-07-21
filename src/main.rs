// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
// use bevy_gltf_blueprints::BlueprintsPlugin;
use bevy_registry_export::ExportRegistryPlugin;
use blenvy::{BlenvyPlugin, BlueprintInfo, SpawnBlueprint};
use swift_dreams_are_made_for_dweebs::SwiftDreamsAreMadeForDweebsPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));
    app.add_plugins(BlenvyPlugin {
        // registry_save_path: todo!(),
        // registry_component_filter: todo!(),
        // registry_resource_filter: todo!(),
        // aabbs: todo!(),
        // save_component_filter: todo!(),
        // save_resource_filter: todo!(),
        ..Default::default()
    });
    // app.add_plugins(BlueprintsPlugin {
        // legacy_mode: false,
        // // format: todo!(),
        // // library_folder: todo!(),
        // library_folder: "library".into(),
        // // aabbs: todo!(),
        // // material_library: todo!(),
        // // material_library_folder: todo!(),
        // ..Default::default()
    // });
    app.add_plugins(ExportRegistryPlugin::default());
    app.add_plugins(SwiftDreamsAreMadeForDweebsPlugin);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut leak1: Local<Handle<Scene>>,
    // mut leak2: Local<Handle<Scene>>,
) {
    // TODO: load this from the scene file? Or maybe not
    //commands.spawn(Camera3dBundle {
        //transform: Transform::from_xyz(7.0, -6.0, 4.0).looking_at(Vec3::ZERO, Dir3::Y),
        //..Default::default()
    //});
    // *leak1 = asset_server.load("levels/Level.glb#Scene0");
    // *leak2 = asset_server.load("library/Player.glb#Scene0");
    commands.spawn((
            BlueprintInfo::from_path("levels/Level.glb"),
            SpawnBlueprint,
    ));
    // commands.spawn(bevy_gltf_blueprints::BluePrintBundle {
        // blueprint: bevy_gltf_blueprints::BlueprintName("Level.glb")
        // spawn_here: bevy_gltf_blueprints::SpawnHere,
    // });
    //commands.spawn(Camera2dBundle::default());
    //commands.spawn(SpriteBundle {
        //texture: asset_server.load("ducky.png"),
        //..Default::default()
    //});
}
