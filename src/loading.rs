use bevy::prelude::*;
use bevy_yoleck::prelude::YoleckLoadLevel;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadLevel), load_the_level);
    }
}

fn load_the_level(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    commands.spawn(YoleckLoadLevel(asset_server.load("levels/Level.yol")));
    app_state.set(AppState::Game);
}
