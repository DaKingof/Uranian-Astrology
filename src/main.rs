use eframe::egui;
use std::f32::consts::PI;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Uranian Astrology Dial"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Uranian Astrology",
        options,
        Box::new(|_cc| Box::<DialApp>::default()),
    )
}

struct DialApp {
    current_degree: f32,
    is_dragging: bool,
    dial_center: egui::Pos2,
    dial_radius: f32,
}

impl Default for DialApp {
    fn default() -> Self {
        Self {
            current_degree: 0.0,
            is_dragging: false,
            dial_center: egui::pos2(400.0, 300.0),
            dial_radius: 200.0,
        }
    }
}

impl eframe::App for DialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Uranian Astrology Dial");
            
            // Create a custom widget area for the dial
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(800.0, 500.0),
                egui::Sense::click_and_drag(),
            );
            
            self.dial_center = response.rect.center();
            
            // Handle mouse interaction
            if response.drag_started() {
                self.is_dragging = true;
            }
            if response.drag_released() {
                self.is_dragging = false;
            }
            
            if self.is_dragging {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let dx = pointer_pos.x - self.dial_center.x;
                    let dy = pointer_pos.y - self.dial_center.y;
                    self.current_degree = dy.atan2(dx).to_degrees() + 90.0;
                    if self.current_degree < 0.0 {
                        self.current_degree += 360.0;
                    }
                }
            }
            
            // Draw the dial
            draw_dial(&painter, self.dial_center, self.dial_radius, self.current_degree);
            
            // Display current degree
            ui.separator();
            ui.label(format!("Current Degree: {:.1}°", self.current_degree));
        });
    }
}
fn draw_dial(painter: &egui::Painter, center: egui::Pos2, radius: f32, current_degree: f32) {
    // Draw outer circle
    painter.circle_stroke(
        center,
        radius,
        egui::Stroke::new(2.0, egui::Color32::from_gray(100)),
    );
    
    // Draw tick marks
    for i in 0..360 {
        if i % 30 == 0 {
            // Major tick
            let angle = (i as f32 - 90.0) * PI / 180.0;
            let inner_radius = radius - 20.0;
            let start = egui::pos2(
                center.x + inner_radius * angle.cos(),
                center.y + inner_radius * angle.sin(),
            );
            let end = egui::pos2(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            painter.line_segment(
                [start, end],
                egui::Stroke::new(2.0, egui::Color32::from_gray(80)),
            );
            
            // Draw degree label
            let label_radius = radius - 35.0;
            let label_pos = egui::pos2(
                center.x + label_radius * angle.cos(),
                center.y + label_radius * angle.sin(),
            );
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                format!("{}°", i),
                egui::FontId::default(),
                egui::Color32::from_gray(60),
            );
        } else if i % 5 == 0 {
            // Minor tick
            let angle = (i as f32 - 90.0) * PI / 180.0;
            let inner_radius = radius - 10.0;
            let start = egui::pos2(
                center.x + inner_radius * angle.cos(),
                center.y + inner_radius * angle.sin(),
            );
            let end = egui::pos2(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            painter.line_segment(
                [start, end],
                egui::Stroke::new(1.0, egui::Color32::from_gray(120)),
            );
        }
    }
    
    // Draw center dot
    painter.circle_filled(center, 4.0, egui::Color32::from_gray(60));
    
    // Draw pointer
    let pointer_angle = (current_degree - 90.0) * PI / 180.0;
    let pointer_end = egui::pos2(
        center.x + (radius - 30.0) * pointer_angle.cos(),
        center.y + (radius - 30.0) * pointer_angle.sin(),
    );
    painter.line_segment(
        [center, pointer_end],
        egui::Stroke::new(3.0, egui::Color32::RED),
    );
    
    // Draw pointer tip
    painter.circle_filled(pointer_end, 6.0, egui::Color32::RED);
}

// Test module for dial functionality
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_initialization() {
        let app = DialApp::default();
        assert_eq!(app.current_degree, 0.0);
        assert_eq!(app.is_dragging, false);
        assert_eq!(app.dial_radius, 200.0);
    }

    #[test]
    fn test_degree_normalization() {
        // Test that degrees wrap around correctly
        let mut degree = -45.0;
        if degree < 0.0 {
            degree += 360.0;
        }
        assert_eq!(degree, 315.0);
        
        let mut degree = 450.0;
        degree = degree % 360.0;
        assert_eq!(degree, 90.0);
    }
}