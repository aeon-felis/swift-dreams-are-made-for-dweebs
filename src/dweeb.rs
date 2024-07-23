use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_yoleck::{
    prelude::*, vpeol::VpeolWillContainClickableChildren, vpeol_3d::Vpeol3dPosition,
};

use crate::{player_controls::PotentialAttackTarget, util::affix_vpeol_y};

pub struct DweebPlugin;

impl Plugin for DweebPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_entity_type({
            YoleckEntityType::new("Dweeb")
                .with::<Vpeol3dPosition>()
                .insert_on_init(|| {
                    (
                        Dweeb {},
                        PotentialAttackTarget {
                            offset: 0.5 * Vec3::Y,
                        },
                    )
                })
        });
        affix_vpeol_y::<With<Dweeb>>(app, 2.0);
        app.add_systems(YoleckSchedule::Populate, populate_dweeb);
    }
}

#[derive(Component)]
pub struct Dweeb {}

fn populate_dweeb(mut pupulate: YoleckPopulate<(), With<Dweeb>>, asset_server: Res<AssetServer>) {
    pupulate.populate(|ctx, mut cmd, ()| {
        if ctx.is_first_time() {
            cmd.insert(VpeolWillContainClickableChildren);
            cmd.insert(SceneBundle {
                scene: asset_server.load("Dweeb.glb#Scene0"),
                ..Default::default()
            });
            cmd.insert(RigidBody::Dynamic);
            cmd.insert(Collider::capsule(0.5, 1.0));
            cmd.insert(TnuaControllerBundle::default());
            cmd.insert(TnuaAvian3dSensorShape(Collider::cuboid(0.45, 0.0, 0.45)));
        }
    });
}
