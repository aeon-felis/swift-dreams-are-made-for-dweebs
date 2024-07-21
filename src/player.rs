use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Serialize, Deserialize)]
pub struct IsPlayer;
