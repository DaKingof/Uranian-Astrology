use std::f32::consts::PI;
use crate::utils::math::{normalize_degrees, degrees_to_radians, radians_to_degrees};

/// Represents an angular position in degrees, minutes, and seconds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegreePosition {
    degrees: u16,
    minutes: u8,
    seconds: u8,
}

impl DegreePosition {
    /// Creates a new DegreePosition
    pub fn new(degrees: u16, minutes: u8, seconds: u8) -> Self {
        // Normalize values
        let mut total_seconds = seconds;
        let mut total_minutes = minutes;
        
        if total_seconds >= 60 {
            total_minutes += total_seconds / 60;
            total_seconds %= 60;
        }
        
        let mut total_degrees = degrees;
        if total_minutes >= 60 {
            total_degrees += total_minutes as u16 / 60;
            total_minutes %= 60;
        }
        
        total_degrees %= 360;
        
        Self {
            degrees: total_degrees,
            minutes: total_minutes,
            seconds: total_seconds,
        }
    }
    
    /// Creates from total degrees
    pub fn from_degrees(total_degrees: f32) -> Self {
        let normalized = normalize_degrees(total_degrees);
        let degrees = normalized.floor() as u16;
        let remaining_minutes = (normalized - degrees as f32) * 60.0;
        let minutes = remaining_minutes.floor() as u8;
        let seconds = ((remaining_minutes - minutes as f32) * 60.0).round() as u8;
        
        Self::new(degrees, minutes, seconds)
    }
    
    /// Creates from radians
    pub fn from_radians(radians: f32) -> Self {
        let degrees = radians_to_degrees(radians);
        Self::from_degrees(degrees)
    }
    
    /// Converts to total degrees
    pub fn to_degrees(&self) -> f32 {
        self.degrees as f32 + (self.minutes as f32 / 60.0) + (self.seconds as f32 / 3600.0)
    }
    
    /// Converts to radians
    pub fn to_radians(&self) -> f32 {
        degrees_to_radians(self.to_degrees())
    }
    
    /// Formats as string (e.g., "123°45'12"" or "123°45'" if no seconds)
    pub fn format(&self) -> String {
        if self.seconds == 0 {
            format!("{}°{:02}'", self.degrees, self.minutes)
        } else {
            format!("{}°{:02}'{:02}\"", self.degrees, self.minutes, self.seconds)
        }
    }
    
    /// Adds another DegreePosition
    pub fn add(&self, other: DegreePosition) -> Self {
        let total_degrees = self.to_degrees() + other.to_degrees();
        Self::from_degrees(total_degrees)
    }
    
    /// Subtracts another DegreePosition
    pub fn subtract(&self, other: DegreePosition) -> Self {
        let total_degrees = self.to_degrees() - other.to_degrees();
        Self::from_degrees(total_degrees)
    }
    
    /// Interpolates between this position and another
    pub fn interpolate(&self, other: DegreePosition, factor: f32) -> Self {
        let start_deg = self.to_degrees();
        let end_deg = other.to_degrees();
        
        // Handle crossing the 0°/360° boundary
        let mut diff = end_deg - start_deg;
        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }
        
        let result = start_deg + diff * factor;
        Self::from_degrees(result)
    }
    
    // Getters
    pub fn degrees(&self) -> u16 { self.degrees }
    pub fn minutes(&self) -> u8 { self.minutes }
    pub fn seconds(&self) -> u8 { self.seconds }
}

/// Convert an angle to its harmonic equivalent
pub fn to_harmonic_angle(angle: f32, harmonic: u8) -> f32 {
    if harmonic <= 1 {
        return angle;
    }
    
    normalize_degrees(angle * harmonic as f32)
}

/// Convert a harmonic angle back to the base angle
pub fn from_harmonic_angle(harmonic_angle: f32, harmonic: u8) -> f32 {
    if harmonic <= 1 {
        return harmonic_angle;
    }
    
    normalize_degrees(harmonic_angle / harmonic as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_degree_position_creation() {
        let pos = DegreePosition::new(123, 45, 12);
        assert_eq!(pos.degrees(), 123);
        assert_eq!(pos.minutes(), 45);
        assert_eq!(pos.seconds(), 12);
    }
    
    #[test]
    fn test_from_degrees() {
        let pos = DegreePosition::from_degrees(123.753333);
        assert_eq!(pos.degrees(), 123);
        assert_eq!(pos.minutes(), 45);
        assert_eq!(pos.seconds(), 12);
    }
    
    #[test]
    fn test_to_degrees() {
        let pos = DegreePosition::new(123, 45, 12);
        assert!((pos.to_degrees() - 123.753333).abs() < 0.0001);
    }
    
    #[test]
    fn test_format() {
        let pos1 = DegreePosition::new(123, 45, 12);
        assert_eq!(pos1.format(), "123°45'12\"");
        
        let pos2 = DegreePosition::new(90, 30, 0);
        assert_eq!(pos2.format(), "90°30'");
    }
    
    #[test]
    fn test_normalization() {
        let pos1 = DegreePosition::from_degrees(-45.0);
        assert_eq!(pos1.degrees(), 315);
        
        let pos2 = DegreePosition::from_degrees(370.0);
        assert_eq!(pos2.degrees(), 10);
    }
    
    #[test]
    fn test_add() {
        let pos1 = DegreePosition::from_degrees(30.0);
        let pos2 = DegreePosition::from_degrees(60.0);
        let result = pos1.add(pos2);
        assert_eq!(result.degrees(), 90);
    }
    
    #[test]
    fn test_subtract() {
        let pos1 = DegreePosition::from_degrees(90.0);
        let pos2 = DegreePosition::from_degrees(30.0);
        let result = pos1.subtract(pos2);
        assert_eq!(result.degrees(), 60);
    }
    
    #[test]
    fn test_interpolate() {
        let pos1 = DegreePosition::from_degrees(0.0);
        let pos2 = DegreePosition::from_degrees(90.0);
        
        let mid = pos1.interpolate(pos2, 0.5);
        assert_eq!(mid.degrees(), 45);
        
        let quarter = pos1.interpolate(pos2, 0.25);
        assert_eq!(quarter.degrees(), 22);
        assert_eq!(quarter.minutes(), 30);
    }
}