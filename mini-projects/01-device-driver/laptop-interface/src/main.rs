mod commander;
mod controller_management;
mod gui;
mod libserial;
mod threads;

use eframe::NativeOptions;
use gui::GameApp;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Mini Project 1",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(GameApp::default()))),
    );

    Ok(())
}
