use crate::byteutil::*;
use log::{debug, info};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct DataFile {
    pub filename: String,
    pub blocks: Vec<RawDataBlock>,
}

pub struct RawDataBlock {
    pub block_number: u16,
    pub start_address: usize,
    pub data_bytes: Vec<u8>,
}

impl RawDataBlock {
    pub fn peek_u32(&self, addr: usize) -> Result<u32, String> {
        u32_at_addr(&self.data_bytes, addr)
    }

    pub fn peek_u16(&self, addr: usize) -> Result<u16, String> {
        u16_at_addr(&self.data_bytes, addr)
    }

    pub fn peek_string(&self, addr: usize, len: usize) -> Result<String, String> {
        string_at_addr(&self.data_bytes, addr, len)
    }
}

pub struct DataFileParser {
    filepath: PathBuf,
}

impl DataFileParser {
    pub fn new(filepath: PathBuf) -> Self {
        Self { filepath }
    }

    pub fn parse_file(&self) -> Result<DataFile, String> {
        let filepath = &self.filepath;
        let filename = filepath
            .file_name()
            .expect(&format!(
                "filepath {filepath:?} should have at least one component"
            ))
            .to_str()
            .expect("Filepath {filepath:?} should be valid unicode");
        debug!("Validating file {}", filename);
        let mut file =
            File::open(filepath).map_err(|e| format!("Unable to open {filepath:?}: {e}"))?;
        debug!("Opened file {filename}");
        let metadata = file
            .metadata()
            .map_err(|e| format!("Could not get metadata for {filename}: {e}"))?;
        let md_filesize: usize = metadata
            .len()
            .try_into()
            .expect("Could not convert u64 into usize; maybe running on an unsupported platform?");
        debug!("Allocating vec for reading file {filename}; expecting {md_filesize} bytes");
        let mut file_bytes: Vec<u8> = vec![0; md_filesize];
        let file_bytes_read = file
            .read(&mut file_bytes)
            .map_err(|e| format!("Failed to read contents of file {filename}: {e}"))?;
        assert_eq!(
            md_filesize,
            file_bytes_read.try_into().unwrap(),
            "File size according to file metadata (left) disagrees with actual bytes read (right)"
        );
        debug!("Successfully read file {filename} into memory");

        // The first ~2K bytes include header/manifest data.
        // Some important addresses:
        //
        // Range (hex)   Type       Purpose/Value
        // 00-03         uint32     65536 (file identifier?)
        // 04-07         uint32     total file size
        // 08-13         ?          ?
        // 14-17         uint32     number of blocks
        // 18-1F         ?          ?
        // 20-27         ascii      "LPPALPPA" identifier for DreamFactory data files
        // 28-1FF        ?          zero padding?
        // 200-3FF       ?          ?
        // 400+          uint32     block offsets
        let file_id = u32_at_addr(&file_bytes, 0x00)?;
        if file_id != 65536 {
            return Err(format!("File {filename} malformed: expected exact value 65536 at range 0x00-0x03, actual value {file_id}"));
        }

        let header_file_size: usize = u32_at_addr(&file_bytes, 0x04)?.try_into().unwrap();
        if header_file_size != md_filesize {
            return Err(format!("File {filename} malformed: expected filesize in bytes ({md_filesize}) at range 0x04-0x07, actual value {header_file_size}"));
        }

        // TODO: Parse 0x08-0x13

        let number_blocks: usize = u32_at_addr(&file_bytes, 0x14)?.try_into().unwrap();

        // TODO: Parse 0x18-0x1F

        let lppalppa = string_at_addr(&file_bytes, 0x20, 8)?;
        if lppalppa != "LPPALPPA" {
            return Err(format!("File {filename} malformed? expected exact ASCII string LPPALPPA at range 0x20-0x27, actual value {lppalppa}"));
        }

        let mut block_addrs: Vec<usize> = Vec::new();

        for block_n in 0..number_blocks {
            let pointer: usize = (0x400 + block_n * 4).into();
            let block_offset: usize = u32_at_addr(&file_bytes, pointer)?.try_into().unwrap();
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
            let block_label: usize = u32_at_addr(&file_bytes, *start_addr)?.try_into().unwrap();
            assert_eq!(
                block_n, block_label,
                "First two bytes at 0x{0:x} should match block number {1}, was {2}",
                start_addr, block_n, block_label
            );
            // the next four bytes are the block's size, in bytes
            let block_size: usize = u32_at_addr(&file_bytes, start_addr + 4)?
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
        debug!("Finished validating file {:?}", self.filepath);
        Ok(DataFile {
            filename: filename.to_string(),
            blocks,
        })
    }
}
