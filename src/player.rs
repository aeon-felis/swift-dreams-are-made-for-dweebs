use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IsPlayer>();
        app.add_systems(Update, temp_notice_player_name);
    }
}

#[derive(Reflect, Component, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct IsPlayer;

fn temp_notice_player_name(query: Query<&Name, Added<IsPlayer>>) {
    for name in query.iter() {
        info!("{:?} is a player", name);
    }
}
