use log::{debug, info};
use std::fs::File;
use std::io::Read;
use std::ops::Range;

pub struct DataFile {
    pub filename: String,
    pub blocks: Vec<RawDataBlock>,
}

pub struct RawDataBlock {
    block_number: u16,
    start_address: usize,
    data_bytes: Vec<u8>,
}

pub struct DataFileParser {
    filepath: String,
}

impl DataFileParser {
    pub fn new(filepath: &str) -> Self {
        Self {
            filepath: filepath.to_string(),
        }
    }

    fn u16_at_addr(&self, buffer: &Vec<u8>, addr: usize) -> Result<u16, String> {
        let range: Range<usize> = addr..addr + 2;
        let bytes: [u8; 2] = buffer.get(range).unwrap().try_into().unwrap();
        let result: u16 = u16::from_le_bytes(bytes)
            .try_into()
            .map_err(|e| format!("Two bytes at addr 0x{addr:x} are not a valid u16: {e}"))?;
        Ok(result)
    }

    fn u32_at_addr(&self, buffer: &Vec<u8>, addr: usize) -> Result<u32, String> {
        let range: Range<usize> = addr..addr + 4;
        let bytes: [u8; 4] = buffer.get(range).unwrap().try_into().unwrap();
        let result: u32 = u32::from_le_bytes(bytes)
            .try_into()
            .map_err(|e| format!("Two bytes at addr 0x{addr:x} are not a valid u32: {e}"))?;
        Ok(result)
    }

    fn string_at_addr(&self, buffer: &Vec<u8>, addr: usize, len: usize) -> Result<String, String> {
        let end: usize = addr + len;
        let range: Range<usize> = addr..end;
        let bytes: Vec<u8> = buffer.get(range).unwrap().to_vec();
        let s = String::from_utf8(bytes)
            .map_err(|e| format!("Bytes in range 0x{addr:x}-0x{end:x} are not valid utf8: {e}"))?;
        Ok(s)
    }

    pub fn parse_file(&self) -> Result<DataFile, String> {
        let filepath = &self.filepath;
        debug!("Validating file {}", filepath);
        let mut file =
            File::open(filepath).map_err(|e| format!("Unable to open {filepath}: {e}"))?;
        debug!("Opened file {filepath}");
        let metadata = file
            .metadata()
            .map_err(|e| format!("Could not get metadata for {filepath}: {e}"))?;
        let md_filesize: usize = metadata
            .len()
            .try_into()
            .expect("Could not convert u64 into usize; maybe running on an unsupported platform?");
        debug!("Allocating vec for reading file {filepath}; expecting {md_filesize} bytes");
        let mut file_bytes: Vec<u8> = vec![0; md_filesize];
        let file_bytes_read = file
            .read(&mut file_bytes)
            .map_err(|e| format!("Failed to read contents of file {filepath}: {e}"))?;
        assert_eq!(
            md_filesize,
            file_bytes_read.try_into().unwrap(),
            "File size according to file metadata (left) disagrees with actual bytes read (right)"
        );
        debug!("Successfully read file {filepath} into memory");

        // The first ~2K bytes include header/manifest data.
        // Some important addresses:
        //
        // Range (hex)   Type       Purpose/Value
        // 00-03         uint32     65536 (file identifier?)
        // 04-07         uint32     total file size
        // 08-13         ?          ?
        // 14-15         uint16     number of blocks
        // 16-17         ?          zero padding?
        // 18-1F         ?          ?
        // 20-27         ascii      "LPPALPPA" identifier for DreamFactory data files
        // 28-1FF        ?          zero padding?
        // 200-3FF       ?          ?
        // 400+          uint32     block offsets
        let file_id = self.u32_at_addr(&file_bytes, 0x00)?;
        if file_id != 65536 {
            return Err(format!("File {filepath} malformed: expected exact value 65536 at range 0x00-0x03, actual value {file_id}"));
        }

        let header_file_size: usize = self.u32_at_addr(&file_bytes, 0x04)?.try_into().unwrap();
        if header_file_size != md_filesize {
            return Err(format!("File {filepath} malformed: expected filesize in bytes ({md_filesize}) at range 0x04-0x07, actual value {header_file_size}"));
        }

        // TODO: Parse 0x08-0x13

        let number_blocks = self.u16_at_addr(&file_bytes, 0x14)?;

        // TODO: Parse zero-padding at 0x16, or parse number_blocks as a u32?
        // TODO: Parse 0x18-0x1F

        let lppalppa = self.string_at_addr(&file_bytes, 0x20, 8)?;
        if lppalppa != "LPPALPPA" {
            return Err(format!("File {filepath} malformed? expected exact ASCII string LPPALPPA at range 0x20-0x27, actual value {lppalppa}"));
        }

        let mut block_addrs: Vec<usize> = Vec::new();

        for block_n in 0..number_blocks {
            let pointer: usize = (0x400 + block_n * 4).into();
            let block_offset: usize = self.u32_at_addr(&file_bytes, pointer)?.try_into().unwrap();
            assert!(
                block_offset < md_filesize,
                "Unexpected: block {block_n} starts at address 0x{block_offset:x}, which is after file ends at 0x{md_filesize:x}",
            );
            block_addrs.push(block_offset.try_into().unwrap());
        }

        assert_eq!(block_addrs.len(), number_blocks.into());

        let mut blocks: Vec<RawDataBlock> = Vec::new();
        for (block_n, start_addr) in block_addrs.iter().enumerate() {
            debug!("Reading block {}, staring at {}", block_n, start_addr);

            // the first four bytes of every block are labeled with the block's number
            let block_label: usize = self
                .u32_at_addr(&file_bytes, *start_addr)?
                .try_into()
                .unwrap();
            assert_eq!(
                block_n, block_label,
                "First two bytes at 0x{0:x} should match block number {1}, was {2}",
                start_addr, block_n, block_label
            );
            // the next four bytes are the block's size, in bytes
            let block_size: usize = self
                .u32_at_addr(&file_bytes, start_addr + 4)?
                .try_into()
                .unwrap();

            let data_start_addr = start_addr + 8;
            let data_end_addr = data_start_addr + block_size;
            debug!("Block {block_n} is block_size {block_size}, data is from 0x{data_start_addr:x} to 0x{data_end_addr:x}");

            let block_data: Vec<u8> =
                Vec::from(file_bytes.get(data_start_addr..data_end_addr).unwrap());
            let block = RawDataBlock {
                block_number: block_n.try_into().unwrap(),
                start_address: *start_addr,
                data_bytes: block_data,
            };
            blocks.push(block);
        }
        debug!("Finished validating file {}", self.filepath);
        Ok(DataFile {
            filename: self.filepath.clone(),
            blocks: blocks,
        })
    }
}
