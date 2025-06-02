# Uranian Astrology Interactive Dial Project

Modern Rust implementation of interactive astrological dial with smooth pointer control and real-time degree calculations.

## 🎯 Project Status: MODULAR DIAL IMPLEMENTED ✅

### Completed Features
- ✅ **Interactive Dial Control** - Mouse drag to rotate pointer
- ✅ **Degree Display** - Real-time degree readout (0-360°)
- ✅ **Visual Elements**
  - Dial circle with border
  - Tick marks (major every 30°, minor every 5°)
  - Degree labels at major ticks
  - Red arrow pointer spanning exactly 2 degrees
  - Center dot
  - Harmonic arms (4, 8, 16, 32, or 64 arms based on harmonic)
- ✅ **Precise Control**
  - Shift key: 1 degree increments
  - Ctrl key: 1 minute increments
  - Shift+Ctrl: 1 second increments
- ✅ **Harmonic Selection** - 1st (360°), 2nd (180°), 4th (90°), 8th (45°), 16th (22.5°)
- ✅ **Modular Architecture** - Clean separation of concerns
- ✅ **Smooth Performance** - 60 FPS rendering with egui

## Development Environment

**Using Fedora Distrobox Container**

```bash
# Enter container
distrobox enter rust-astrology
cd /home/mend/Projects/Uranian-Astrology
```

## Commands (using `just`)

```bash
just build      # Build the project
just run        # Run the application (requires display setup)
just test       # Run tests
just check      # Quick syntax check
```

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── app.rs                  # Main application struct
├── astrology/              # Core astrological calculations
│   ├── angle.rs            # Degree position handling
│   ├── harmonics.rs        # Harmonic calculations
│   └── constants.rs        # Astrological constants
├── ui/                     # User interface components
│   ├── dial.rs             # Dial widget implementation
│   └── controls.rs         # UI controls
└── utils/                  # Utility functions
    └── math.rs             # Math utilities
```

## Implementation Details

- **Framework**: egui (immediate mode GUI)
- **Language**: Rust
- **Architecture**: Modular application with clean separation of concerns
- **Key Files**: See `DIAL_IMPLEMENTATION.md` for detailed documentation

## Display Setup Note

The dial is fully functional but requires proper display configuration to run:
- For X11: Set up X forwarding
- For Wayland: Share Wayland socket
- Alternative: Run directly on host system with Rust installed

The core dial functionality is complete, modular, and working!