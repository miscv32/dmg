// https://github.com/SingleStepTests/sm83

// TODO rewrite so that we don't just exit on first failure

#[cfg(test)]
mod single_step_test {
    use crate::memory::Memory;
    use crate::{gb, memory};
    use std::ffi::OsString;
    use std::{fs, path::PathBuf};
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
        cycles: SingleStepTestsCycles,
    }

    fn run_individual_test(gameboy: &mut gb::GameBoy, test_json: &serde_json::Value) {
        let test: SingleStepTest = serde_json::from_value::<SingleStepTest>(test_json.clone())
            .expect("Could not deserialise JSON into Rust type");

        // Set up initial state of processor
        gameboy.registers.a = test.initial.a;
        gameboy.registers.f = test.initial.f;
        gameboy.registers.b = test.initial.b;
        gameboy.registers.c = test.initial.c;
        gameboy.registers.d = test.initial.d;
        gameboy.registers.e = test.initial.e;
        gameboy.registers.h = test.initial.h;
        gameboy.registers.l = test.initial.l;
        gameboy.registers.sp = test.initial.sp;
        gameboy.registers.pc = test.initial.pc;
        gameboy.running = true;
        gameboy.cycles_to_idle = Some(0);

        // Write to RAM
        for cell in test.initial.ram {
            gameboy.memory.write(cell.0, cell.1);
        }

        // tick the CPU
        for _ in 0..(test.cycles.len() + 1) {
            gameboy.tick();
        }

        // Compare the final state of the processor to the test
        assert_eq!(gameboy.registers.a, test.r#final.a, "A mismatch");
        assert_eq!(gameboy.registers.f, test.r#final.f, "F mismatch");
        assert_eq!(gameboy.registers.b, test.r#final.b, "B mismatch");
        assert_eq!(gameboy.registers.c, test.r#final.c, "C mismatch");
        assert_eq!(gameboy.registers.d, test.r#final.d, "D mismatch");
        assert_eq!(gameboy.registers.e, test.r#final.e, "E mismatch");
        assert_eq!(gameboy.registers.h, test.r#final.h, "H mismatch");
        assert_eq!(gameboy.registers.l, test.r#final.l, "L mismatch");
        assert_eq!(gameboy.registers.sp, test.r#final.sp, "SP mismatch");
        assert_eq!(gameboy.registers.pc, test.r#final.pc, "PC mismatch");

        // Compare the final state of RAM to the test
        for cell in test.r#final.ram {
            let ram_value: u8 = gameboy.memory.read(cell.0);
            assert_eq!(ram_value, cell.1, "RAM mismatch at address {:#04X}", cell.0);
        }
    }

    fn run_test_file(gameboy: &mut gb::GameBoy, path: &PathBuf) {
        println!("{:?}", path.file_name().unwrap());
        let file_contents: String = fs::read_to_string(path).expect("Could not read test file");
        let tests_json: serde_json::Value =
            serde_json::from_str(&file_contents).expect("Could not parse test JSON");

        if let Some(tests_vector) = tests_json.as_array() {
            for test in tests_vector {
                run_individual_test(gameboy, test);
            }
        } else {
            println!("Could not parse test JSON as JSON array");
            assert!(false);
        }
    }

    // #[test]
    fn _debug_run_test() {
        let mut gameboy = gb::init();

        let path_strings: Vec<&str> = vec![
            "./sm83/v1/e8.json",
            "./sm83/v1/e0.json",
            "./sm83/v1/00.json",
            "./sm83/v1/76.json",
        ];
        for path_string in path_strings {
            let path: PathBuf = PathBuf::from(OsString::from(path_string));
            run_test_file(&mut gameboy, &path);
        }
    }

    #[test]
    fn _run_all_tests() {
        let mut gameboy = gb::init();

        match fs::read_dir("./sm83/v1/") {
            Ok(value) => {
                let test_paths: fs::ReadDir = value;
                let mut i: u32 = 0;
                for test_path in test_paths {
                    i += 1;
                    print!("running test {}: ", i);
                    run_test_file(
                        &mut gameboy,
                        &test_path.expect("Could not read path").path(),
                    )
                }
            }
            Err(_) => assert!(false),
        }
    }
}
