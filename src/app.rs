use eframe::{egui, epi};
use std::thread;
use crate::config::Config;
use crate::utils::{get_java_versions, run_jar_with_java};

pub struct MyApp {
    java_versions: Vec<String>,
    selected_version: Option<String>,
    jar_path: String,
    debug: bool,
    refresh: bool,
    show_settings: bool,
}

impl MyApp {
    pub fn new(jar_path: String) -> Self {
        Self {
            java_versions: Vec::new(),
            selected_version: None,
            jar_path,
            debug: false,
            refresh: true,
            show_settings: false,
        }
    }

    pub fn load_java_versions(&mut self) {
        if self.refresh {
            self.java_versions = get_java_versions();
            self.refresh = false;
        }
    }

    pub fn load_config(&mut self) {
        if let Ok(config) = Config::load() {
            self.selected_version = config.selected_version;
        }
    }

    pub fn save_config(&self) {
        let config = Config {
            selected_version: self.selected_version.clone(),
        };
        if let Err(e) = config.save() {
            eprintln!("Failed to save config: {}", e);
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Rust Java Launcher"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.load_java_versions();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Rust Java Launcher");
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("⚙").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_settings {
                egui::Window::new("Settings")
                    .default_height(100.0)
                    .show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Default java version: ");
                                ui.vertical(|ui| {
                                    ui.radio_value(&mut self.selected_version, None, "None");
                                    for version in &self.java_versions {
                                        ui.radio_value(&mut self.selected_version, Some(version.clone()), version);
                                    }
                                });
                            });
                            if ui.button("Save").clicked() {
                                self.save_config();
                                self.show_settings = false;
                            }
                        });
                    });
            } else {
                if ui.button("Load Java Versions").clicked() {
                    self.refresh = true;
                    self.load_java_versions();
                }

                egui::ScrollArea::vertical()
                    .max_height(100.0)
                    .show(ui, |ui| {
                        for version in &self.java_versions {
                            ui.radio_value(&mut self.selected_version, Some(version.clone()), version);
                        }
                    });

                ui.horizontal(|ui| {
                    ui.label("JAR Path: ");
                    ui.text_edit_singleline(&mut self.jar_path);
                });

                ui.horizontal(|ui| {
                    if ui.button("Run JAR").clicked() {
                        if let Some(version) = &self.selected_version {
                            let jar_path = self.jar_path.clone();
                            let version = version.clone();
                            let debug = self.debug;
                            thread::spawn(move || {
                                run_jar_with_java(&jar_path, &version, debug);
                            });
                        }
                    }
                    ui.add_space(10.0);
                    ui.checkbox(&mut self.debug, "Debug");
                });
            }

        });
    }
}