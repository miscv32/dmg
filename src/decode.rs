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
            0x00 => return Ok(Instruction { _name: InstructionName::Nop }),
            0x08 => return Ok(Instruction { _name: InstructionName::LdPtrU16Sp }),
            0x10 => return Ok(Instruction { _name: InstructionName::Stop }),
            0x18 => return Ok(Instruction { _name: InstructionName::JrUnconditional }),
            0x76 => return Ok(Instruction { _name: InstructionName::Halt }),
            0xE0 => return Ok(Instruction { _name: InstructionName::LdhAPtrU8 }),
            0xE2 => return Ok(Instruction { _name: InstructionName::LdhPtrCA }),
            0xE8 => return Ok(Instruction { _name: InstructionName::LdhPtrU8A }),
            0xEA => return Ok(Instruction { _name: InstructionName::LdPtrU16A }),
            0xF0 => return Ok(Instruction { _name: InstructionName::AddSpI8 }),
            0xF2 => return Ok(Instruction { _name: InstructionName::LdhAPtrC }),
            0xF8 => return Ok(Instruction { _name: InstructionName::LdHlSpI8 }),
            0xFA => return Ok(Instruction { _name: InstructionName::LdAPtrU16 }),
            0xCD => return Ok(Instruction { _name: InstructionName::CallU16 }),
            _ => {
                // TODO match opcodes which take arguments

                // match illegal opcodes
                match self {
                    0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xEB | 0xEC | 0xFC | 0xDD | 0xED | 0xFD => return Err(DecodeError::IllegalOpcode),
                    _ => (),
                }
                // Otherwise the opcode must be unimplemented
                return Err(DecodeError::UnimplementedOpcode);
            },
        };
    }
}