use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{AppState, During};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScore(0));
        app.insert_resource(GameTime(Default::default()));
        app.add_systems(Update, (display_score, handle_score_event));
        app.add_systems(Update, update_time.in_set(During::Gameplay));
        app.add_systems(OnEnter(AppState::LoadLevel), restart_score_and_timer);
        app.add_event::<IncreaseScore>();
    }
}

#[derive(Resource)]
pub struct GameScore(pub usize);

#[derive(Event)]
pub struct IncreaseScore;

#[derive(Resource)]
pub struct GameTime(Timer);

impl GameTime {
    pub fn is_finished(&self) -> bool {
        self.0.finished()
    }
}

fn display_score(mut egui_contexts: EguiContexts, score: Res<GameScore>, game_time: Res<GameTime>) {
    let ctx = egui_contexts.ctx_mut();
    let panel = egui::Area::new("display-score".into()).fixed_pos([0.0, 0.0]);
    panel.show(ctx, |ui| {
        ui.label(
            egui::RichText::new(format!("Score: {}", score.0))
                .strong()
                .size(36.0),
        );
        let remaining_time = game_time.0.remaining();
        ui.add(
            egui::ProgressBar::new(
                remaining_time.as_secs_f32() / game_time.0.duration().as_secs_f32(),
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

fn handle_score_event(mut reader: EventReader<IncreaseScore>, mut score: ResMut<GameScore>) {
    for _ in reader.read() {
        score.0 += 1;
    }
}

fn update_time(
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if game_time.0.tick(time.delta()).finished() {
        next_state.set(AppState::GameOver);
    }
}

fn restart_score_and_timer(mut game_score: ResMut<GameScore>, mut game_time: ResMut<GameTime>) {
    game_score.0 = 0;
    game_time.0 = Timer::new(Duration::from_secs(60), TimerMode::Once);
}
