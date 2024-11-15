use eframe::egui;
use egui::{Frame, Margin};

#[derive(PartialEq)]
enum DeviceState {
    PendingInit = 0,
    PendingStart = 1,
    Running = 2,
    Complete = 3,
}

pub struct GameApp {
    controller_one_pos: (i32, i32),
    controller_two_pos: (i32, i32),
    state: DeviceState,
    winner: Option<String>,
}

impl Default for GameApp {
    fn default() -> Self {
        Self {
            controller_one_pos: (0, 0),
            controller_two_pos: (0, 0),
            state: DeviceState::PendingInit,
            winner: None,
        }
    }
}
impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.state {
            DeviceState::PendingInit => {
                display_welcome(ui);
                if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
                    self.state = DeviceState::PendingStart;
                }
            }
            DeviceState::PendingStart => {
                display_starting(ui);
                if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
                    self.state = DeviceState::Running;
                }
            }
            DeviceState::Running => {
                display_running(ui, self.controller_one_pos, self.controller_two_pos);
                if ctx.input(|input| input.key_pressed(egui::Key::Space)) {
                    self.state = DeviceState::Complete;
                }
            }
            DeviceState::Complete => {
                display_complete(ui, &self.winner);
                if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
                    self.state = DeviceState::PendingStart;
                } else if ctx.input(|input| input.key_pressed(egui::Key::Space)) {
                    self.state = DeviceState::PendingInit;
                }
            }
        });
    }
}

fn display_welcome(ui: &mut egui::Ui) {
    Frame::default()
        .inner_margin(Margin::same(225.0))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Welcome").size(60.0).strong(),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new("Initializing controller 0 and 1...")
                        .size(20.0)
                        .italics(),
                ));
            })
        });
}

fn display_starting(ui: &mut egui::Ui) {
    Frame::default()
        .inner_margin(Margin::same(225.0))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Welcome").size(60.0).strong(),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new("Initialized controller 0 and 1...")
                        .size(20.0)
                        .italics(),
                ));
            });
            ui.add_space(200.0);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Press Enter to start")
                        .size(20.0)
                        .strong()
                        .italics(),
                ));
            });
        });
}

fn display_running(
    ui: &mut egui::Ui,
    controller_one_pos: (i32, i32),
    controller_two_pos: (i32, i32),
) {
    Frame::default()
        .inner_margin(Margin::same(60.0))
        .show(ui, |ui| {
            egui::Grid::new("controllers_grid")
                .num_columns(2)
                .spacing([0.0, 100.0])
                .show(ui, |ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new("Controller 0").size(40.0).strong(),
                    ));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("Controller 1").size(40.0).strong(),
                        ));
                    });
                    ui.end_row();

                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{:?}", controller_one_pos)).size(40.0),
                    ));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("{:?}", controller_two_pos)).size(40.0),
                        ));

                        ui.end_row();
                    });
                });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Press Space to stop")
                        .size(20.0)
                        .strong()
                        .italics(),
                ));
            });
        });
}

fn display_complete(ui: &mut egui::Ui, winner: &Option<String>) {
    Frame::default()
        .inner_margin(Margin::same(225.0))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Congrats!").size(60.0).strong(),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new(format!("The winner is {:?}", *winner))
                        .size(20.0)
                        .italics(),
                ));
            });
            ui.add_space(200.0);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Press Enter to restart")
                        .size(20.0)
                        .strong()
                        .italics(),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new("Press Space to reset")
                        .size(20.0)
                        .strong()
                        .italics(),
                ));
            });
        });
}
