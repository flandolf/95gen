#![windows_subsystem = "windows"]

use eframe::egui::{self, Label, ScrollArea};
use rand::Rng;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        follow_system_theme: true,
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
    keys: Vec<String>,
    key_type: KeyType,
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
            ui.heading("Key Generator - by @flandolf");

            ui.horizontal(|ui| {
                ui.label("Amount of keys to generate:");
                ui.add(egui::Slider::new(&mut self.amount, 1..=500));
            });

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.key_type, KeyType::Windows95, "Windows 95");
                ui.radio_value(&mut self.key_type, KeyType::Office95, "Office 95");
            });

            if ui.button("Generate").clicked() {
                self.keys.clear();
                for _ in 0..self.amount {
                    let key = match self.key_type {
                        KeyType::Windows95 => generate_key_95(),
                        KeyType::Office95 => generate_key_office95(),
                    };
                    self.keys.push(key);
                }
            }

            ui.label("Generated Keys:");
            ui.separator();
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for key in &self.keys {
                    ui.add(Label::new(key));
                }
            });
        });
    }
}

fn generate_key_95() -> String {
    let day = rand::thread_rng().gen_range(1..366);
    let year = rand::thread_rng().gen_range(95..99);
    let algodigits = gen_algo_key();
    let randomend = rand::thread_rng().gen_range(10000..99999);
    if day < 10 {
        format!("00{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    } else if day < 100 {
        format!("0{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    } else {
        format!("{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    }
}

fn generate_key_office95() -> String {
    let mut rng = rand::thread_rng();

    // Generate a random 3-digit number excluding specific values
    let first_three_digits: u16 = loop {
        let num = rng.gen_range(100..1000);
        if num != 333 && num != 444 && num != 555 && num != 666 && num != 777 && num != 888 && num != 999 {
            break num;
        }
    };

    // Generate a random 7-digit number divisible by 21
    let mut next_seven_digits: u32 = loop {
        let num = rng.gen_range(1000000..10000000);
        if num % 21 == 0 {
            break num;
        }
    };

    // Format the key
    let key = format!("{:03}-{}", first_three_digits, next_seven_digits);

    key
}

fn gen_algo_key() -> i32 {
    let mut rng = rand::thread_rng();

    loop {
        let random_number = rng.gen_range(100_000..1_000_000);
        let sum_of_digits: i32 = random_number
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .sum();

        if sum_of_digits % 7 == 0 {
            return random_number;
        }
    }
}
