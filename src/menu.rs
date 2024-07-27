use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui_kbgp::prelude::*;

use crate::{score::GameData, ActionForKbgp, AppState, During};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameUi>();
        app.add_systems(Update, handle_user_kbgp_actions.in_set(During::Gameplay));
        app.add_systems(
            Update,
            (
                prepare_menu,
                menu_header,
                main_menu.run_if(in_state(AppState::MainMenu)),
                pause_menu.run_if(in_state(AppState::PauseMenu)),
                game_over_menu.run_if(in_state(AppState::GameOver)),
                #[cfg(not(target_arch = "wasm32"))]
                exit_button,
                draw_menu,
            )
                .chain()
                .in_set(During::Menu),
        );
    }
}

fn handle_user_kbgp_actions(
    mut egui_contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let egui_context = egui_contexts.ctx_mut();
    let Some(action) = egui_context.kbgp_user_action() else {
        return;
    };
    match action {
        ActionForKbgp::Menu => {
            next_state.set(AppState::PauseMenu);
        }
        ActionForKbgp::RestartLevel => {
            next_state.set(AppState::LoadLevel);
        }
    }
}

#[derive(PartialEq)]
pub enum FocusLabel {
    Start,
    Exit,
    NextLevel,
    BackToMainMenu,
}

#[derive(Resource, Default)]
struct FrameUi(Option<egui::Ui>);

fn prepare_menu(
    state: Res<State<AppState>>,
    mut egui_contexts: EguiContexts,
    mut frame_ui: ResMut<FrameUi>,
) {
    if state.get().is_menu() {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(egui_contexts.ctx_mut(), |ui| {
                let layout = egui::Layout::top_down(egui::Align::Center);
                ui.with_layout(layout, |ui| {
                    let frame = egui::Frame::none();
                    let mut prepared = frame.begin(ui);
                    let style = prepared.content_ui.style_mut();
                    style.text_styles.insert(
                        egui::TextStyle::Button,
                        egui::FontId {
                            size: 32.0,
                            family: egui::FontFamily::Proportional,
                        },
                    );
                    style.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
                    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(32.0);
                    style.visuals.widgets.inactive.rounding = egui::Rounding::same(32.0);
                    frame_ui.0 = Some(prepared.content_ui);
                });
            });
    } else {
        frame_ui.0 = None;
    }
}

fn draw_menu(mut egui_contexts: EguiContexts, mut frame_ui: ResMut<FrameUi>) {
    let Some(frame_ui) = frame_ui.0.take() else {
        return;
    };
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_contexts.ctx_mut(), |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center);
            ui.with_layout(layout, |ui| {
                let frame = egui::Frame::none();
                let mut prepared = frame.begin(ui);
                prepared.content_ui = frame_ui;
                prepared.end(ui);
            });
        });
}

fn menu_header(mut frame_ui: ResMut<FrameUi>) {
    let Some(ui) = frame_ui.0.as_mut() else {
        return;
    };
    ui.add_space(20.0);

    let mut title_text = egui::text::LayoutJob::default();
    title_text.append(
        "Swift Dreams",
        0.0,
        egui::TextFormat {
            font_id: egui::FontId {
                size: 40.0,
                family: egui::FontFamily::Proportional,
            },
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );
    title_text.append(
        "\nare Made for",
        0.0,
        egui::TextFormat {
            font_id: egui::FontId {
                size: 20.0,
                family: egui::FontFamily::Proportional,
            },
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );
    title_text.append(
        "\nDweebs",
        0.0,
        egui::TextFormat {
            font_id: egui::FontId {
                size: 60.0,
                family: egui::FontFamily::Proportional,
            },
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );
    ui.label(title_text);
    ui.add_space(10.0);
}

fn main_menu(mut frame_ui: ResMut<FrameUi>, mut next_state: ResMut<NextState<AppState>>) {
    let Some(ui) = frame_ui.0.as_mut() else {
        return;
    };
    if ui
        .button("Start")
        .kbgp_navigation()
        .kbgp_focus_label(FocusLabel::Start)
        .kbgp_initial_focus()
        .clicked()
    {
        next_state.set(AppState::LoadLevel);
        ui.kbgp_clear_input();
        ui.kbgp_set_focus_label(FocusLabel::NextLevel);
    }
}

fn pause_menu(mut frame_ui: ResMut<FrameUi>, mut next_state: ResMut<NextState<AppState>>) {
    let Some(ui) = frame_ui.0.as_mut() else {
        return;
    };
    if ui
        .button("Resume")
        .kbgp_navigation()
        .kbgp_initial_focus()
        .kbgp_click_released()
        || ui.kbgp_user_action() == Some(ActionForKbgp::Menu)
    {
        next_state.set(AppState::Game);
    }
    if ui.button("Retry").kbgp_navigation().kbgp_click_released() {
        next_state.set(AppState::LoadLevel);
    }
    if ui.button("Main Menu").kbgp_navigation().clicked() {
        next_state.set(AppState::MainMenu);
        ui.kbgp_clear_input();
        ui.kbgp_set_focus_label(FocusLabel::Start);
    }
}

fn game_over_menu(
    mut frame_ui: ResMut<FrameUi>,
    game_data: Res<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Some(ui) = frame_ui.0.as_mut() else {
        return;
    };
    if game_data.is_finished() {
        ui.label(
            egui::RichText::new("Time Out")
                .size(50.0)
                .strong()
                .color(egui::Color32::LIGHT_BLUE),
        );
        ui.label(
            egui::RichText::new(format!(
                "The dweebs have managed\nto scribe {} ideas",
                game_data.score()
            ))
            .size(30.0)
            .strong()
            .color(egui::Color32::LIGHT_GREEN),
        );
    } else {
        ui.label(
            egui::RichText::new("Game Over")
                .size(60.0)
                .strong()
                .color(egui::Color32::RED),
        );
    }
    ui.add_space(20.0);
    if ui.kbgp_user_action() == Some(ActionForKbgp::Menu) {
        ui.kbgp_set_focus_label(FocusLabel::BackToMainMenu);
    }
    if ui
        .button("Retry")
        .kbgp_navigation()
        .kbgp_initial_focus()
        .clicked()
    {
        next_state.set(AppState::LoadLevel);
    }
    if ui.button("Main Menu").kbgp_navigation().clicked() {
        next_state.set(AppState::MainMenu);
        ui.kbgp_clear_input();
        ui.kbgp_set_focus_label(FocusLabel::Start);
    }
}

#[allow(dead_code)]
fn exit_button(mut frame_ui: ResMut<FrameUi>, mut exit: EventWriter<bevy::app::AppExit>) {
    let Some(ui) = frame_ui.0.as_mut() else {
        return;
    };
    if ui
        .button("Exit")
        .kbgp_navigation()
        .kbgp_focus_label(FocusLabel::Exit)
        .clicked()
    {
        exit.send(Default::default());
    }
}
