# Uranian Astrology API Reference

## astrology::angle

### DegreePosition

Represents an angular position in degrees, minutes, and seconds.

```rust
struct DegreePosition {
    degrees: u16,
    minutes: u8,
    seconds: u8,
}
```

#### Methods

- `new(degrees: u16, minutes: u8, seconds: u8) -> Self`: Creates a new position with the given components
- `from_degrees(total_degrees: f32) -> Self`: Creates from total degrees
- `from_radians(radians: f32) -> Self`: Creates from radians
- `to_degrees(&self) -> f32`: Converts to total degrees
- `to_radians(&self) -> f32`: Converts to radians
- `format(&self) -> String`: Formats as string (e.g., "123°45'12"" or "123°45'" if no seconds)
- `add(&self, other: DegreePosition) -> Self`: Adds another position
- `subtract(&self, other: DegreePosition) -> Self`: Subtracts another position
- `interpolate(&self, other: DegreePosition, factor: f32) -> Self`: Interpolates between positions

## astrology::harmonics

### Harmonic

Represents a harmonic dial setting.

```rust
enum Harmonic {
    First,      // 360° - standard zodiac
    Second,     // 180° - opposition dial
    Fourth,     // 90° - square dial
    Eighth,     // 45° - eighth harmonic dial
    Sixteenth,  // 22.5° - sixteenth harmonic dial
    Custom(u8), // Custom harmonic
}
```

#### Methods

- `from_number(num: u8) -> Self`: Creates a harmonic from a number
- `to_number(&self) -> u8`: Converts to a number
- `angle_span(&self) -> f32`: Returns the angle span for this harmonic
- `arm_count(&self) -> u8`: Returns the number of arms to display
- `display_name(&self) -> String`: Returns a display name for this harmonic

#### Functions

- `calculate_harmonic_position(position: f32, harmonic: Harmonic) -> f32`: Calculates the position in a harmonic
- `calculate_original_position(harmonic_position: f32, harmonic: Harmonic) -> f32`: Calculates the original position

## ui::dial

### Dial

The main dial widget for rendering and interaction.

```rust
struct Dial {
    center: egui::Pos2,
    radius: f32,
    current_position: DegreePosition,
    harmonic: Harmonic,
    is_dragging: bool,
    shift_pressed: bool,
    ctrl_pressed: bool,
}
```

#### Methods

- `new() -> Self`: Creates a new dial
- `get_increment(&self) -> f32`: Gets the increment based on modifier keys
- `snap_angle(&self, angle: f32) -> f32`: Snaps angle to the current increment
- `position(&self) -> DegreePosition`: Gets the current position
- `set_position(&mut self, position: DegreePosition)`: Sets the position
- `harmonic(&self) -> Harmonic`: Gets the current harmonic
- `set_harmonic(&mut self, harmonic: Harmonic)`: Sets the harmonic
- `update_modifiers(&mut self, modifiers: egui::Modifiers)`: Updates modifier key states
- `handle_key(&mut self, key: egui::Key)`: Handles keyboard input
- `ui(&mut self, ui: &mut egui::Ui) -> egui::Response`: Renders the dial

## utils::math

Various mathematical utility functions:

- `normalize_degrees(degrees: f32) -> f32`: Normalizes an angle in degrees to [0, 360)
- `normalize_radians(radians: f32) -> f32`: Normalizes an angle in radians to [0, 2π)
- `degrees_to_radians(degrees: f32) -> f32`: Converts degrees to radians
- `radians_to_degrees(radians: f32) -> f32`: Converts radians to degrees
- `angular_difference(a: f32, b: f32) -> f32`: Calculates the smallest angular difference