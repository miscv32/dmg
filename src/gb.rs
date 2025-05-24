// TODO clean up this file especially fetch_decode_execute()
use core::panic;

use crate::memory::Memory;
use crate::{memory, util};
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
    fn set_flag_z(&mut self, value: bool) {
        match value {
            true => self.registers.f |= 0x80,
            false => self.registers.f &= 0x7F,
        }
    }
    fn set_flag_n(&mut self, value: bool) {
        match value {
            true => self.registers.f |= 0x40,
            false => self.registers.f &= 0xBF,
        }
    }
    fn set_flag_h(&mut self, value: bool) {
        match value {
            true => self.registers.f |= 0x20,
            false => self.registers.f &= 0xDF,
        }
    }
    fn set_flag_c(&mut self, value: bool) {
        match value {
            true => self.registers.f |= 0x10,
            false => self.registers.f &= 0xEF,
        }
    }

    fn get_flag_c(&self) -> u8 {
        match self.registers.f & 0x10 == 0 {
            true => 0,
            false => 1,
        }
    }

    fn get_flag_z(&self) -> u8 {
        match self.registers.f & 0x80 == 0 {
            true => 0,
            false => 1,
        }
    }

    fn _get_flag_n(&self) -> u8 {
        match self.registers.f & 0x40 == 0 {
            true => 0,
            false => 1,
        }
    }

    fn _get_flag_h(&self) -> u8 {
        match self.registers.f & 0x20 == 0 {
            true => 0,
            false => 1,
        }
    }

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
            self.clock += 1;
        }
    }

    fn fetch_decode_execute(&mut self, opcode: u8) -> Option<u8> {
        match opcode {
            // TODO implement the rest of the opcodes
            0x00 => Some(1),
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
            }
            0x10 => {
                // TODO verify this is correct behaviour.
                // Not sure it matters that much, i think only CGB uses this
                self.running = false;
                Some(1)
            }
            0x76 => {
                self.running = false;
                None
            }
            0xCB => {
                // 0xCB prefixed opcodes
                let cb_opcode: u8 = self.memory.read(self.registers.pc);
                let r8: u8 = cb_opcode & 0b111;
                let bit = (cb_opcode >> 3) & 0b111;
                match cb_opcode >> 6 {
                    0b00 => {
                        match (cb_opcode >> 3) & 0b111 {
                            0 => {
                                // RLC
                                self.registers.pc += 1;
                                let ms_bit = self.get_r8(r8) & 0x80;
                                self.set_flag_c(ms_bit != 0);
                                self.set_r8(r8, (self.get_r8(r8) << 1) | (ms_bit >> 7));
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                Some(2)
                            }
                            1 => {
                                // RRC
                                self.registers.pc += 1;
                                let ls_bit = self.get_r8(r8) & 0x01;
                                self.set_flag_c(ls_bit != 0);
                                self.set_r8(r8, (self.get_r8(r8) >> 1) | (ls_bit << 7));
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                Some(2)
                            }
                            2 => {
                                // RL
                                self.registers.pc += 1;
                                let ms_bit = self.get_r8(r8) & 0x80;
                                self.set_r8(r8, (self.get_r8(r8) << 1) | self.get_flag_c());
                                self.set_flag_c(ms_bit != 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                Some(2)
                            }
                            3 => {
                                // RR
                                self.registers.pc += 1;
                                let ls_bit = self.get_r8(r8) & 0x01;
                                self.set_r8(r8, (self.get_r8(r8) >> 1) | self.get_flag_c() << 7);
                                self.set_flag_c(ls_bit != 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                Some(2)
                            }
                            4 => {
                                // SLA
                                self.registers.pc += 1;
                                let ms_bit = self.get_r8(r8) & 0x80;
                                self.set_r8(r8, (self.get_r8(r8) << 1) | 0);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_c(ms_bit != 0);
                                Some(2)
                            }
                            5 => {
                                // SRA
                                self.registers.pc += 1;
                                let ms_bit = self.get_r8(r8) & 0x80;
                                let ls_bit = self.get_r8(r8) & 0x01;
                                self.set_r8(r8, (self.get_r8(r8) >> 1) | ms_bit);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_c(ls_bit != 0);
                                Some(2)
                            }
                            6 => {
                                // SWAP
                                self.registers.pc += 1;
                                let r8 = cb_opcode & 0b111;
                                let r8_value = self.get_r8(r8);
                                let r8_high_shift = (r8_value & 0xF0) >> 4;
                                let r8_low_shift = (r8_value & 0x0F) << 4;
                                let result = r8_high_shift | r8_low_shift;
                                self.set_r8(r8, result);
                                self.set_flag_z(result == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_c(false);
                                Some(2)
                            }
                            7 => {
                                // SRL
                                self.registers.pc += 1;
                                let r8 = cb_opcode & 0b111;
                                let ls_bit = self.get_r8(r8) & 0x01;
                                self.set_r8(r8, self.get_r8(r8) >> 1);
                                self.set_flag_c(ls_bit != 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_z(self.get_r8(r8) == 0);
                                Some(2)
                            }
                            _ => None,
                        }
                    }
                    0b01 => {
                        self.registers.pc += 1;
                        self.set_flag_z((self.get_r8(r8) & (1 << bit)) == 0);
                        self.set_flag_n(false);
                        self.set_flag_h(true);
                        Some(2)
                    }
                    0b10 => {
                        self.registers.pc += 1;
                        self.set_r8(r8, self.get_r8(r8) & !(1 << bit));
                        Some(2)
                    }
                    0b11 => {
                        self.registers.pc += 1;
                        self.set_r8(r8, self.get_r8(r8) | (1 << bit));
                        Some(2)
                    }
                    _ => None,
                }
            }
            0xE0 => {
                let n = self.memory.read(self.registers.pc);
                self.registers.pc += 1;
                self.memory
                    .write(util::unsigned_16(0xFF, n), self.registers.a);
                Some(3)
            }
            0xE8 => {
                // ADD SP i8
                let e: i8 = self.memory.read(self.registers.pc) as i8;
                self.registers.pc += 1;
                self.set_flag_z(false);
                self.set_flag_n(false);
                // ugly but I think all of these casts are necessary
                self.set_flag_h((((self.registers.sp & 0xF) as i16) + ((e & 0xF) as i16)) > 0xF);
                self.set_flag_c((((self.registers.sp & 0xFF) as i16) + (e as i16 & 0xFF)) > 0xFF);
                self.registers.sp = (self.registers.sp as i16 + e as i16) as u16;
                Some(4)
            }
            0xF0 => Some(3),
            0xF8 => {
                // LD HL SP + i8
                let e: i8 = self.memory.read(self.registers.pc) as i8;
                self.registers.pc += 1;
                self.set_flag_z(false);
                self.set_flag_n(false);
                // ugly but I think all of these casts are necessary
                self.set_flag_h((((self.registers.sp & 0xF) as i16) + ((e & 0xF) as i16)) > 0xF);
                self.set_flag_c((((self.registers.sp & 0xFF) as i16) + (e as i16 & 0xFF)) > 0xFF);
                self.set_hl((self.registers.sp as i16 + e as i16) as u16);

                Some(3)
            }
            0xF9 => {
                self.registers.sp = self.get_hl();
                Some(2)
            }
            0xE2 => Some(2),
            0xEA => Some(4),
            0xF2 => Some(2),
            0xFA => {
                let lsb = self.memory.read(self.registers.pc);
                self.registers.pc += 1;
                let msb = self.memory.read(self.registers.pc);
                self.registers.pc += 1;
                self.registers.a = self.memory.read(util::unsigned_16(msb, lsb));
                Some(4)
            }
            0xCD => Some(6),
            _ => {
                match opcode >> 6 {
                    0b00 => {
                        let r16 = (opcode >> 4) & 0b11;
                        match opcode & 0b1111 {
                            0b0001 => {
                                // LD r16 u16
                                let lsb = self.memory.read(self.registers.pc);
                                self.registers.pc += 1;
                                let msb = self.memory.read(self.registers.pc);
                                self.registers.pc += 1;
                                self.set_r16_group_1(r16, util::unsigned_16(msb, lsb));
                                Some(3)
                            }
                            0b0011 => {
                                // INC r16
                                let r16_value = self.get_r16_group_2(r16);
                                self.set_r16_group_1(r16, r16_value + 1);
                                Some(1)
                            }
                            0b1011 => {
                                // DEC r16
                                let r16_value = self.get_r16_group_2(r16);
                                self.set_r16_group_1(r16, r16_value - 1);
                                Some(1)
                            }
                            0b1001 => {
                                // Add HL r16
                                let left = self.get_hl();
                                let right = self.get_r16_group_1(r16);
                                let result = left + right;
                                self.set_hl(result);
                                self.set_flag_n(false);
                                self.set_flag_h((left & 0xFFF) + (right & 0xFFF) > 0xFFF);
                                self.set_flag_c(left as u32 + right as u32 > 0xFFFF);
                                Some(2)
                            }
                            0b0010 => {
                                // LD (r16), A
                                let r16_value = self.get_r16_group_2(r16);
                                self.memory.write(r16_value, self.registers.a);
                                if r16 == 3 {
                                    self.set_hl(self.get_hl() + 1);
                                }
                                Some(2)
                            }
                            0b1010 => {
                                // TODO complete LD A, (r16)
                                let r16_value: u16 = self.get_r16_group_2(r16);
                                self.registers.a = self.memory.read(r16_value);
                                Some(2)
                            }
                            _ => {
                                match opcode & 0b11_000_111 {
                                    0b00_000_110 => {
                                        // LD r8, u8
                                        let r8 = opcode >> 3;
                                        self.set_r8(r8, self.memory.read(self.registers.pc));
                                        self.registers.pc += 1;
                                        Some(2)
                                    }
                                    0b00_000_100 => {
                                        // INC r8
                                        let r8 = opcode >> 3;
                                        let r8_old = self.get_r8(r8);
                                        let result = self.get_r8(r8) + 1;
                                        self.set_r8(r8, result);
                                        self.set_flag_z(result == 0);
                                        self.set_flag_n(false);
                                        self.set_flag_h((r8_old & 0xF) + 1 > 0xF);
                                        Some(1)
                                    }
                                    0b00_000_101 => {
                                        // DEC r8
                                        let r8 = opcode >> 3;
                                        let r8_old = self.get_r8(r8);
                                        let result = self.get_r8(r8) - 1;
                                        self.set_r8(r8, result);
                                        self.set_flag_z(result == 0);
                                        self.set_flag_n(true);
                                        self.set_flag_h((r8_old & 0xF) - 1 > 0xF);
                                        Some(1)
                                    }
                                    0b00_000_111 => {
                                        // opcode group 1
                                        match opcode >> 3 {
                                            0 => {
                                                // RLCA
                                                let ms_bit = self.registers.a & 0x80;
                                                self.registers.a =
                                                    (self.registers.a << 1) | (ms_bit >> 7);
                                                self.set_flag_c(ms_bit != 0);
                                                self.set_flag_n(false);
                                                self.set_flag_h(false);
                                                self.set_flag_z(false);
                                                Some(1)
                                            }
                                            5 => {
                                                // CPL
                                                self.registers.a = !self.registers.a;
                                                self.set_flag_n(true);
                                                self.set_flag_h(true);
                                                Some(1)
                                            }
                                            6 => {
                                                // SCF
                                                self.set_flag_n(false);
                                                self.set_flag_h(false);
                                                self.set_flag_c(true);
                                                Some(1)
                                            }
                                            7 => {
                                                // CCF
                                                self.set_flag_n(false);
                                                self.set_flag_h(false);
                                                self.set_flag_c(!self.get_flag_c() != 0);
                                                Some(1)
                                            }
                                            _ => None,
                                        }
                                    }
                                    _ => {
                                        match opcode >> 5 {
                                            0b001 => {
                                                let condition;
                                                match (opcode >> 3) & 0b11 {
                                                    // JR conditional
                                                    0 => condition = self.get_flag_z() == 0,
                                                    1 => condition = self.get_flag_z() != 0,
                                                    2 => condition = self.get_flag_c() == 0,
                                                    3 => condition = self.get_flag_c() != 0,
                                                    _ => panic!(
                                                        "not possible condition - JR conditional"
                                                    ),
                                                }
                                                let e = self.memory.read(self.registers.pc) as i8;
                                                self.registers.pc += 1;
                                                if condition {
                                                    self.registers.pc = (self.registers.pc as i16
                                                        + e as i16)
                                                        as u16;
                                                    Some(3)
                                                } else {
                                                    Some(2)
                                                }
                                            }
                                            _ => None,
                                        }
                                    }
                                }
                            }
                        }
                    }
                    0b01 => {
                        let r8_source: u8 = opcode & 0b111;
                        let r8_dest: u8 = (opcode >> 3) & 0b111;
                        self.set_r8(r8_dest, self.get_r8(r8_source));
                        Some(1)
                    }
                    0b10 => {
                        // ALU A, r8
                        match (opcode >> 3) & 0b111 {
                            0 => {
                                let r8: u8 = opcode & 0b111;
                                let left: u8 = self.registers.a;
                                let right: u8 = self.get_r8(r8);
                                self.registers.a += right;
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(((left & 0xF) + (right & 0xF)) > 0xF);
                                self.set_flag_c(((left as u16) + (right as u16)) > 0xFF);
                                Some(1)
                            }
                            1 => {
                                let r8: u8 = opcode & 0b111;
                                let c_save: u8 = self.get_flag_c();
                                let left: u8 = self.registers.a;
                                let right: u8 = self.get_r8(r8);
                                self.registers.a += right + c_save;
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(((left & 0xF) + (right & 0xF) + c_save) > 0xF);
                                self.set_flag_c(
                                    ((left as u16) + (right as u16) + (c_save as u16)) > 0xFF,
                                );
                                Some(1)
                            }
                            2 => {
                                // SUB A, r8
                                let r8: u8 = opcode & 0b111;
                                let left: u8 = self.registers.a;
                                let right: u8 = self.get_r8(r8);
                                self.registers.a -= right;
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(true);
                                self.set_flag_h(((left & 0xF) - (right & 0xF)) > 0xF);
                                self.set_flag_c(((left as u16) - (right as u16)) > 0xFF);
                                Some(1)
                            }
                            3 => {
                                let r8: u8 = opcode & 0b111;
                                let c_save: u8 = self.get_flag_c();
                                let left: u8 = self.registers.a;
                                let right: u8 = self.get_r8(r8);
                                self.registers.a = left - right - c_save;
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(true);
                                self.set_flag_h(((left & 0xF) - (right & 0xF) - c_save) > 0xF);
                                self.set_flag_c(
                                    ((left as u16) - (right as u16) - (c_save as u16)) > 0xFF,
                                );
                                Some(1)
                            }
                            4 => {
                                // AND r
                                self.registers.a &= self.get_r8(opcode & 0b111);
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(true);
                                self.set_flag_c(false);
                                Some(1)
                            }
                            5 => {
                                // XOR r
                                self.registers.a ^= self.get_r8(opcode & 0b111);
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_c(false);
                                Some(1)
                            }
                            6 => {
                                // OR r
                                self.registers.a |= self.get_r8(opcode & 0b111);
                                self.set_flag_z(self.registers.a == 0);
                                self.set_flag_n(false);
                                self.set_flag_h(false);
                                self.set_flag_c(false);
                                Some(1)
                            }
                            7 => {
                                // CP r
                                let r8: u8 = opcode & 0b111;
                                let left: u8 = self.registers.a;
                                let right: u8 = self.get_r8(r8);
                                let result = left - right;
                                self.set_flag_z(result == 0);
                                self.set_flag_n(true);
                                self.set_flag_h(((left & 0xF) - (right & 0xF)) > 0xF);
                                self.set_flag_c(((left as u16) - (right as u16)) > 0xFF);
                                Some(1)
                            }
                            _ => None,
                        }
                    }
                    0b11 => {
                        match opcode & 0b1111 {
                            0b0001 => {
                                // POP r16
                                let r16 = (opcode >> 4) & 0b11;
                                let lsb = self.memory.read(self.registers.sp);
                                self.registers.sp += 1;
                                let msb = self.memory.read(self.registers.sp);
                                self.registers.sp += 1;
                                self.set_r16_group_3(r16, util::unsigned_16(msb, lsb));
                                Some(3)
                            }
                            _ => {
                                match opcode & 0b11100111 {
                                    0b110_00_000 => {
                                        // RET conditional
                                        let condition;
                                        match (opcode >> 3) & 0b11 {
                                            0 => condition = self.get_flag_z() == 0,
                                            1 => condition = self.get_flag_z() != 0,
                                            2 => condition = self.get_flag_c() == 0,
                                            3 => condition = self.get_flag_c() != 0,
                                            _ => panic!("not possible condition - RET conditional"),
                                        }
                                        if condition {
                                            let lsb = self.memory.read(self.registers.sp);
                                            self.registers.sp += 1;
                                            let msb = self.memory.read(self.registers.sp);
                                            self.registers.sp += 1;
                                            self.registers.pc = util::unsigned_16(msb, lsb);
                                            Some(5)
                                        } else {
                                            Some(2)
                                        }
                                    }
                                    _ => {
                                        match opcode & 0b11_000_111 {
                                            0b11_000_110 => {
                                                // ALU A, u8
                                                match (opcode >> 3) & 0b111 {
                                                    5 => {
                                                        // XOR
                                                        self.registers.a ^=
                                                            self.memory.read(self.registers.pc);
                                                        self.registers.pc += 1;
                                                        self.set_flag_z(self.registers.a == 0);
                                                        self.set_flag_n(false);
                                                        self.set_flag_h(false);
                                                        self.set_flag_c(false);
                                                        Some(2)
                                                    }
                                                    7 => {
                                                        // CP
                                                        let left: u8 = self.registers.a;
                                                        let right: u8 =
                                                            self.memory.read(self.registers.pc);
                                                        self.registers.pc += 1;
                                                        let result: u8 = left - right;
                                                        self.set_flag_z(result == 0);
                                                        self.set_flag_n(true);
                                                        self.set_flag_h(
                                                            ((left & 0xF) - (right & 0xF)) > 0xF,
                                                        );
                                                        self.set_flag_c(
                                                            ((left as u16) - (right as u16)) > 0xFF,
                                                        );
                                                        Some(1)
                                                    }
                                                    _ => None,
                                                }
                                            }
                                            0b11_000_111 => {
                                                // RST
                                                let exp = opcode & 0b00_111_000;
                                                self.registers.sp -= 1;
                                                self.memory.write(
                                                    self.registers.sp,
                                                    util::msb(self.registers.pc),
                                                );
                                                self.registers.sp -= 1;
                                                self.memory.write(
                                                    self.registers.sp,
                                                    util::lsb(self.registers.pc),
                                                );
                                                self.registers.pc = util::unsigned_16(0x00, exp);
                                                Some(4)
                                            }
                                            _ => None,
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    fn get_hl(&self) -> u16 {
        util::unsigned_16(self.registers.h, self.registers.l)
    }

    fn set_hl(&mut self, value: u16) {
        self.registers.h = ((value & 0xFF00) >> 8) as u8;
        self.registers.l = value as u8;
    }

    fn get_bc(&self) -> u16 {
        util::unsigned_16(self.registers.b, self.registers.c)
    }

    fn set_bc(&mut self, value: u16) {
        self.registers.b = ((value & 0xFF00) >> 8) as u8;
        self.registers.c = value as u8;
    }

    fn get_de(&self) -> u16 {
        util::unsigned_16(self.registers.d, self.registers.e)
    }

    fn set_de(&mut self, value: u16) {
        self.registers.d = ((value & 0xFF00) >> 8) as u8;
        self.registers.e = value as u8;
    }

    fn set_af(&mut self, value: u16) {
        self.registers.a = ((value & 0xFF00) >> 8) as u8;
        self.registers.f = value as u8;
    }

    fn set_r8(&mut self, r8: u8, data: u8) {
        match r8 {
            0 => self.registers.b = data,
            1 => self.registers.c = data,
            2 => self.registers.d = data,
            3 => self.registers.e = data,
            4 => self.registers.h = data,
            5 => self.registers.l = data,
            6 => self.memory.write(self.get_hl(), data),
            7 => self.registers.a = data,
            _ => (),
        }
    }

    fn get_r8(&self, value: u8) -> u8 {
        match value & 0b111 {
            0 => self.registers.b,
            1 => self.registers.c,
            2 => self.registers.d,
            3 => self.registers.e,
            4 => self.registers.h,
            5 => self.registers.l,
            6 => self.memory.read(self.get_hl()),
            7 => self.registers.a,
            _ => 0, // This isn't mathematically possible but I need to return something for rust to not moan
        }
    }
    fn get_r16_group_2(&mut self, r16: u8) -> u16 {
        match r16 {
            0 => self.get_bc(), //TODO implement me
            1 => self.get_de(), // TODO implement me
            2 => {
                let ret = self.get_hl();
                self.set_hl(ret + 1);
                ret
            }
            3 => {
                let ret = self.get_hl();
                self.set_hl(ret - 1);
                ret
            }
            _ => panic!("get_r16_group_2 recieved illegal value"),
        }
    }

    fn get_r16_group_1(&mut self, r16: u8) -> u16 {
        match r16 {
            0 => self.get_bc(),
            1 => self.get_de(),
            2 => self.get_hl(),
            3 => self.registers.sp,
            _ => panic!("get_r16_group_1 recieved illegal value"),
        }
    }

    fn set_r16_group_1(&mut self, r16: u8, value: u16) {
        match r16 {
            0 => self.set_bc(value),
            1 => self.set_de(value),
            2 => self.set_hl(value),
            3 => self.registers.sp = value,
            _ => panic!("get_r16_group_2 recieved illegal value"),
        }
    }

    fn set_r16_group_3(&mut self, r16: u8, value: u16) {
        match r16 {
            0 => self.set_bc(value),
            1 => self.set_de(value),
            2 => self.set_hl(value),
            3 => self.set_af(value),
            _ => panic!("get_r16_group_2 recieved illegal value"),
        }
    }
}
