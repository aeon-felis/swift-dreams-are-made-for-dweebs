use bevy::{ecs::query::QueryFilter, prelude::*};
use bevy_yoleck::prelude::*;
use bevy_yoleck::vpeol_3d::Vpeol3dPosition;

pub fn affix_vpeol_y<F: 'static + QueryFilter>(app: &mut App, y_value: f32) {
    app.add_yoleck_edit_system(move |mut query: Query<&mut Vpeol3dPosition, F>| {
        for mut position in query.iter_mut() {
            position.0.y = y_value;
        }
    });
}
