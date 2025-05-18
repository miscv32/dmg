use crate::{memory, util};
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
    clock: u32,
    pub running: bool,
    pub registers: Registers,
    pub cycles_to_idle: Option<u8>,
    pub memory: memory::FlatRAM,
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

    let memory = memory::init();

    GameBoy {
        clock: 0,
        running: true,
        registers: registers,
        cycles_to_idle: Some(0),
        memory: memory,
    }
}

impl GameBoy {
    pub fn tick(&mut self) {
        // Not m-cycle accurate, but 1 tick = 1 m cycle for easier timing
        if self.running {
            if let Some(cycles_to_idle) = self.cycles_to_idle {
                if cycles_to_idle == 0 {
                    let opcode: u8 = self.memory.read(self.registers.pc);
                    self.registers.pc += 1;
                    self.cycles_to_idle = self.fetch_decode_execute(opcode);
                }
            }
        }
    }

    fn fetch_decode_execute(&mut self, opcode: u8) -> Option<u8> {
        match opcode {
            0x00 => {
                Some(1)
            },
            0x08 => {
                let nn_lsb: u8 = self.memory.read(self.registers.pc);
                self.registers.pc += 1;
                let nn_msb: u8 = self.memory.read(self.registers.pc);
                self.registers.pc += 1;
                let mut nn: u16 = util::unsigned_16(nn_msb, nn_lsb);
                self.memory.write(nn, util::lsb(self.registers.sp));
                nn += 1;
                self.memory.write(nn, util::msb(self.registers.sp));
                Some(5)
            },
            0x10 => {
                // TODO verify this is correct behaviour.
                // Not sure it matters that much, i think only CGB uses this
                self.running = false;
                Some(1)
            },
            0x76 => {
                self.running = false;
                None
            },
            0xE0 => {
                let n = self.memory.read(self.registers.pc);
                println!("{:#04x}", n);
                self.registers.pc += 1;
                self.memory.write( util::unsigned_16(0xFF, n), self.registers.a);
                Some(3)
            },
            0xE8 => {Some(4)},
            0xF0 => {Some(3)},
            0xF8 => {Some(3)},
            0xE2 => {Some(2)},
            0xEA => {Some(4)},
            0xF2 => {Some(2)},
            0xFA => {Some(4)},
            0xCD => {Some(6)},
            _ => None,
        }
    }
}
