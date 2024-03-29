#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{self, Label, RichText, ScrollArea},
    Theme,
};
use egui::IconData;
use image::GenericImageView;

mod icon;
mod windowsgen;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 480.0])
            .with_icon(load_icon()),
        default_theme: Theme::Dark,
        follow_system_theme: false,
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Win95Keygen",
        options,
        Box::new(|_cc| Box::<MainApp>::default()),
    )
}

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(icon::ICON).unwrap();
        let (width, height) = image.dimensions();
        let rgba = image.into_rgba8();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba.to_vec(),
        width: icon_width,
        height: icon_height,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum KeyType {
    Windows95,
    Office95,
}

struct MainApp {
    amount: i32,
    keys: Vec<KeyInfo>,
    key_type: KeyType,
}

struct KeyInfo {
    key: String,
    verified: Option<bool>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            amount: 1,
            keys: Vec::new(),
            key_type: KeyType::Windows95,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.33);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("win95/office95 keygen - by @flandolf");
                if ui.button("github").clicked() {
                    open::that("https://github.com/flandolf/95gen").unwrap();
                }
                if ui.button("toggle theme").clicked() {
                    if ctx.style().visuals.dark_mode {
                        ctx.set_visuals(egui::Visuals::light());
                    } else {
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("amount of keys to generate:");
                ui.add(egui::Slider::new(&mut self.amount, 1..=500));
            });

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.key_type, KeyType::Windows95, "windows 95");
                ui.radio_value(&mut self.key_type, KeyType::Office95, "office 95");
            });

            ui.horizontal(|ui| {
                if ui.button("generate").clicked() {
                    self.keys.clear();
                    for _ in 0..self.amount {
                        let key = match self.key_type {
                            KeyType::Windows95 => windowsgen::generate_key_95(),
                            KeyType::Office95 => windowsgen::generate_key_office95(),
                        };
                        self.keys.push(KeyInfo {
                            key,
                            verified: None,
                        });
                    }
                }

                if ui.button("verify all").clicked() {
                    for key_info in &mut self.keys {
                        key_info.verified = match self.key_type {
                            KeyType::Windows95 => Some(windowsgen::verify_win95(&key_info.key)),
                            KeyType::Office95 => Some(windowsgen::verify_office95(&key_info.key)),
                        };
                    }
                }

                if ui.button("save").clicked() {

                    if self.keys.is_empty() {
                        return;
                    }

                    let mut file_content = String::new();
                    for key_info in &self.keys {
                        file_content.push_str(&key_info.key);
                        file_content.push('\n');
                    }

                    let files = rfd::FileDialog::new()
                        .add_filter("text", &["txt"])
                        .set_directory("/")
                        .save_file();

                    if let Some(file) = files {
                        std::fs::write(file, &file_content).unwrap();
                    }
                }

                if ui.button("load").clicked() {
                    let files = rfd::FileDialog::new()
                        .add_filter("text", &["txt"])
                        .set_directory("/")
                        .pick_file();

                    if let Some(file) = files {
                        let file_content = std::fs::read_to_string(file).unwrap();
                        self.keys.clear();
                        for line in file_content.lines() {
                            self.keys.push(KeyInfo {
                                key: line.to_string(),
                                verified: None,
                            });
                        }
                    }
                }
                if ui.button("clear").clicked() {
                    self.keys.clear();
                }
            });

            ui.label("generated keys:");
            ui.separator();
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for key_info in &self.keys {
                    ui.horizontal(|ui| {
                        ui.add(Label::new(&key_info.key));
                        match key_info.verified {
                            Some(true) => {
                                ui.label(RichText::new("valid").color(egui::Color32::GREEN))
                            }
                            Some(false) => {
                                ui.label(RichText::new("invalid").color(egui::Color32::RED))
                            }
                            None => ui.label("not verified"),
                        }
                    });
                }
            });
        });
    }
}
