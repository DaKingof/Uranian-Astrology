use eframe::egui;
use crate::ui::dial::Dial;
use crate::astrology::harmonics::Harmonic;

pub struct DialApp {
    dial: Dial,
}

impl Default for DialApp {
    fn default() -> Self {
        Self {
            dial: Dial::new(),
        }
    }
}

impl eframe::App for DialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        ctx.input(|i| {
            self.dial.update_modifiers(i.modifiers);
            
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.dial.handle_key(egui::Key::ArrowLeft);
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.dial.handle_key(egui::Key::ArrowRight);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Uranian Astrology Dial");
            
            // Harmonic selector
            ui.horizontal(|ui| {
                ui.label("Harmonic:");
                
                if ui.selectable_value(&mut self.dial.harmonic(), Harmonic::First, "1st (360°)").clicked() {
                    self.dial.set_harmonic(Harmonic::First);
                }
                if ui.selectable_value(&mut self.dial.harmonic(), Harmonic::Second, "2nd (180°)").clicked() {
                    self.dial.set_harmonic(Harmonic::Second);
                }
                if ui.selectable_value(&mut self.dial.harmonic(), Harmonic::Fourth, "4th (90°)").clicked() {
                    self.dial.set_harmonic(Harmonic::Fourth);
                }
                if ui.selectable_value(&mut self.dial.harmonic(), Harmonic::Eighth, "8th (45°)").clicked() {
                    self.dial.set_harmonic(Harmonic::Eighth);
                }
                if ui.selectable_value(&mut self.dial.harmonic(), Harmonic::Sixteenth, "16th (22.5°)").clicked() {
                    self.dial.set_harmonic(Harmonic::Sixteenth);
                }
            });
            
            // Render the dial
            self.dial.ui(ui);
            
            // Display current position and controls
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Position:");
                ui.monospace(self.dial.position().format());
                ui.separator();
                ui.label("Controls: Shift = 1° | Ctrl = 1' | Shift+Ctrl = 1\"");
            });
        });
    }
}