// To change endianness:
// - swap most_significant_byte and least_significant_byte function definitions
// - swap msb and lsb in unsigned_16 function

pub fn msb(two_bytes: u16) -> u8 {
    (two_bytes >> 8) as u8
}

pub fn lsb(two_bytes: u16) -> u8 {
    (two_bytes & 0xFF) as u8
}

pub fn unsigned_16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | lsb as u16
}
