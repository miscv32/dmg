use crate::memory;
use crate::memory::Memory;
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

pub struct GameBoy {
    pub clock: u32,
    pub running: bool,
    pub registers: Registers,
    pub cycles_to_idle: Option<u8>,
    pub memory: memory::FlatRAM,
    pub ime: bool,
}

pub fn init() -> GameBoy {
    let registers: Registers = Registers {
        a: 0,
        f: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        sp: 0,
        pc: 0,
    };

    let memory: [u8; 65536] = memory::init();

    GameBoy {
        clock: 0,
        running: true,
        registers: registers,
        cycles_to_idle: Some(0),
        memory: memory,
        ime: false,
    }
}

impl GameBoy {
    pub fn tick(&mut self) {
        // This should be called once every M-cycle.
        // Current behaviour is M-cycle faking, i.e. all work is done in first M-cycle
        // CPU & RAM idle for the rest of the instruction's M-cycles
        if self.running {
            if let Some(cycles_to_idle) = self.cycles_to_idle {
                if cycles_to_idle == 0 {
                    let opcode: u8 = self.memory.read(self.registers.pc);
                    self.registers.pc += 1;
                    self.cycles_to_idle = self.fetch_decode_execute(opcode);
                }
            }
            self.clock += 1;
        }
    }
}
