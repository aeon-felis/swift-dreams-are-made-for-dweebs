use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_yoetz::prelude::*;

use crate::dweeb::Dweeb;

pub struct DweebBehaviorPlugin;

impl Plugin for DweebBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YoetzPlugin::<DweebBehavior>::new(FixedUpdate));
        app.observe(add_behavior_to_dweeb);
        app.add_systems(FixedUpdate, (suggest_idle,).in_set(YoetzSystemSet::Suggest));
        app.add_systems(FixedUpdate, (enact_idle,).in_set(YoetzSystemSet::Act));
    }
}

#[derive(YoetzSuggestion)]
enum DweebBehavior {
    Idle, // TODO: maybe remove this in the future?
}

fn gen_walk(direction: Vec3) -> TnuaBuiltinWalk {
    TnuaBuiltinWalk {
        desired_velocity: 2.5 * direction,
        desired_forward: direction.normalize_or_zero(),
        float_height: 2.0,
        // cling_distance: todo!(),
        // spring_strengh: todo!(),
        // spring_dampening: todo!(),
        // acceleration: todo!(),
        // air_acceleration: todo!(),
        // coyote_time: todo!(),
        // free_fall_extra_gravity: todo!(),
        // tilt_offset_angvel: todo!(),
        // tilt_offset_angacl: todo!(),
        // turning_angvel: todo!(),
        // max_slope: todo!(),
        ..Default::default()
    }
}

fn add_behavior_to_dweeb(trigger: Trigger<OnInsert, Dweeb>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(YoetzAdvisor::<DweebBehavior>::new(10.0));
}

fn suggest_idle(mut query: Query<&mut YoetzAdvisor<DweebBehavior>>) {
    for mut advisor in query.iter_mut() {
        advisor.suggest(0.0, DweebBehavior::Idle);
    }
}

fn enact_idle(mut query: Query<&mut TnuaController, With<DweebBehaviorIdle>>) {
    for mut controller in query.iter_mut() {
        controller.basis(gen_walk(Vec3::ZERO));
    }
}
