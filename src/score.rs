use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{AppState, During};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameData::init());
        app.add_systems(Update, (display_score, handle_score_event));
        app.add_systems(Update, update_time.in_set(During::Gameplay));
        app.add_systems(OnEnter(AppState::LoadLevel), restart_score_and_timer);
        app.add_event::<IncreaseScore>();
    }
}

#[derive(Resource)]
pub struct GameData {
    score: usize,
    time: Timer,
}

impl GameData {
    // Don't use Default so that it can be kept private
    fn init() -> Self {
        Self {
            score: 0,
            time: Timer::new(Duration::from_secs(60), TimerMode::Once),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.time.finished()
    }
}

#[derive(Event)]
pub struct IncreaseScore;

fn display_score(mut egui_contexts: EguiContexts, game_data: Res<GameData>) {
    let ctx = egui_contexts.ctx_mut();
    let panel = egui::Area::new("display-score".into()).fixed_pos([0.0, 0.0]);
    panel.show(ctx, |ui| {
        ui.label(
            egui::RichText::new(format!("Score: {}", game_data.score))
                .strong()
                .size(36.0),
        );
        let remaining_time = game_data.time.remaining();
        ui.add(
            egui::ProgressBar::new(
                remaining_time.as_secs_f32() / game_data.time.duration().as_secs_f32(),
            )
            .desired_width(150.0)
            .fill(egui::Color32::YELLOW)
            .text(
                egui::RichText::new(format!("Time: {:.1}", remaining_time.as_secs_f32()))
                    .strong()
                    .color(egui::Color32::RED)
                    .size(24.0),
            ),
        );
    });
}

fn handle_score_event(mut reader: EventReader<IncreaseScore>, mut game_data: ResMut<GameData>) {
    for _ in reader.read() {
        game_data.score += 1;
    }
}

fn update_time(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if game_data.time.tick(time.delta()).finished() {
        next_state.set(AppState::GameOver);
    }
}

fn restart_score_and_timer(mut game_data: ResMut<GameData>) {
    *game_data = GameData::init();
}
