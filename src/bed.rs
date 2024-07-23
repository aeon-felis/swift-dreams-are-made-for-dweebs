use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_yoleck::{
    prelude::*, vpeol::VpeolWillContainClickableChildren, vpeol_3d::Vpeol3dPosition,
};

use crate::util::affix_vpeol_y;

pub struct BedPlugin;

impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Bed")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| Bed)
        });
        affix_vpeol_y::<With<Bed>>(app, 2.0);
        app.add_systems(YoleckSchedule::Populate, populate_bed);
    }
}

#[derive(Component)]
pub struct Bed;

fn populate_bed(mut pupulate: YoleckPopulate<(), With<Bed>>, asset_server: Res<AssetServer>) {
    pupulate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(SceneBundle {
                scene: asset_server.load("Bed.glb#Scene0"),
                ..Default::default()
            });
            cmd.insert(RigidBody::Static);
            cmd.insert(Collider::cuboid(1.00, 0.7, 2.0));
        }
    });
}
