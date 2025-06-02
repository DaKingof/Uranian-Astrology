/// Represents a harmonic dial setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harmonic {
    First,   // 360° - standard zodiac
    Second,  // 180° - opposition dial
    Fourth,  // 90° - square dial
    Eighth,  // 45° - eighth harmonic dial
    Sixteenth, // 22.5° - sixteenth harmonic dial
    Custom(u8), // Custom harmonic
}

impl Harmonic {
    /// Creates a harmonic from a number
    pub fn from_number(num: u8) -> Self {
        match num {
            1 => Self::First,
            2 => Self::Second,
            4 => Self::Fourth,
            8 => Self::Eighth,
            16 => Self::Sixteenth,
            n => Self::Custom(n),
        }
    }
    
    /// Converts to a number
    pub fn to_number(&self) -> u8 {
        match self {
            Self::First => 1,
            Self::Second => 2,
            Self::Fourth => 4,
            Self::Eighth => 8,
            Self::Sixteenth => 16,
            Self::Custom(n) => *n,
        }
    }
    
    /// Returns the angle span for this harmonic
    pub fn angle_span(&self) -> f32 {
        360.0 / self.to_number() as f32
    }
    
    /// Returns the number of arms to display
    pub fn arm_count(&self) -> u8 {
        match self {
            Self::First => 4,     // 4 arms for cardinal points
            Self::Second => 8,    // 8 arms
            Self::Fourth => 16,   // 16 arms
            Self::Eighth => 32,   // 32 arms
            Self::Sixteenth => 64, // 64 arms
            Self::Custom(n) => n * 4, // 4 times the harmonic number
        }
    }
    
    /// Returns a display name for this harmonic
    pub fn display_name(&self) -> String {
        match self {
            Self::First => format!("1st (360°)"),
            Self::Second => format!("2nd (180°)"),
            Self::Fourth => format!("4th (90°)"),
            Self::Eighth => format!("8th (45°)"),
            Self::Sixteenth => format!("16th (22.5°)"),
            Self::Custom(n) => format!("{}th ({:.1}°)", n, 360.0 / *n as f32),
        }
    }
}

impl Default for Harmonic {
    fn default() -> Self {
        Self::First
    }
}

/// Calculates the position in the specified harmonic
pub fn calculate_harmonic_position(position: f32, harmonic: Harmonic) -> f32 {
    (position * harmonic.to_number() as f32) % 360.0
}

/// Calculates the original position from a harmonic position
pub fn calculate_original_position(harmonic_position: f32, harmonic: Harmonic) -> f32 {
    (harmonic_position / harmonic.to_number() as f32) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_harmonic_from_number() {
        assert_eq!(Harmonic::from_number(1), Harmonic::First);
        assert_eq!(Harmonic::from_number(2), Harmonic::Second);
        assert_eq!(Harmonic::from_number(4), Harmonic::Fourth);
        assert_eq!(Harmonic::from_number(8), Harmonic::Eighth);
        assert_eq!(Harmonic::from_number(16), Harmonic::Sixteenth);
        
        if let Harmonic::Custom(n) = Harmonic::from_number(7) {
            assert_eq!(n, 7);
        } else {
            panic!("Expected Custom(7)");
        }
    }
    
    #[test]
    fn test_harmonic_to_number() {
        assert_eq!(Harmonic::First.to_number(), 1);
        assert_eq!(Harmonic::Second.to_number(), 2);
        assert_eq!(Harmonic::Fourth.to_number(), 4);
        assert_eq!(Harmonic::Eighth.to_number(), 8);
        assert_eq!(Harmonic::Sixteenth.to_number(), 16);
        assert_eq!(Harmonic::Custom(7).to_number(), 7);
    }
    
    #[test]
    fn test_angle_span() {
        assert_eq!(Harmonic::First.angle_span(), 360.0);
        assert_eq!(Harmonic::Second.angle_span(), 180.0);
        assert_eq!(Harmonic::Fourth.angle_span(), 90.0);
        assert_eq!(Harmonic::Eighth.angle_span(), 45.0);
        assert_eq!(Harmonic::Sixteenth.angle_span(), 22.5);
        assert_eq!(Harmonic::Custom(12).angle_span(), 30.0);
    }
    
    #[test]
    fn test_arm_count() {
        assert_eq!(Harmonic::First.arm_count(), 4);
        assert_eq!(Harmonic::Second.arm_count(), 8);
        assert_eq!(Harmonic::Fourth.arm_count(), 16);
        assert_eq!(Harmonic::Eighth.arm_count(), 32);
        assert_eq!(Harmonic::Sixteenth.arm_count(), 64);
        assert_eq!(Harmonic::Custom(3).arm_count(), 12);
    }
    
    #[test]
    fn test_calculate_harmonic_position() {
        assert_eq!(calculate_harmonic_position(30.0, Harmonic::First), 30.0);
        assert_eq!(calculate_harmonic_position(30.0, Harmonic::Second), 60.0);
        assert_eq!(calculate_harmonic_position(30.0, Harmonic::Fourth), 120.0);
        assert_eq!(calculate_harmonic_position(30.0, Harmonic::Eighth), 240.0);
        
        // Test wrap-around
        assert_eq!(calculate_harmonic_position(90.0, Harmonic::Fourth), 0.0);
    }
    
    #[test]
    fn test_calculate_original_position() {
        assert_eq!(calculate_original_position(30.0, Harmonic::First), 30.0);
        assert_eq!(calculate_original_position(60.0, Harmonic::Second), 30.0);
        assert_eq!(calculate_original_position(120.0, Harmonic::Fourth), 30.0);
        assert_eq!(calculate_original_position(240.0, Harmonic::Eighth), 30.0);
    }
}