use std::time::Duration;

use crate::controller_management::{
    BackgroundMultiDevice, ControllerInput, DeviceState, MultiDevice,
};
use eframe::egui;
use egui::{Frame, Margin};
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    prelude::{Circle, IntoDrawingArea},
    series::LineSeries,
    style::{BLUE, RED, WHITE},
};

use std::path::Path;

pub struct GameApp {
    device_manager: Option<BackgroundMultiDevice>,
    winner: Option<String>,
    controller_one_pos: Vec<(f64, (i32, i32))>,
    controller_two_pos: Vec<(f64, (i32, i32))>,
}

impl Default for GameApp {
    fn default() -> Self {
        Self {
            device_manager: None,
            winner: None,
            controller_one_pos: vec![(0.0, (0, 0))],
            controller_two_pos: vec![(0.0, (0, 0))],
        }
    }
}

impl GameApp {
    pub fn get_state(&self) -> DeviceState {
        self.device_manager.as_ref().unwrap().get_state()
    }
    pub fn get_pos_player_one(&self) -> (i32, i32) {
        self.device_manager.as_ref().unwrap().get_pos()[0]
    }
    pub fn get_pos_player_two(&self) -> (i32, i32) {
        self.device_manager.as_ref().unwrap().get_pos()[1]
    }
    pub fn set_controller_input(&mut self, controller_input: Option<ControllerInput>) {
        self.device_manager
            .as_mut()
            .unwrap()
            .set_controller_input(controller_input);
    }

    fn plot_data(
        &self,
        filename: &str,
        title: &str,
        label1: &str,
        label2: &str,
        data1: Vec<(f64, f64)>,
        data2: Vec<(f64, f64)>,
    ) {
        let width = 800;
        let height = 600;

        let folder = Path::new("plots");
        std::fs::create_dir_all(folder).expect("Failed to create folder");
        let file_path = folder.join(filename);

        let drawing_area = BitMapBackend::new(&file_path, (width, height)).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        let root = drawing_area;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("Arial", 20))
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                0f64..self.controller_one_pos.last().unwrap().0, // Time range
                -self.get_max_displacement()..self.get_max_displacement(), // Displacement range
            )
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(data1, &RED))
            .unwrap()
            .label(label1)
            .legend(|(x, y)| Circle::new((x, y), 3, &RED));

        chart
            .draw_series(LineSeries::new(data2, &BLUE))
            .unwrap()
            .label(label2)
            .legend(|(x, y)| Circle::new((x, y), 3, &BLUE));

        chart.configure_series_labels().draw().unwrap();
        root.present().unwrap();
    }

    // Function to compute displacement from positions (time, (x, y))
    fn compute_displacement(&self, positions: &Vec<(f64, (i32, i32))>) -> Vec<(f64, f64)> {
        positions
            .iter()
            .map(|(t, (x, y))| (*t, ((x.pow(2) + y.pow(2)) as f64).sqrt()))
            .collect()
    }

    // Plot positions for controllers
    fn plot_positions(&self) {
        let positions_one = self.compute_displacement(&self.controller_one_pos);
        let positions_two = self.compute_displacement(&self.controller_two_pos);
        self.plot_data(
            "positions.png",
            "Position vs Time",
            "Controller 1",
            "Controller 2",
            positions_one,
            positions_two,
        );
    }

    // Plot displacement differences between controllers
    fn plot_difference(&self) {
        let positions_one = self.compute_displacement(&self.controller_one_pos);
        let positions_two = self.compute_displacement(&self.controller_two_pos);

        let difference_one: Vec<(f64, f64)> = positions_one
            .iter()
            .zip(positions_two.iter())
            .map(|((t1, d1), (_, d2))| (*t1, d1 - d2))
            .collect();

        let difference_two: Vec<(f64, f64)> = positions_two
            .iter()
            .zip(positions_one.iter())
            .map(|((t1, d1), (_, d2))| (*t1, d1 - d2))
            .collect();

        self.plot_data(
            "difference.png",
            "Displacement Difference vs Time",
            "Controller 1",
            "Controller 2",
            difference_one,
            difference_two,
        );
    }

    // Helper function to get the maximum displacement across both controllers
    fn get_max_displacement(&self) -> f64 {
        self.controller_one_pos
            .iter()
            .chain(self.controller_two_pos.iter())
            .map(|(_, (x, y))| ((x.pow(2) + y.pow(2)) as f64).sqrt())
            .fold(0.0, |a, b| a.max(b))
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.device_manager {
                // device manager not yet initialized
                None => match BackgroundMultiDevice::from_auto_configure(2) {
                    Ok(device_manager) => {
                        self.device_manager = Some(device_manager);
                    }
                    Err(e) => {
                        display_welcome(ui, &e.to_string());
                    }
                },
                // device manager initialized
                Some(device_manager) => match device_manager.get_state() {
                    DeviceState::PendingInit => {
                        display_welcome(ui, &"Initialized Controllers");
                        self.device_manager
                            .as_mut()
                            .unwrap()
                            .set_controller_input(None);
                    }
                    DeviceState::PendingStart => {
                        display_starting(ui);
                        if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
                            self.set_controller_input(Some(ControllerInput::StartController));
                        } else {
                            self.set_controller_input(None);
                        }
                    }
                    DeviceState::Running => {
                        display_running(ui, self.get_pos_player_one(), self.get_pos_player_two());
                        self.controller_one_pos
                            .push((ctx.input(|input| input.time), self.get_pos_player_one()));
                        self.controller_two_pos
                            .push((ctx.input(|input| input.time), self.get_pos_player_two()));

                        if ctx.input(|input| input.key_pressed(egui::Key::Space)) {
                            self.set_controller_input(Some(ControllerInput::StopGame));
                            self.plot_positions();
                            self.plot_difference();
                        } else {
                            self.set_controller_input(None);
                        }
                    }
                    DeviceState::Complete => {
                        self.winner =
                            calculate_winner(self.get_pos_player_one(), self.get_pos_player_two());
                        display_complete(ui, &self.winner);
                        if ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
                            self.set_controller_input(Some(ControllerInput::RestartGame));
                        } else if ctx.input(|input| input.key_pressed(egui::Key::Space)) {
                            self.set_controller_input(Some(ControllerInput::ResetGame));
                        }
                    }
                },
            }
        });
        ctx.request_repaint_after(Duration::from_millis(10)); // Request repaint every frame

        /*

            match self.get_state() {
            DeviceState::PendingInit => {
                display_welcome(ui);
                match MultiDevice::from_auto_configure(2) {
                    Ok(device_manager) => self.device_manager = Some(device_manager),
                    Err(e) => {
                        eprintln!("Error initializing devices: {}", e);
                    }
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
         */
    }
}

fn display_welcome(ui: &mut egui::Ui, intialization_text: &str) {
    Frame::default()
        .inner_margin(Margin::same(225.0))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("Welcome").size(60.0).strong(),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new(intialization_text).size(20.0).italics(),
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

                let winner = match winner {
                    Some(winner) => format!("The winner is {winner}").to_string(),
                    None => "It is a Draw".to_string(),
                };

                ui.add(egui::Label::new(
                    egui::RichText::new(winner).size(20.0).italics(),
                ));
            });
            ui.add_space(50.0);
            ui.add(egui::Label::new(egui::RichText::new(
                "Check out plots/positions.png and plots/difference.png for data about your round",
            ).size(20.0)));
            ui.add_space(100.0);
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

fn calculate_winner(
    controller_one_pos: (i32, i32),
    controller_two_pos: (i32, i32),
) -> Option<String> {
    let pos_one = ((controller_one_pos.0.pow(2) + controller_one_pos.1.pow(2)) as f32).sqrt();
    let pos_two = ((controller_two_pos.0.pow(2) + controller_two_pos.1.pow(2)) as f32).sqrt();
    if pos_one > pos_two {
        Some("Controller 0".to_string())
    } else if pos_one < pos_two {
        Some("Controller 1".to_string())
    } else {
        None
    }
}
