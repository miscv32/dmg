#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]
mod fde;
mod gb;
mod memory;
mod single_step_tests;
mod util;

fn main() {
    let mut gameboy: gb::GameBoy = gb::init();

    loop {
        gameboy.tick();
    }
}
