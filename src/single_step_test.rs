// https://github.com/SingleStepTests/sm83
#[cfg(test)] 
mod single_step_test {
use std::ffi::OsString;
use std::{fs, path::PathBuf};

use crate::cpu;
use crate::ram;
use crate::util;
type SingleStepTestsRam = Vec<(u16, u8)>;

#[derive(serde::Serialize, serde::Deserialize)]
struct SingleStepTestsInitial {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    ram: SingleStepTestsRam,
}

type SingleStepTestsFinal = SingleStepTestsInitial;

type SingleStepTestsCycles = Vec<(u16, Option<u16>, String)>;

#[derive(serde::Serialize, serde::Deserialize)]
struct SingleStepTest {
    name: String,
    initial: SingleStepTestsInitial,
    r#final: SingleStepTestsFinal,
    cycles: SingleStepTestsCycles
}

fn run_individual_test(processor: &mut cpu::CPU, ram: &mut ram::RAM, test_json: &serde_json::Value) {
    
    println!("{}", test_json["name"]);
    
    let test: SingleStepTest = serde_json::from_value::<SingleStepTest>(test_json.clone())
        .expect("Could not deserialise JSON into Rust type");
    
    // Set up initial state of processor
    processor.register_file._af = util::_unsigned_16(test.initial.a, test.initial.f);
    processor._flags._zero = (test.initial.f & 0x80) != 0;
    processor._flags._subtraction = (test.initial.f & 0x40) != 0;
    processor._flags._half_carry = (test.initial.f & 0x20) != 0;
    processor._flags._carry = (test.initial.f & 0x10) != 0;
    processor.register_file._bc = util::_unsigned_16(test.initial.b, test.initial.c);
    processor.register_file._de = util::_unsigned_16(test.initial.d, test.initial.e);
    processor.register_file._hl = util::_unsigned_16(test.initial.h, test.initial.l);
    processor.register_file._sp = test.initial.sp;
    processor.register_file.pc = test.initial.pc;

    // Write to RAM
    for cell in test.initial.ram {
        println!("RAM write: address: {:#04X}, value: {:#02X}", cell.0, cell.1);
        ram._write(cell.1, cell.0)
            .expect("Test attempts to write to illegal or unwriteable RAM address");
    }

    // tick the CPU
    for _ in 0..(test.cycles.len()+1) {
        println!("tick!");
        processor.tick(ram)
            .expect("Test attempts to execute instruction which causes a CPU error");
    }

    // Compare the final state of the processor to the test
    assert_eq!(processor.register_file._af, util::_unsigned_16(test.r#final.a, test.r#final.f), "AF mismatch");
    assert_eq!(processor.register_file._bc, util::_unsigned_16(test.r#final.b, test.r#final.c), "BC mismatch");
    assert_eq!(processor.register_file._de, util::_unsigned_16(test.r#final.d, test.r#final.e), "DE mismatch");
    assert_eq!(processor.register_file._hl, util::_unsigned_16(test.r#final.h, test.r#final.l), "HL mismatch");
    assert_eq!(processor.register_file._sp, test.r#final.sp, "SP mismatch");
    assert_eq!(processor.register_file.pc, test.r#final.pc, "PC mismatch");

    // Compare the final state of RAM to the test
    for cell in test.r#final.ram {
        println!("address: {:#04X}, value: {:#02X}", cell.0, cell.1);
        let ram_value: u8 = ram.read(cell.0)
            .expect("Test attempts to read from illegal or unreadable RAM address when checking final state");
        assert_eq!(ram_value, cell.1, "RAM mismatch at address {:#04X}: should be {:#02X}, got {:#02X} ", cell.0, cell.1, ram_value);
    }

}

fn run_test_file(processor: &mut cpu::CPU, ram: &mut ram::RAM, path: &PathBuf) {
    
    let file_contents: String = fs::read_to_string(path)
        .expect("Could not read test file");
    let tests_json: serde_json::Value = serde_json::from_str(&file_contents)
        .expect("Could not parse test JSON");

    if let Some(tests_vector) = tests_json.as_array() {
        for test in tests_vector {
            run_individual_test(processor, ram, test);
        }
    } else {
        println!("Could not parse test JSON as JSON array");
        assert!(false);
    }
    
    println!("{:?}: passed", path.file_name().unwrap());
}

//#[test] // For now this is the default test
fn debug_run_test() {
        // initiate a new CPU which we will use to run all our tests
        let mut processor = cpu::init();

        // Initialise RAM. 
        let mut ram: ram::RAM = ram::init();
    
        let path_string: String = "./sm83/v1/00.json".to_string();

        let path: PathBuf = PathBuf::from(OsString::from(path_string));

        run_test_file(&mut processor, &mut ram, &path);
}

// #[test] TODO enable this as the test once all opcodes are implemented. 
// For now we only want to test individual opcodes at a time, since most instructions are unimplemented 
fn _run_all_tests() {

    // initiate a new CPU which we will use to run all our tests
    let mut processor = cpu::init();

    // Initialise RAM. 
    let mut ram: ram::RAM = ram::init();

    match fs::read_dir("./sm83/v1/") {
        Ok(value) => {
            let test_paths: fs::ReadDir = value;
            for test_path in test_paths {
                run_test_file(&mut processor, &mut ram, &test_path.expect("Could not read path").path())
            }
        },
        Err(_) => assert!(false)
    }
}

}