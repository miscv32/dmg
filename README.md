# dmg

Game Boy emulator backend.

Current progress
- All CPU instructions implemented and behaviour verified
- Partially working scanline renderer (background only)
- Partially working interrupt system

How to use
```rust
// Create a new Game Boy object with default configuration values
let mut gameboy = gb::init();

// We can advance Game Boy state by 1 M-cycle*
gameboy.tick(); // to match Game Boy's original operating frequency call this at ~1MHz (17556 times per frame, at 59.7fps)
// *actually, the CPU state is not exactly cycle-accurate for instructions that take more than 1 tick. 
// However, this is mostly unobservable, the CPU state before/after an instruction should be correct

// Display is simply a flat array of 160x144 pixels, each taking on one of 4 values from 0 (lightest) to 3 (darkest).
// Pixel ordering: display[0] is top left and display[160*144-1] is bottom right.
your_drawing_function(gameboy.display); 
// The display is updated every 17556 ticks (or less frequently, depending on LCD disable/halting).
// To see intermediate output look at gameboy.display_temp
```

Next steps
- fully working scanline renderer
- complete interrupt system
- interface to handle input
