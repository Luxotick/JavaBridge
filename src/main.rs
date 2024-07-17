#![windows_subsystem = "windows"]

mod app;
mod config;
mod utils;

use app::MyApp;
use eframe::{egui};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let jar_path = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    let mut app = MyApp::new(jar_path);
    app.load_config();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
