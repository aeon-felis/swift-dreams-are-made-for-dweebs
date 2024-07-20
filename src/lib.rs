use bevy::prelude::*;
use player::PlayerPlugin;

mod player;

pub struct SwiftDreamsAreMadeForDweebsPlugin;

impl Plugin for SwiftDreamsAreMadeForDweebsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}
