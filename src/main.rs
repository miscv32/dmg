mod gb;
mod memory;
mod single_step_tests;
mod util;
fn main() {
    // Initialise CPU internals and RAM.
    let mut gameboy: gb::GameBoy = gb::init();

    loop {
        gameboy.tick();
    }
}
