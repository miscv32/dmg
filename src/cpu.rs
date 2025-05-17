use std::collections::VecDeque;

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
    pub queue: VecDeque<decode::MicroOp>
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
        queue: VecDeque::from([]),
    };
    
}

#[derive(Debug)]
pub enum CPUError {
    IllegalFetch,
    IllegalOpcode,
    UnimplementedOpcode,
    EmptyMicroOpQueue,
}

impl CPU {
    pub fn tick (&mut self, ram: &mut ram::RAM) -> Result<(), CPUError> {
    // Step CPU / RAM by one M-cycle.
        // if you try to tick while we aren't running do nothing
        if !self.running {
            return Ok(());
        }
        match self.stage {
            // Fetch/Decode state
            CPUStage::FetchDecode => {
                println!("fetch decode stage");

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
                    Ok(instruction) => { 
                        self.current_instruction = instruction;
                        let micro_ops_result: Result<Vec<fn(&mut CPU, &mut ram::RAM)>, decode::DecodeError> = self.current_instruction.micro_ops();
                        let micro_ops: Vec<fn(&mut CPU, &mut ram::RAM)>;
                        match micro_ops_result {
                            Ok(val) => micro_ops = val,
                            Err(_) => return Err(CPUError::UnimplementedOpcode),
                        }

                        for micro_op in micro_ops {
                            self.queue.push_back(micro_op);
                            println!("push to queue")
                        }
                        
                        self.stage = CPUStage::Execute; // note that the stage value may be immediately overwritten by the next micro-operation

                    }, // TODO push all operations to queue
                    Err(decode::DecodeError::IllegalOpcode) => {
                        println!("Illegal opcode encountered");
                        return Err(CPUError::IllegalOpcode)
                    },
                    Err(decode::DecodeError::UnimplementedOpcode) => {
                        println!("Unimplemented opcode encountered");
                        return Err(CPUError::UnimplementedOpcode)
                    }
                }

                // simulate F/D/E overlap: Execute the one micro-op left on queue from the previous operation
                // The penultimate (and no other) micro-op must trigger a fetch-execute, otherwise the pipeline gets messed up.
                if let Some(value) = self.execute_micro_op(ram) {
                    return value;
                }

            },

            CPUStage::Execute => {
                // pop a micro op from the queue and execute
                println!("execute stage");
                if let Some(value) = self.execute_micro_op(ram) {
                    return value;
                }
            }
        }

        return Ok(());
    }

    fn execute_micro_op(&mut self, ram: &mut ram::RAM) -> Option<Result<(), CPUError>> {
        let micro_op_option: Option<fn(&mut CPU, &mut ram::RAM)> = self.queue.pop_front();
        let micro_op_fn_ptr: fn(&mut CPU, &mut ram::RAM);
        match micro_op_option {
            Some(x) => micro_op_fn_ptr = x,
            None => return Some(Err(CPUError::EmptyMicroOpQueue)),
        }
        micro_op_fn_ptr(self, ram);
        None
    }
}