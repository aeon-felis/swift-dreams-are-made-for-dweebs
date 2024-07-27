use bevy::prelude::*;
use bevy_yoleck::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadLevel), load_the_level);
    }
}

fn load_the_level(
    asset_server: Res<AssetServer>,
    existing_levels_query: Query<Entity, With<YoleckKeepLevel>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for existing_level in existing_levels_query.iter() {
        commands.entity(existing_level).despawn_recursive();
    }
    commands.spawn(YoleckLoadLevel(asset_server.load("levels/Level.yol")));
    app_state.set(AppState::Game);
}
