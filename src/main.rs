// #![windows_subsystem = "windows"]

use eframe::egui::{self, Label, RichText, ScrollArea};

mod windowsgen;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Win95Keygen",
        options,
        Box::new(|_cc| Box::<MainApp>::default()),
    )
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
            key_type: KeyType::Windows95, // Default to Windows 95 keys
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Win95/Office95 Key Generator - by @flandolf");
                if ui.button("github").clicked() {
                    open::that("https://github.com/flandolf/95gen").unwrap();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Amount of keys to generate:");
                ui.add(egui::Slider::new(&mut self.amount, 1..=500));
            });

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.key_type, KeyType::Windows95, "Windows 95");
                ui.radio_value(&mut self.key_type, KeyType::Office95, "Office 95");
            });

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    self.keys.clear();
                    for _ in 0..self.amount {
                        let key = match self.key_type {
                            KeyType::Windows95 => windowsgen::generate_key_95(),
                            KeyType::Office95 => windowsgen::generate_key_office95(),
                        };
                        self.keys.push(KeyInfo {
                            key,
                            verified: None, // Initialize as None
                        });
                    }
                }

                if ui.button("Verify All").clicked() {
                    for key_info in &mut self.keys {
                        key_info.verified = match self.key_type {
                            KeyType::Windows95 => Some(windowsgen::verify_win95(&key_info.key)),
                            KeyType::Office95 => Some(windowsgen::verify_office95(&key_info.key)),
                        };
                    }
                }
            });

            ui.label("Generated Keys:");
            ui.separator();
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for key_info in &self.keys {
                    // Display each generated key along with its verification status.
                    ui.horizontal(|ui| {
                        ui.add(Label::new(&key_info.key));
                        match key_info.verified {
                            Some(true) => ui.label(RichText::new("Verified").color(egui::Color32::GREEN)),
                            Some(false) => ui.label(RichText::new("Not verified").color(egui::Color32::RED)),
                            None => ui.label("Not verified"),
                        }
                    });
                }
            });
        });
    }
}
