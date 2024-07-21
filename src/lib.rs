use bevy::prelude::*;
use bevy_yoleck::prelude::*;
use camera::SwiftDreamsAreMadeForDweebsCameraPlugin;
use player::PlayerPlugin;

mod camera;
mod player;

pub struct SwiftDreamsAreMadeForDweebsPlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for SwiftDreamsAreMadeForDweebsPlugin {
    fn build(&self, app: &mut App) {
        // app.configure_sets(
        // Update,
        // (
        // During::Menu.run_if(|state: Res<State<AppState>>| state.is_menu()),
        // During::Gameplay.run_if(in_state(AppState::Game)),
        // ),
        // );
        app.init_state::<AppState>();
        app.add_plugins(SwiftDreamsAreMadeForDweebsCameraPlugin);
        if self.is_editor {
            app.add_plugins(YoleckSyncWithEditorState {
                when_editor: AppState::Editor,
                when_game: AppState::Game,
            });
        } else {
            // app.add_plugins(MenuPlugin);
            // app.add_plugins(LevelHandlingPlugin);
            // if let Some(start_at_level) = &self.start_at_level {
            // let start_at_level = if start_at_level.ends_with(".yol") {
            // start_at_level.clone()
            // } else {
            // format!("{}.yol", start_at_level)
            // };
            // app.add_systems(
            // Startup,
            // move |mut level_progress: ResMut<LevelProgress>,
            // mut app_state: ResMut<NextState<AppState>>| {
            // level_progress.current_level = Some(start_at_level.clone());
            // app_state.set(AppState::LoadLevel);
            // },
            // );
            // }
        }
        app.add_plugins(PlayerPlugin);
    }
}

#[derive(States, Default, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    #[default]
    MainMenu,
    PauseMenu,
    LevelSelectMenu,
    LoadLevel,
    Editor,
    Game,
    LevelCompleted,
    GameOver,
}
