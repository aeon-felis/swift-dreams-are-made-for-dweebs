use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_yoleck::{
    prelude::*, vpeol::VpeolWillContainClickableChildren, vpeol_3d::Vpeol3dPosition,
};

use crate::util::affix_vpeol_y;

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Desk")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| Desk)
        });
        affix_vpeol_y::<With<Desk>>(app, 1.0);
        app.add_systems(YoleckSchedule::Populate, populate_desk);
    }
}

#[derive(Component)]
pub struct Desk;

fn populate_desk(mut pupulate: YoleckPopulate<(), With<Desk>>, asset_server: Res<AssetServer>) {
    pupulate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(SceneBundle {
                scene: asset_server.load("Desk.glb#Scene0"),
                ..Default::default()
            });
            cmd.insert(RigidBody::Static);
            cmd.insert(Collider::cuboid(2.00, 1.0, 0.8));
        }
    });
}
