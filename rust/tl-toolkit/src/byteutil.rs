use std::ops::Range;
use log::info;

pub fn u16_at_addr(buffer: &Vec<u8>, addr: usize) -> Result<u16, String> {
    let range: Range<usize> = addr..addr + 2;
    let bytes: [u8; 2] = buffer
        .get(range)
        .ok_or(format!(
            "There are not 2 bytes from address {addr} - buffer is only {0} bytes",
            buffer.len()
        ))?
        .try_into()
        .unwrap();
    let result: u16 = u16::from_le_bytes(bytes)
        .try_into()
        .map_err(|e| format!("Two bytes at addr 0x{addr:x} are not a valid u16: {e}"))?;
    Ok(result)
}

pub fn u32_at_addr(buffer: &Vec<u8>, addr: usize) -> Result<u32, String> {
    let range: Range<usize> = addr..addr + 4;
    let bytes: [u8; 4] = buffer
        .get(range)
        .ok_or(format!(
            "There are not 4 bytes from address {addr} - buffer is only {0} bytes",
            buffer.len()
        ))?
        .try_into()
        .unwrap();
    let result: u32 = u32::from_le_bytes(bytes)
        .try_into()
        .map_err(|e| format!("Two bytes at addr 0x{addr:x} are not a valid u32: {e}"))?;
    Ok(result)
}

pub fn string_at_addr(buffer: &Vec<u8>, addr: usize, len: usize) -> Result<String, String> {
    let end: usize = addr + len;
    let range: Range<usize> = addr..end;
    let bytes: Vec<u8> = buffer
        .get(range)
        .ok_or(format!(
            "There are not {len} bytes from address {addr} - buffer is only {0} bytes",
            buffer.len()
        ))?
        .to_vec();
    let s = String::from_utf8(bytes)
        .map_err(|e| format!("Bytes in range 0x{addr:x}-0x{end:x} are not valid utf8: {e}"))?;
    Ok(s)
}

pub fn run_length_decode(compressed: &Vec<u8>, max_bytes: usize) -> Result<Vec<u8>, String> {
    info!("Executing run length decoding on compressed bitmap ({0} bytes), expanding up to {max_bytes} bytes", compressed.len());
    let mut expanded: Vec<u8> = Vec::with_capacity(max_bytes);
    // 0 means that the decoder is in "run_length" mode,
    // any value greater than 0 is "absolute" mode.
    // "Absolute mode" means a region of the compressed bitmap that is not actually
    // compressed; that is, we read N bytes verbatim.
    let mut absolute_count: u8 = 0;
    // Read bytes in pairs. This could be better optimized (especially when in absolute mode).
    for bytes in compressed.chunks(2) {
        if expanded.len() >= max_bytes {
            break;
        }
        // TODO: Is it safe to assume that bitmaps are always word-aligned? this will panic if we
        // have an odd number of bytes.
        let b1 = bytes.get(0).unwrap();
        let b2 = bytes.get(1).unwrap();
        // basic case for absolute mode; push the two bytes as they are into the expanded bitmap.
        if absolute_count >= 2 {
            expanded.push(*b1);
            expanded.push(*b2);
            absolute_count = absolute_count.saturating_sub(2);
            if absolute_count == 0 {
                info!("RLE: done with absolute mode (absolute_count={absolute_count})");
            }
        // edge case where absolute mode is an odd number. We ignore b2 (it is padding) and just
        // take b1.
        } else if absolute_count == 1 {
            expanded.push(*b1);
            absolute_count = absolute_count.saturating_sub(1);
            if absolute_count == 0 {
                info!("RLE: done with absolute mode (absolute_count={absolute_count})");
            }
        // basic case when the decoder is NOT in absolute mode (that is, we're doing run length
        // decoding)
        } else {
            // b2 set to 0x00 is our signal to switch to absolute mode.
            if *b2 == 0x00 {
                // In this case, b1 tells us the number of bytes to read verbatim.
                absolute_count = u8::from_le_bytes([*b1]);
                info!("RLE: shifting to absolute mode (counter: {absolute_count})");
            } else {
                // Finally: plain old run length decoding.
                // b2 tells us the run length, b1 is the byte to pack.
                let run_length: usize = u8::from_le_bytes([*b2]).try_into().unwrap();
                let mut to_append: Vec<u8> = vec![*b1; run_length];
                expanded.append(&mut to_append);
                info!("RLE: run length {run_length} of byte 0x{0:x}", *b1);
            }
        }
    }
    info!("Run length decoding complete: expanded from {0} to {1} bytes", compressed.len(), expanded.len());
    Ok(expanded)
}
