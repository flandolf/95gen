use catppuccin_egui::MACCHIATO;
use eframe::egui::{self, Label, ScrollArea};
use rand::Rng;

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

struct MainApp {
    amount: i32,
    keys: Vec<String>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            amount: 1,
            keys: Vec::new(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, MACCHIATO);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Windows 95 Keygen - by @flandolf");

            ui.horizontal(|ui| {
                ui.label("Amount of keys to generate:");
                ui.add(egui::Slider::new(&mut self.amount, 1..=1000));
            });

            if ui.button("Generate").clicked() {
                self.keys.clear();
                for _ in 0..self.amount {
                    let key = generate_key();
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

fn generate_key() -> String {
    let day = rand::thread_rng().gen_range(1..366);
    let year = rand::thread_rng().gen_range(95..99);
    let algodigits = gen_algo_key();
    let randomend = rand::thread_rng().gen_range(10000..99999);

    if day < 100 {
        format!("0{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    } else if day < 10 {
        format!("00{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    } else {
        format!("{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    }
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
