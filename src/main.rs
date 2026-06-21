use eframe::egui;
use ui_macro::generate_ui;
use common::Command; // Changed: Import from common

#[generate_ui]
fn add(a: i32, b: i32) -> i32 { a + b }

#[generate_ui]
fn mult(a: i32, b: i32, c: i32) -> i32 { a * b * c }

struct MyApp {
    registry: Vec<Box<dyn Command>>,
    active_command: Option<Box<dyn Command>>,
    results: Vec<i32>,
    search: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            registry: vec![
                Box::new(AddDialog::default()), // Now matches PascalCase
                Box::new(MultDialog::default()),
            ],
            active_command: None,
            results: Vec::new(),
            search: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ... (rest of your UI logic remains the same)
        egui::SidePanel::left("palette").show(ctx, |ui| {
            ui.heading("Command Palette");
            ui.text_edit_singleline(&mut self.search);
            ui.separator();
            for cmd in &self.registry {
                if cmd.name().contains(&self.search) {
                    if ui.button(cmd.name()).clicked() {
                        self.active_command = Some(dyn_clone::clone_box(&**cmd));
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(cmd) = &mut self.active_command {
                if let Some(res) = cmd.show(ui) {
                    self.results.push(res);
                }
            }
            ui.label(format!("History: {:?}", self.results));
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native("Command Palette", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}