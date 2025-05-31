#!/bin/bash
# Run the Uranian Astrology dial application

# Enter distrobox and run with display forwarding
distrobox enter rust-astrology -- bash -c "
    source \$HOME/.cargo/env
    cd /home/mend/Projects/Uranian-Astrology
    
    # Try different display methods
    if [ -n \"\$WAYLAND_DISPLAY\" ]; then
        echo 'Running with Wayland...'
        cargo run
    elif [ -n \"\$DISPLAY\" ]; then
        echo 'Running with X11...'
        cargo run
    else
        echo 'No display found, trying with default settings...'
        DISPLAY=:0 cargo run
    fi
"