use eframe;
use egui;

use crate::memory::{Memory, Algorithm};

const WIDTH: f32 = 350_f32;
const HEIGHT: f32 = 50_f32;


pub struct MyEguiApp {
    memory: Memory
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>, memory: Memory) -> Self {
        let style = egui::Style {
            visuals: egui::Visuals::dark(),
            ..egui::Style::default()
        };
        cc.egui_ctx.set_style(style);
        
        Self {
            memory
        }
    }

    fn button_pressed(&mut self, algorithm: Algorithm, ctx: &egui::Context) {
        if !self.memory.has_processes_waiting() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        } 
        self.memory.update(algorithm);
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Best fit").clicked() {
                    self.button_pressed(Algorithm::BestFit, ctx);
                }

                if ui.button("Worst fit").clicked() {
                    self.button_pressed(Algorithm::WorstFit, ctx);
                }
            });

            let mut center = egui::pos2(100_f32, 100_f32);
            let dimensions = egui::vec2(WIDTH, HEIGHT);
            let mut rect = egui::Rect::from_min_size(center, dimensions);
            ui.painter().add(egui::Shape::rect_stroke(rect, egui::Rounding::ZERO, egui::Stroke::new(1_f32, egui::Color32::WHITE)));
            ui.put(rect, egui::Label::new(egui::RichText::new("Memory Layout").color(egui::Color32::WHITE)).wrap());
            for partition in &self.memory.get_partitions() {
                center.y += HEIGHT + 1_f32;
                rect = egui::Rect::from_min_size(center, dimensions);
                let color;
                if partition.is_free() {
                    color = egui::Color32::DARK_GREEN;
                } else {
                    color = egui::Color32::DARK_RED;
                }
                ui.painter().add(egui::Shape::rect_filled(rect, egui::Rounding::ZERO, color));
                ui.painter().add(egui::Shape::rect_stroke(rect, egui::Rounding::ZERO, egui::Stroke::new(1_f32, egui::Color32::WHITE)));
                let label = egui::Label::new(egui::RichText::new(format!("{partition}")).color(egui::Color32::WHITE)).wrap();
                ui.put(rect, label);
            }
        });
    }
}