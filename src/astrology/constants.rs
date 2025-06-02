/// Cardinal points (0°, 90°, 180°, 270°)
pub const CARDINAL_POINTS: [f32; 4] = [0.0, 90.0, 180.0, 270.0];

/// Fixed points (45°, 135°, 225°, 315°)
pub const FIXED_POINTS: [f32; 4] = [45.0, 135.0, 225.0, 315.0];

/// Mutable points (30°, 60°, 120°, 150°, 210°, 240°, 300°, 330°)
pub const MUTABLE_POINTS: [f32; 8] = [
    30.0, 60.0, 120.0, 150.0, 210.0, 240.0, 300.0, 330.0,
];

/// Standard major aspect angles and their names
pub const MAJOR_ASPECTS: [(f32, &str); 5] = [
    (0.0, "Conjunction"),
    (60.0, "Sextile"),
    (90.0, "Square"),
    (120.0, "Trine"),
    (180.0, "Opposition"),
];

/// Standard minor aspect angles and their names
pub const MINOR_ASPECTS: [(f32, &str); 5] = [
    (30.0, "Semi-sextile"),
    (45.0, "Semi-square"),
    (135.0, "Sesquiquadrate"),
    (150.0, "Quincunx"),
    (72.0, "Quintile"),
];

/// Default orb for aspects in degrees
pub const DEFAULT_ORB: f32 = 2.0;