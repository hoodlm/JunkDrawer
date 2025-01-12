use log::{debug, info};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

fn main() {
    log_setup();
    let home = env::var("HOME").unwrap();
    let game_directory = format!("{home}/.steam/steam/steamapps/common/Timelapse");
    let mut count = 0;
    for cd in vec!["LOCAL", "I", "E", "M", "A", "Z"] {
        let cd_directory = format!("{game_directory}/{cd}");
        for gamefile in fs::read_dir(cd_directory).unwrap() {
            let gamefile_path = gamefile.unwrap().path();
            let gamefile_path_str = gamefile_path.as_path().to_str().unwrap();
            validate_file(gamefile_path_str);
            count = count + 1;
        }
    }
    info!("Done; validated {count} files");
}

/// Set up global logger.
fn log_setup() {
    let log_conf = ConfigBuilder::new().set_time_format_rfc3339().build();
    let _ = SimpleLogger::init(LevelFilter::Info, log_conf);
}

fn validate_file(target_file_path: &str) {
    info!("Validating file {}", target_file_path);
    let mut file = File::open(target_file_path).unwrap();
    debug!("Opened file");

    // The first ~2K bytes include header/manifest data.
    // (The block offsets section is variable size depending
    // on the size of the files; from experimentation 50K is enough for
    // the largest files.)
    //
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
    let mut header_bytes: [u8; 50000] = [0; 50000];
    let _ = file.read(&mut header_bytes).unwrap();

    let full_file_size = u32::from_le_bytes(header_bytes[0x04..0x08].try_into().unwrap());
    debug!("Full file size: {0} bytes (0x{0:x})", &full_file_size);

    let number_blocks = u16::from_le_bytes(header_bytes[0x14..0x16].try_into().unwrap());
    debug!("Number of blocks: {}", &number_blocks);

    let lppalppa = String::from_utf8(header_bytes[0x20..0x28].try_into().unwrap()).unwrap();
    debug!("lppalppa? {}", &lppalppa);

    let mut block_addrs: Vec<usize> = Vec::new();

    for block_n in 0..number_blocks {
        let pointer: usize = (0x400 + block_n * 4).into();
        let block_offset_bytes: [u8; 4] = header_bytes.get(pointer..pointer+4)
        .expect(&format!("couldn't get block offset bytes from header bytes (tried to read 4 bytes at 0x{pointer:x})"))
        .try_into()
        .expect("couldn't coerce header bytes into u8 array");
        let block_offset = u32::from_le_bytes(block_offset_bytes);
        // eprintln!("block {} starts at address 0x{1:x}", block_n, block_offset);
        assert!(
            block_offset < full_file_size,
            "block {0} starts at address 0x{1:x}, which is after file ends at 0x{2:x}",
            block_n,
            block_offset,
            full_file_size
        );
        block_addrs.push(block_offset.try_into().unwrap());
    }

    assert_eq!(block_addrs.len(), number_blocks.into());

    debug!("Finished parsing the header; now load the full file into memory");
    let mut file_bytes: Vec<u8> = vec![0; full_file_size.try_into().unwrap()];
    debug!("Initialized file_bytes at {}", file_bytes.len());

    // TODO: don't close & reopen unnecessarily, reuse existing handler
    let mut file = File::open(&target_file_path).unwrap();
    let file_bytes_read = file.read(&mut file_bytes).unwrap();
    assert_eq!(
        full_file_size,
        file_bytes_read.try_into().unwrap(),
        "File size according to header (left) disagrees with actual file size (right)"
    );

    for (block_n, start_addr) in block_addrs.iter().enumerate() {
        debug!("would read {} at {}", block_n, start_addr);

        // the first four bytes of every block are labeled with the block's number
        let block_label_range: Range<usize> = *start_addr..(start_addr + 4);
        let block_label_bytes: [u8; 4] = file_bytes
            .get(block_label_range)
            .unwrap()
            .try_into()
            .unwrap();
        let block_label: usize = u32::from_le_bytes(block_label_bytes).try_into().unwrap();
        assert_eq!(
            block_n, block_label,
            "First two bytes at 0x{0:x} should match block number {1}, was {2}",
            start_addr, block_n, block_label
        );

        // hypothesis: the next four bytes are the block size?
        let block_size_range: Range<usize> = (start_addr + 4)..(start_addr + 8);
        let block_size_bytes: [u8; 4] = file_bytes
            .get(block_size_range)
            .unwrap()
            .try_into()
            .unwrap();
        let block_size: usize = u32::from_le_bytes(block_size_bytes).try_into().unwrap();
        let data_start_addr = start_addr + 8;
        let data_end_addr = data_start_addr + block_size;
        debug!("Block {block_n} is block_size {block_size}, data is from 0x{data_start_addr:x} to 0x{data_end_addr:x}");
    }
    info!("Finished validating file {}", target_file_path);
}
