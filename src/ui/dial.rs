use eframe::egui;
use std::f32::consts::PI;
use crate::astrology::{DegreePosition, Harmonic};

pub struct Dial {
    center: egui::Pos2,
    radius: f32,
    current_position: DegreePosition,
    harmonic: Harmonic,
    is_dragging: bool,
    drag_start_pos: Option<egui::Pos2>,
    drag_start_angle: f32,
    shift_pressed: bool,
    ctrl_pressed: bool,
}

impl Default for Dial {
    fn default() -> Self {
        Self::new()
    }
}

impl Dial {
    pub fn new() -> Self {
        Self {
            center: egui::pos2(0.0, 0.0),  // Will be set during rendering
            radius: 200.0,
            current_position: DegreePosition::from_degrees(0.0),
            harmonic: Harmonic::default(),
            is_dragging: false,
            drag_start_pos: None,
            drag_start_angle: 0.0,
            shift_pressed: false,
            ctrl_pressed: false,
        }
    }
    
    /// Gets the increment based on modifier keys
    pub fn get_increment(&self) -> f32 {
        match (self.shift_pressed, self.ctrl_pressed) {
            (true, true) => 1.0 / 3600.0,  // 1 second
            (false, true) => 1.0 / 60.0,   // 1 minute
            (true, false) => 1.0,          // 1 degree
            (false, false) => 1.0 / 60.0,  // Default to 1 minute
        }
    }
    
    /// Snaps angle to the current increment
    pub fn snap_angle(&self, angle: f32) -> f32 {
        let increment = self.get_increment();
        (angle / increment).round() * increment
    }    
    // Getters and setters
    pub fn position(&self) -> DegreePosition {
        self.current_position
    }
    
    pub fn set_position(&mut self, position: DegreePosition) {
        self.current_position = position;
    }
    
    pub fn harmonic(&self) -> Harmonic {
        self.harmonic
    }
    
    pub fn set_harmonic(&mut self, harmonic: Harmonic) {
        self.harmonic = harmonic;
    }
    
    pub fn update_modifiers(&mut self, modifiers: egui::Modifiers) {
        self.shift_pressed = modifiers.shift;
        self.ctrl_pressed = modifiers.ctrl;
    }
    
    /// Calculate angle from center to a point (in degrees, clockwise from top)
    fn calculate_angle(&self, pos: egui::Pos2) -> f32 {
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        
        // Calculate angle in radians, then convert to degrees
        // atan2 gives counterclockwise from right, we need clockwise from top
        let angle_rad = dy.atan2(dx);
        
        // Convert to degrees and adjust to start from top (90 deg offset)
        // and go clockwise (negative)
        let angle_deg = (angle_rad * 180.0 / PI) + 90.0;
        
        // Normalize to [0, 360)
        (360.0 + angle_deg) % 360.0
    }
    
    /// Handle keyboard input
    pub fn handle_key(&mut self, key: egui::Key) {
        let increment = self.get_increment();
        
        match key {
            egui::Key::ArrowLeft => {
                // Counterclockwise movement (increasing angle)
                let new_degrees = self.current_position.to_degrees() + increment;
                self.current_position = DegreePosition::from_degrees(new_degrees);
            }
            egui::Key::ArrowRight => {
                // Clockwise movement (decreasing angle)
                let new_degrees = self.current_position.to_degrees() - increment;
                self.current_position = DegreePosition::from_degrees(new_degrees);
            }
            _ => {}
        }
    }    
    /// Handle mouse drag - improved to keep position when clicking and fix direction
    pub fn handle_pointer_interaction(&mut self, pointer_pos: Option<egui::Pos2>, drag_started: bool, drag_released: bool) {
        if drag_started {
            // When drag starts, store the starting position and current angle
            self.is_dragging = true;
            self.drag_start_pos = pointer_pos;
            self.drag_start_angle = self.current_position.to_degrees();
        }
        
        if drag_released {
            self.is_dragging = false;
            self.drag_start_pos = None;
        }
        
        if self.is_dragging {
            // Only move the dial when actually dragging, not on initial click
            if let (Some(start_pos), Some(current_pos)) = (self.drag_start_pos, pointer_pos) {
                // Only move if the mouse has actually moved from the start position
                if start_pos != current_pos {
                    // Get the start angle and current angle from the mouse position
                    let start_angle = self.calculate_angle(start_pos);
                    let current_angle = self.calculate_angle(current_pos);
                    
                    // Calculate the angle difference (how much the mouse has moved)
                    // FIXED: Reversed the order to match the expected direction
                    let mut angle_diff = start_angle - current_angle;
                    
                    // Handle wrap-around cases
                    if angle_diff > 180.0 {
                        angle_diff -= 360.0;
                    } else if angle_diff < -180.0 {
                        angle_diff += 360.0;
                    }
                    
                    // Calculate the new position based on the start position and the angle difference
                    // FIXED: Changed to addition to match expected direction
                    let new_angle = self.drag_start_angle + angle_diff;
                    
                    // Snap to increment
                    let snapped_angle = self.snap_angle(new_angle);
                    self.current_position = DegreePosition::from_degrees(snapped_angle);
                    
                    // Update the drag start position and angle for the next frame
                    self.drag_start_pos = Some(current_pos);
                    self.drag_start_angle = snapped_angle;
                }
            }
        }
    }
    
    /// Render the dial
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        // Create a custom widget area for the dial
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(800.0, 450.0),
            egui::Sense::click_and_drag(),
        );
        
        self.center = response.rect.center();
        
        // Handle mouse interaction
        self.handle_pointer_interaction(
            response.interact_pointer_pos(),
            response.drag_started(),
            response.drag_released(),
        );
        
        // Draw the dial
        self.draw(&painter);
        
        response
    }    
    /// Draw the dial
    fn draw(&self, painter: &egui::Painter) {
        // Draw outer circle
        painter.circle_stroke(
            self.center,
            self.radius,
            egui::Stroke::new(2.0, egui::Color32::from_gray(100)),
        );
        
        // Draw reversed outer numbering (clockwise)
        self.draw_outer_degree_markings(painter);
        
        // Draw inner tick marks
        self.draw_inner_tick_marks(painter);
        
        // Draw the 4 main arms
        self.draw_main_arms(painter);
        
        // Draw center dot
        painter.circle_filled(self.center, 4.0, egui::Color32::from_gray(60));
    }
    
    /// Draw reversed outer degree markings (clockwise)
    fn draw_outer_degree_markings(&self, painter: &egui::Painter) {
        // Draw tick marks and numbers (clockwise)
        for i in 0..360 {
            // Calculate angle for clockwise numbering
            let display_degree = i as f32;
            let angle = (90.0 + display_degree) * PI / 180.0; // 90° offset for 0° at top, + for clockwise
            
            // Outer tick radius
            let outer_radius = self.radius + 5.0;
            
            if i % 30 == 0 {
                // Major tick
                let inner_radius = self.radius + 15.0;
                let start = egui::pos2(
                    self.center.x + outer_radius * angle.cos(),
                    self.center.y - outer_radius * angle.sin(),
                );
                let end = egui::pos2(
                    self.center.x + inner_radius * angle.cos(),
                    self.center.y - inner_radius * angle.sin(),
                );
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(2.0, egui::Color32::from_gray(80)),
                );
                
                // Draw degree label
                let label_radius = self.radius + 30.0;
                let label_pos = egui::pos2(
                    self.center.x + label_radius * angle.cos(),
                    self.center.y - label_radius * angle.sin(),
                );
                painter.text(
                    label_pos,
                    egui::Align2::CENTER_CENTER,
                    format!("{}", i),
                    egui::FontId::default(),
                    egui::Color32::from_gray(60),
                );            } else if i % 5 == 0 {
                // Medium tick every 5 degrees
                let inner_radius = self.radius + 10.0;
                let start = egui::pos2(
                    self.center.x + outer_radius * angle.cos(),
                    self.center.y - outer_radius * angle.sin(),
                );
                let end = egui::pos2(
                    self.center.x + inner_radius * angle.cos(),
                    self.center.y - inner_radius * angle.sin(),
                );
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(1.0, egui::Color32::from_gray(120)),
                );
            } else {
                // Minor tick for every degree
                let inner_radius = self.radius + 7.0;
                let start = egui::pos2(
                    self.center.x + outer_radius * angle.cos(),
                    self.center.y - outer_radius * angle.sin(),
                );
                let end = egui::pos2(
                    self.center.x + inner_radius * angle.cos(),
                    self.center.y - inner_radius * angle.sin(),
                );
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(0.5, egui::Color32::from_gray(150)),
                );
            }
        }
    }
    
    /// Draw inner tick marks
    fn draw_inner_tick_marks(&self, painter: &egui::Painter) {
        // Get current rotation angle - FIXED: Now uses negative to match clockwise direction
        let current_degree = -self.current_position.to_degrees();
        
        // Draw tick marks inside the dial circle
        for i in 0..360 {
            // Calculate angle for the tick, considering the current position
            let angle_offset = (current_degree + i as f32) % 360.0;
            let angle = (90.0 - angle_offset) * PI / 180.0; // 90° offset for 0° at top
            
            let tick_length = 10.0;
            let inner_radius = self.radius - tick_length;
            
            // Major tick every 30 degrees
            if i % 30 == 0 {
                let start = egui::pos2(
                    self.center.x + self.radius * angle.cos(),
                    self.center.y - self.radius * angle.sin(),
                );
                let end = egui::pos2(
                    self.center.x + inner_radius * angle.cos(),
                    self.center.y - inner_radius * angle.sin(),
                );                
                // Determine color - make 0 degrees red, others gray
                let color = if i == 0 {
                    egui::Color32::RED
                } else {
                    egui::Color32::from_gray(120)
                };
                
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(2.0, color),
                );
            } else if i % 10 == 0 {
                // Medium tick every 10 degrees
                let medium_inner_radius = self.radius - (tick_length * 0.7);
                let start = egui::pos2(
                    self.center.x + self.radius * angle.cos(),
                    self.center.y - self.radius * angle.sin(),
                );
                let end = egui::pos2(
                    self.center.x + medium_inner_radius * angle.cos(),
                    self.center.y - medium_inner_radius * angle.sin(),
                );
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(1.0, egui::Color32::from_gray(140)),
                );
            } else if i % 5 == 0 {
                // Minor tick every 5 degrees
                let minor_inner_radius = self.radius - (tick_length * 0.5);
                let start = egui::pos2(
                    self.center.x + self.radius * angle.cos(),
                    self.center.y - self.radius * angle.sin(),
                );
                let end = egui::pos2(
                    self.center.x + minor_inner_radius * angle.cos(),
                    self.center.y - minor_inner_radius * angle.sin(),
                );
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(0.5, egui::Color32::from_gray(160)),
                );
            }
        }
    }    
    /// Draw the 4 main arms
    fn draw_main_arms(&self, painter: &egui::Painter) {
        // Get current rotation angle - FIXED: Now uses negative to match clockwise direction
        let current_degree = -self.current_position.to_degrees();
        
        // Draw the 4 main cardinal arms (at 0, 90, 180, 270 degrees)
        for i in 0..4 {
            // Calculate angle for the arm, considering the current position
            let arm_angle = (90.0 * i as f32 + current_degree) % 360.0;
            let angle_rad = (90.0 - arm_angle) * PI / 180.0;
            
            let inner_radius = 10.0;
            let start = egui::pos2(
                self.center.x + inner_radius * angle_rad.cos(),
                self.center.y - inner_radius * angle_rad.sin(),
            );
            let end = egui::pos2(
                self.center.x + (self.radius - 15.0) * angle_rad.cos(),
                self.center.y - (self.radius - 15.0) * angle_rad.sin(),
            );
            
            // First arm (0 degrees) is red, others are gray
            let color = if i == 0 {
                egui::Color32::RED
            } else {
                egui::Color32::from_gray(100)
            };
            
            painter.line_segment(
                [start, end],
                egui::Stroke::new(2.0, color),
            );
        }
        
        // Draw additional arms based on harmonic (thinner)
        let arm_count = match self.harmonic {
            Harmonic::First => 4,
            Harmonic::Second => 8,
            Harmonic::Fourth => 16,
            Harmonic::Eighth => 32,
            Harmonic::Sixteenth => 64,
            _ => 4,
        };
        
        if arm_count > 4 {
            let base_angle_step = 360.0 / arm_count as f32;
            
            for i in 0..arm_count {
                // Skip the main arms (0, 90, 180, 270)
                if i % (arm_count / 4) == 0 {
                    continue;
                }                
                // Calculate angle for the arm, considering the current position
                let arm_angle = (base_angle_step * i as f32 + current_degree) % 360.0;
                let angle_rad = (90.0 - arm_angle) * PI / 180.0;
                
                let inner_radius = 30.0;
                let start = egui::pos2(
                    self.center.x + inner_radius * angle_rad.cos(),
                    self.center.y - inner_radius * angle_rad.sin(),
                );
                let end = egui::pos2(
                    self.center.x + (self.radius - 20.0) * angle_rad.cos(),
                    self.center.y - (self.radius - 20.0) * angle_rad.sin(),
                );
                
                painter.line_segment(
                    [start, end],
                    egui::Stroke::new(0.5, egui::Color32::from_gray(140)),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_increment_calculation() {
        let mut dial = Dial::new();
        
        // Test shift only (1 degree)
        dial.shift_pressed = true;
        dial.ctrl_pressed = false;
        assert_eq!(dial.get_increment(), 1.0);
        
        // Test ctrl only (1 minute)
        dial.shift_pressed = false;
        dial.ctrl_pressed = true;
        assert_eq!(dial.get_increment(), 1.0 / 60.0);
        
        // Test shift + ctrl (1 second)
        dial.shift_pressed = true;
        dial.ctrl_pressed = true;
        assert_eq!(dial.get_increment(), 1.0 / 3600.0);
    }
    
    #[test]
    fn test_calculate_angle() {
        let mut dial = Dial::new();
        dial.center = egui::pos2(100.0, 100.0);
        
        // Test cardinal points
        assert!(
            (dial.calculate_angle(egui::pos2(100.0, 0.0)) - 0.0).abs() < 0.001, 
            "Top should be 0 degrees"
        );
        assert!(
            (dial.calculate_angle(egui::pos2(200.0, 100.0)) - 90.0).abs() < 0.001, 
            "Right should be 90 degrees"
        );
        assert!(
            (dial.calculate_angle(egui::pos2(100.0, 200.0)) - 180.0).abs() < 0.001, 
            "Bottom should be 180 degrees"
        );
        assert!(
            (dial.calculate_angle(egui::pos2(0.0, 100.0)) - 270.0).abs() < 0.001, 
            "Left should be 270 degrees"
        );
    }
}