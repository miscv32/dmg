use crate::ram;
use crate::decode;
use crate::decode::Opcode;

pub struct RegisterFile {
    pub _af: u16,
    pub _bc: u16,
    pub _de: u16,
    pub _hl: u16,
    pub _sp: u16,
    pub pc: u16,
    pub _ime: bool,
}

pub struct Flags {  
// In hardware, flags are actually stored in the upper 4 bits of register F.
// It's easier for us to store them seperately, and update the actual F register lazily.
    pub _carry: bool,
    pub _half_carry: bool,
    pub _subtraction: bool,
    pub _zero: bool,
}

#[derive(PartialEq, Eq)]
pub enum CPUStage {
    FetchDecode,
    Execute,
}

pub struct CPU {
    pub register_file: RegisterFile,
    pub _flags: Flags,
    pub running: bool,
    pub stage: CPUStage,
    pub current_instruction: decode::Instruction,
}    


pub fn init() -> CPU {
// Return an initialised CPU struct
    let register_file: RegisterFile = RegisterFile {
        _af: 0,
        _bc: 0,
        _de: 0,
        _hl: 0,
        _sp: 0,
        pc: 0,
        _ime: false
    };

    let flags: Flags = Flags {
        _carry: false,
        _half_carry: false,
        _subtraction: false,
        _zero: false,
    };
    
    return CPU {
        register_file: register_file,
        _flags: flags,
        running: true,
        stage: CPUStage::FetchDecode,
        current_instruction: decode::Instruction {_name: decode::InstructionName::Nop},
    };
    
}

pub enum CPUError {
    IllegalFetch,
    IllegalOpcode,
    UnimplementedOpcode,
}

impl CPU {
    pub fn tick (&mut self, ram: &mut ram::RAM) -> Result<(), CPUError> {
    // Step CPU / RAM by one M-cycle.
        match self.stage {
            // Fetch/Decode state
            CPUStage::FetchDecode => {
                // Fetch and decode next instruction
                let opcode: u8;

                match ram.read(self.register_file.pc) {
                    Ok(byte) => opcode = byte,
                    Err(ram::ReadError::AddressDoesNotExist) => {
                        println!("Illegal fetch: address does not exist");
                        return Err(CPUError::IllegalFetch)
                    }
                    Err(ram::ReadError::AddressUnreadable) => {
                        println!("Illegal fetch: address exists but cannot be read");
                        return Err(CPUError::IllegalFetch)
                    },
                }

                match opcode.decode_instruction() {
                    Ok(instruction) => self.current_instruction = instruction, // TODO push all operations to queue
                    Err(decode::DecodeError::IllegalOpcode) => {
                        println!("Illegal opcode encountered");
                        return Err(CPUError::IllegalOpcode)
                    },
                    Err(decode::DecodeError::UnimplementedOpcode) => {
                        println!("Unimplemented opcode encountered");
                        return Err(CPUError::UnimplementedOpcode)
                    }
                }

                // TODO F/D/E overlap: Execute anything left on queue 
                // will simulates fetch/execute overlap, 
                // one micro-op should be to trigger a fetch-execute before the last m cycle of an instruction, 
                // so that the F/D stage of the next instruction overlaps with the last mcycle of the previous instruction. 
                // see gbctr

                // Start executing
                self.stage = CPUStage::Execute;
            },
            // TODO implement Execute stage and micro op queue
            CPUStage::Execute => {
                // pop a micro op from the queue and execute
                // if the operation errors then handle it.
            }
        }

        return Ok(());
    }
}