use std::f32::consts::PI;

/// Normalizes an angle in degrees to [0, 360)
pub fn normalize_degrees(degrees: f32) -> f32 {
    degrees.rem_euclid(360.0)
}

/// Normalizes an angle in radians to [0, 2π)
pub fn normalize_radians(radians: f32) -> f32 {
    radians.rem_euclid(2.0 * PI)
}

/// Converts degrees to radians
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Converts radians to degrees
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

/// Calculates the smallest angular difference between two angles in degrees
pub fn angular_difference(a: f32, b: f32) -> f32 {
    let diff = normalize_degrees(b - a);
    if diff > 180.0 {
        diff - 360.0
    } else {
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_degrees() {
        assert_eq!(normalize_degrees(30.0), 30.0);
        assert_eq!(normalize_degrees(370.0), 10.0);
        assert_eq!(normalize_degrees(-30.0), 330.0);
        assert_eq!(normalize_degrees(-370.0), 350.0);
    }
    
    #[test]
    fn test_normalize_radians() {
        assert_eq!(normalize_radians(PI / 6.0), PI / 6.0);
        assert!((normalize_radians(PI * 2.0 + PI / 6.0) - PI / 6.0).abs() < 1e-6);
        assert!((normalize_radians(-PI / 6.0) - (PI * 2.0 - PI / 6.0)).abs() < 1e-6);
    }
    
    #[test]
    fn test_degrees_to_radians() {
        assert!((degrees_to_radians(180.0) - PI).abs() < 1e-6);
        assert!((degrees_to_radians(90.0) - PI / 2.0).abs() < 1e-6);
        assert!((degrees_to_radians(60.0) - PI / 3.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_radians_to_degrees() {
        assert!((radians_to_degrees(PI) - 180.0).abs() < 1e-6);
        assert!((radians_to_degrees(PI / 2.0) - 90.0).abs() < 1e-6);
        assert!((radians_to_degrees(PI / 3.0) - 60.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_angular_difference() {
        assert_eq!(angular_difference(0.0, 30.0), 30.0);
        assert_eq!(angular_difference(30.0, 0.0), -30.0);
        
        // Test across the 0/360 boundary
        assert_eq!(angular_difference(350.0, 10.0), 20.0);
        assert_eq!(angular_difference(10.0, 350.0), -20.0);
        
        // Test large differences (should take the shortest path)
        assert_eq!(angular_difference(0.0, 190.0), -170.0);
        assert_eq!(angular_difference(190.0, 0.0), 170.0);
    }
}