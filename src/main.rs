mod cpu;
mod ram;
mod util;
mod decode;
mod single_step_test;

fn main() {
    // Initialise CPU internals.
    let mut processor = cpu::init();

    // Initialise RAM. 
    let mut ram: ram::RAM = ram::init();

    // Main CPU loop
    while processor.running {
        let result: Result<(), cpu::CPUError> = processor.tick(&mut ram);
        match result {
            Err(_) => processor.running = false, // TODO print debug messages
            Ok(()) => (),
        }
    }
    // TODO look into JSON to save CPU state, so we can use existing tests

}
