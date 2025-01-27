use std::ops::Range;

pub fn u16_at_addr(buffer: &Vec<u8>, addr: usize) -> Result<u16, String> {
    let range: Range<usize> = addr..addr + 2;
    let bytes: [u8; 2] = buffer.get(range).unwrap().try_into().unwrap();
    let result: u16 = u16::from_le_bytes(bytes)
        .try_into()
        .map_err(|e| format!("Two bytes at addr 0x{addr:x} are not a valid u16: {e}"))?;
    Ok(result)
}

pub fn u32_at_addr(buffer: &Vec<u8>, addr: usize) -> Result<u32, String> {
    let range: Range<usize> = addr..addr + 4;
    let bytes: [u8; 4] = buffer.get(range).unwrap().try_into().unwrap();
    let result: u32 = u32::from_le_bytes(bytes)
        .try_into()
        .map_err(|e| format!("Two bytes at addr 0x{addr:x} are not a valid u32: {e}"))?;
    Ok(result)
}

pub fn string_at_addr(buffer: &Vec<u8>, addr: usize, len: usize) -> Result<String, String> {
    let end: usize = addr + len;
    let range: Range<usize> = addr..end;
    let bytes: Vec<u8> = buffer.get(range).unwrap().to_vec();
    let s = String::from_utf8(bytes)
        .map_err(|e| format!("Bytes in range 0x{addr:x}-0x{end:x} are not valid utf8: {e}"))?;
    Ok(s)
}
