use egui::{self, Ui};
use crate::astrology::harmonics::Harmonic;

pub struct DialControls {
    harmonic: Harmonic,
    shift_pressed: bool,
    ctrl_pressed: bool,
}

impl Default for DialControls {
    fn default() -> Self {
        Self {
            harmonic: Harmonic::default(),
            shift_pressed: false,
            ctrl_pressed: false,
        }
    }
}

impl DialControls {
    pub fn new() -> Self {
        Self::default()
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
    
    pub fn get_increment(&self) -> f32 {
        match (self.shift_pressed, self.ctrl_pressed) {
            (true, true) => 1.0 / 3600.0,  // 1 second
            (false, true) => 1.0 / 60.0,   // 1 minute
            (true, false) => 1.0,          // 1 degree
            (false, false) => 1.0 / 60.0,  // Default to 1 minute
        }
    }
    
    pub fn ui(&mut self, ui: &mut Ui) -> Option<Harmonic> {
        let mut changed = false;
        let mut new_harmonic = self.harmonic;
        
        ui.horizontal(|ui| {
            ui.label("Harmonic:");
            
            if ui.selectable_value(&mut new_harmonic, Harmonic::First, "1st (360°)").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut new_harmonic, Harmonic::Second, "2nd (180°)").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut new_harmonic, Harmonic::Fourth, "4th (90°)").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut new_harmonic, Harmonic::Eighth, "8th (45°)").clicked() {
                changed = true;
            }
            if ui.selectable_value(&mut new_harmonic, Harmonic::Sixteenth, "16th (22.5°)").clicked() {
                changed = true;
            }
        });
        
        if changed {
            self.harmonic = new_harmonic;
            Some(new_harmonic)
        } else {
            None
        }
    }    
    /// Display the controls help text
    pub fn display_help(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Controls:");
            ui.monospace("Shift = 1° | Ctrl = 1' | Shift+Ctrl = 1\"");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_initialization() {
        let controls = DialControls::default();
        assert_eq!(controls.harmonic(), Harmonic::First);
        assert_eq!(controls.shift_pressed, false);
        assert_eq!(controls.ctrl_pressed, false);
    }
    
    #[test]
    fn test_increment_calculation() {
        let mut controls = DialControls::default();
        
        // Test shift only (1 degree)
        controls.shift_pressed = true;
        controls.ctrl_pressed = false;
        assert_eq!(controls.get_increment(), 1.0);
        
        // Test ctrl only (1 minute)
        controls.shift_pressed = false;
        controls.ctrl_pressed = true;
        assert_eq!(controls.get_increment(), 1.0 / 60.0);
        
        // Test shift + ctrl (1 second)
        controls.shift_pressed = true;
        controls.ctrl_pressed = true;
        assert_eq!(controls.get_increment(), 1.0 / 3600.0);
    }
}