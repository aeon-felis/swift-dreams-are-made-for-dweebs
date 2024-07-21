use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_yoleck::{prelude::*, vpeol_3d::Vpeol3dPosition};
use serde::{Deserialize, Serialize};

use crate::util::affix_vpeol_y;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Player")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| IsPlayer)
        });
        affix_vpeol_y::<With<IsPlayer>>(app, 2.0);
        app.add_systems(YoleckSchedule::Populate, populate_player);
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct IsPlayer;

fn populate_player(
    mut pupulate: YoleckPopulate<(), With<IsPlayer>>,
    asset_server: Res<AssetServer>,
) {
    pupulate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(SceneBundle {
                scene: asset_server.load("Player.glb#Scene0"),
                ..Default::default()
            });
            cmd.insert(RigidBody::Dynamic);
            cmd.insert(Collider::capsule(0.25, 1.0));
            cmd.insert(TnuaControllerBundle::default());
            cmd.insert(TnuaAvian3dSensorShape(Collider::cuboid(0.45, 0.0, 0.45)));
        }
    });
}
