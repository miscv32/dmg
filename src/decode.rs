use crate::cpu;
use crate::cpu::CPUStage;
use crate::ram;
use crate::util;
pub enum InstructionName {
    Nop,
    LdPtrU16Sp,
    Stop,
    JrUnconditional,
    Halt,
    LdhPtrU8A,
    AddSpI8,
    LdhAPtrU8,
    LdHlSpI8,
    LdhPtrCA,
    LdPtrU16A,
    LdhAPtrC,
    LdAPtrU16,
    CallU16,
}
pub struct Instruction {
    pub _name: InstructionName,
}

pub type MicroOp = fn(&mut cpu::CPU, &mut ram::RAM);

fn fetch_execute_overlap(_cpu: &mut cpu::CPU, _ram: &mut ram::RAM) {
    _cpu.stage = cpu::CPUStage::FetchDecode;
}

fn nop_m2(cpu: &mut cpu::CPU, _ram: &mut ram::RAM) {
    println!("nop_m2");
    cpu.register_file.pc += 1;
}

fn halt_m1(cpu: &mut cpu::CPU, _ram: &mut ram::RAM) {
    println!("halt_m1");
    cpu.register_file.pc += 1;
    cpu.running = false;
}

fn nothing(_cpu: &mut cpu::CPU, _ram: &mut ram::RAM) {

}

fn ld_ptru16_sp_m2(cpu: &mut cpu::CPU, ram: &mut ram::RAM) {
    cpu.register_file.z = ram.read(cpu.register_file.pc).unwrap_or_default();
    cpu.register_file.pc += 1;
}

fn ld_ptru16_sp_m3(cpu: &mut cpu::CPU, ram: &mut ram::RAM) {
    cpu.register_file.w = ram.read(cpu.register_file.pc).unwrap_or_default();
    cpu.register_file.pc += 1;
}

fn ld_ptru16_sp_m4(cpu: &mut cpu::CPU, ram: &mut ram::RAM) {
    let wz = util::_unsigned_16(cpu.register_file.w, cpu.register_file.z);
    let _ = ram._write(util::_least_significant_byte(cpu.register_file._sp), wz);
}

fn ld_ptru16_sp_m5(cpu: &mut cpu::CPU, ram: &mut ram::RAM) {
    let wz = util::_unsigned_16(cpu.register_file.w, cpu.register_file.z) + 1;
    let _ = ram._write(util::_most_significant_byte(cpu.register_file._sp), wz);
    cpu.stage = CPUStage::FetchDecode;
}

fn ld_ptru16_sp_m6(cpu: &mut cpu::CPU, _ram: &mut ram::RAM) {
    cpu.register_file.pc += 1;
}

impl Instruction {
    pub fn micro_ops(&self) -> Result<Vec<MicroOp>, DecodeError> {
        match self._name {
            InstructionName::Nop => return Ok(vec![fetch_execute_overlap, nop_m2]),
            InstructionName::Halt => return Ok(vec![halt_m1]), // halt actually just lasts indefinitely
            InstructionName::LdPtrU16Sp => return Ok(vec![nothing,ld_ptru16_sp_m2,ld_ptru16_sp_m3,ld_ptru16_sp_m4,ld_ptru16_sp_m5, ld_ptru16_sp_m6]),
            _ => return Err(DecodeError::UnimplementedOpcode),
        }
    }
}

pub enum DecodeError {
    IllegalOpcode,
    UnimplementedOpcode,
}

pub trait Opcode {
    fn decode_instruction(self) -> Result<Instruction, DecodeError>;
}

impl Opcode for u8 {
    fn decode_instruction(self) -> Result<Instruction, DecodeError> {
        match self {
            // check against "constant" opcodes
            0x00 => {
                return Ok(Instruction {
                    _name: InstructionName::Nop,
                });
            }
            0x08 => {
                return Ok(Instruction {
                    _name: InstructionName::LdPtrU16Sp,
                });
            }
            0x10 => {
                return Ok(Instruction {
                    _name: InstructionName::Stop,
                });
            }
            0x18 => {
                return Ok(Instruction {
                    _name: InstructionName::JrUnconditional,
                });
            }
            0x76 => {
                return Ok(Instruction {
                    _name: InstructionName::Halt,
                });
            }
            0xE0 => {
                return Ok(Instruction {
                    _name: InstructionName::LdhAPtrU8,
                });
            }
            0xE2 => {
                return Ok(Instruction {
                    _name: InstructionName::LdhPtrCA,
                });
            }
            0xE8 => {
                return Ok(Instruction {
                    _name: InstructionName::LdhPtrU8A,
                });
            }
            0xEA => {
                return Ok(Instruction {
                    _name: InstructionName::LdPtrU16A,
                });
            }
            0xF0 => {
                return Ok(Instruction {
                    _name: InstructionName::AddSpI8,
                });
            }
            0xF2 => {
                return Ok(Instruction {
                    _name: InstructionName::LdhAPtrC,
                });
            }
            0xF8 => {
                return Ok(Instruction {
                    _name: InstructionName::LdHlSpI8,
                });
            }
            0xFA => {
                return Ok(Instruction {
                    _name: InstructionName::LdAPtrU16,
                });
            }
            0xCD => {
                return Ok(Instruction {
                    _name: InstructionName::CallU16,
                });
            }
            _ => {
                // TODO match opcodes which take arguments

                // match illegal opcodes
                match self {
                    0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xEB | 0xEC | 0xFC | 0xDD | 0xED | 0xFD => {
                        return Err(DecodeError::IllegalOpcode);
                    }
                    _ => (),
                }
                // Otherwise the opcode must be unimplemented
                return Err(DecodeError::UnimplementedOpcode);
            }
        };
    }
}
