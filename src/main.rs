// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use avian3d::PhysicsPlugins;
use bevy_egui::EguiPlugin;
use bevy_egui_kbgp::{KbgpNavBindings, KbgpPlugin, KbgpSettings};
use bevy_tnua::controller::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use bevy_yoleck::vpeol_3d::{Vpeol3dPluginForEditor, Vpeol3dPluginForGame};
use bevy_yoleck::{YoleckPluginForEditor, YoleckPluginForGame};
use clap::Parser;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use swift_dreams_are_made_for_dweebs::SwiftDreamsAreMadeForDweebsPlugin;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    app.add_plugins(EguiPlugin);
    app.add_plugins((
        PhysicsPlugins::default(),
        TnuaControllerPlugin::default(),
        TnuaAvian3dPlugin::default(),
    ));

    if args.editor {
        app.add_plugins((YoleckPluginForEditor, Vpeol3dPluginForEditor::topdown()));
    } else {
        app.add_plugins((YoleckPluginForGame, Vpeol3dPluginForGame));
        app.add_plugins(KbgpPlugin);
        app.insert_resource(KbgpSettings {
            disable_default_navigation: true,
            disable_default_activation: false,
            prevent_loss_of_focus: true,
            focus_on_mouse_movement: true,
            allow_keyboard: true,
            allow_mouse_buttons: false,
            allow_mouse_wheel: false,
            allow_mouse_wheel_sideways: false,
            allow_gamepads: true,
            bindings: {
                KbgpNavBindings::default().with_wasd_navigation()
                /*
                .with_key(KeyCode::Escape, KbgpNavCommand::user(ActionForKbgp::Menu))
                .with_key(
                    KeyCode::Back,
                    KbgpNavCommand::user(ActionForKbgp::RestartLevel),
                )
                .with_key(KeyCode::Space, KbgpNavCommand::Click)
                .with_key(KeyCode::J, KbgpNavCommand::Click)
                .with_gamepad_button(
                    GamepadButtonType::Start,
                    KbgpNavCommand::user(ActionForKbgp::Menu),
                )
                .with_gamepad_button(
                    GamepadButtonType::Select,
                    KbgpNavCommand::user(ActionForKbgp::RestartLevel),
                )
                */
            },
        });
    }

    app.add_plugins(SwiftDreamsAreMadeForDweebsPlugin {
        is_editor: args.editor,
        start_at_level: args.level,
    });
    app.run();
}
