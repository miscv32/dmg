// For now, we just use an array containing all memory locations
// When we get to MBCs this should be more useful

type RAMValue = u8; 
type RAMAddress = u16;
const RAM_SIZE: usize = 0xFFFF;

pub struct RAM {
    buf: [RAMValue; RAM_SIZE],
}

#[derive(Debug)]
pub enum _WriteError {
    AddressDoesNotExist,
    AddressUnwriteable, // address exists but cannot be written to
}

#[derive(Debug)]
pub enum ReadError {
    AddressDoesNotExist,
    AddressUnreadable, // address exists but cannot be read from
}

impl RAM {
    pub fn _write(&mut self, value: RAMValue, address: RAMAddress) -> Result<(), _WriteError> {
        
        if address >= RAM_SIZE as RAMAddress{
            return Err(_WriteError::AddressDoesNotExist);
        }

        if 0xE000 <= address && address <= 0xFDFF { 
        // Echo RAM
            if cfg!(feature = "enable_echo_ram_emulation") { 
                return self._write(value, address - 0xC000); 
            } else {
                return Err(_WriteError::AddressUnwriteable);
            }
        }

        if 0xFEA0 <= address && address <= 0xFEFF { 
        // Prohibited area. 
        // TODO simulate DMG OAM corruption in instruction logic 
        // https://gbdev.io/pandocs/OAM_Corruption_Bug.html#oam-corruption-bug
            if cfg!(feature = "enable_FEA0_FEFF_range_emulation") {
                return Ok(()); 
            } else {
                return Err(_WriteError::AddressUnwriteable);
            }
        }

        self.buf[address as usize] = value;
        return Ok(());
    }

    pub fn read(&self, address: RAMAddress) -> Result<RAMValue, ReadError> {
        if address >= RAM_SIZE as RAMAddress {
            return Err(ReadError::AddressDoesNotExist);
        }
        if 0xE000 <= address && address <= 0xFDFF { 
        // Echo RAM
            if cfg!(feature = "enable_echo_ram_emulation") {
                return self.read(address - 0xC000);
            } else {
                println!("RAM read error: cannot read from echo RAM when echo RAM emulation is disabled");
                return Err(ReadError::AddressUnreadable);
            }
        }

        if 0xFEA0 <= address && address <= 0xFEFF { 
        // Prohibited area.
            if cfg!(feature = "enable_FEA0_FEFF_range_emulation") {
                return Ok(0); 
            } else {
                println!("RAM read error: cannot read from the 0xFEAO - 0xFEFF range when its emulation is disabled.");
                return Err(ReadError::AddressUnreadable);
            }
        }

        return Ok(self.buf[address as usize]);     
    }
}

pub fn init() -> RAM {
    return RAM {
        buf: [0; RAM_SIZE],
    };
}