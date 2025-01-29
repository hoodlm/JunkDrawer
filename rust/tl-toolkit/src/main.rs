use crate::datafile::{DataFile, DataFileParser, RawDataBlock};
use log::{debug, error, info, warn};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::ops::Range;

mod byteutil;
mod datafile;

fn main() {
    log_setup();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut tk = TLToolkit::new();
    help();
    loop {
        let full_command = rl.readline(">> ").unwrap();
        let command_components: Vec<&str> = full_command.split_whitespace().collect();
        let base_command = command_components.get(0).unwrap_or(&"help");
        match *base_command {
            "exit" => break,
            "loadall" => tk.loadall(),
            "help" => help(),
            "extract" => match tk.extract(command_components) {
                    Err(e) => println!("{}", e),
                    Ok(ok) => println!("{}", ok),
            }
            "blockinfo" => {
                match tk.block_info(command_components) {
                    Err(e) => println!("{}", e),
                    Ok(ok) => println!("{}", ok),
                }
            }
            "peek" => {
                match tk.peek(command_components) {
                    Err(e) => println!("{}", e),
                    Ok(ok) => println!("{}", ok),
                }
            },
            "peekall" => {
                match tk.peekall(command_components) {
                    Err(e) => println!("{}", e),
                    Ok(ok) => println!("{}", ok),
                }
            },
            x => println!("Unknown command {x}"),
        }
    }
}

fn help() {
    println!("Supported commands: help, loadall, exit, peek [file] [offset], peekall [offset], blockinfo [blockname], extract");
}

/// Set up global logger.
fn log_setup() {
    let log_conf = ConfigBuilder::new().set_time_format_rfc3339().build();
    let _ = SimpleLogger::init(LevelFilter::Info, log_conf);
}

struct TLToolkit {
    blocks: BTreeMap<String, RawDataBlock>,
}

impl TLToolkit {
    fn new() -> Self {
        Self {
            blocks: BTreeMap::new(),
        }
    }

    fn extract(&self, command_components: Vec<&str>) -> Result<String, String> {
        // Hardcoding for now; will extract from command components later
        let block_id = "I005.STG-0006";
        let block = self.blocks.get(block_id).ok_or(
            format!("{block_id} does not exist or is not loaded yet"))?;
        let start: usize = 0x0c;
        let end: usize = block.data_bytes.len() - (0x0c + 1);
        let data_range: Range<usize> = start..end;
        let bitmap = block.data_bytes.get(data_range).unwrap().to_vec();
        let expanded = byteutil::run_length_decode(&bitmap, 640 * 480)?;
        Ok("not implemented yet".to_string())
    }

    fn peekall(&self, command_components: Vec<&str>) -> Result<String, String> {
        if command_components.len() != 2 {
            return Err("Usage: peekall [offset] - for example, peekall 128".to_string());
        }
        let offset = command_components.get(1).unwrap();
        let offset: usize = offset.parse().map_err(|e| format!("Offset argument {offset} is invalid (should be a positive integer): {e}"))?;
        let mut accum: Vec<String> = Vec::new();
        for (block_id, block) in &self.blocks {
            let result = self.peek_internal(block_id, block, offset)?;
            accum.push(result);
        }
        Ok(accum.join("\n"))
    }

    fn block_info(&self, command_components: Vec<&str>) -> Result<String, String> {
        if command_components.len() != 2 {
            return Err("Usage: blockinfo [block] - for example, blockinfo I008.STG-0001".to_string());
        }
        let block_id: &str = command_components.get(1).unwrap();
        let block = self.blocks.get(block_id);
        if let Some(block) = block {
            Ok(format!("{block_id}: {0} bytes, starts at addr 0x{1:x}", block.data_bytes.len(), block.start_address))
        } else {
            Err(format!("{block_id} does not exist or is not loaded yet"))
        }
    }

    fn peek(&self, command_components: Vec<&str>) -> Result<String, String> {
        if command_components.len() != 3 {
            return Err("Usage: peek [block] [offset] - for example, peek I008.STG-0001 256".to_string());
        }
        let block_id: &str = command_components.get(1).unwrap();
        let offset = command_components.get(2).unwrap();
        let offset: usize = offset.parse().map_err(|e| format!("Offset argument {offset} is invalid (should be a positive integer): {e}"))?;
        let block = self.blocks.get(block_id);
        if block.is_none() {
            Err(format!("{block_id} does not exist or is not loaded yet"))
        } else {
            self.peek_internal(block_id, block.unwrap(), offset)
        }
    }

    fn peek_internal(&self, block_id: &str, block: &RawDataBlock, offset: usize) -> Result<String, String> {
        let data = block.peek_u32(offset);
        match data {
            Ok(data) => {
                Ok(format!("{block_id} at 0x{offset:x}: 0x{data:x} {data}"))
            }
            Err(e) => {
                Err(format!("{block_id} at 0x{offset:x} could not be read: {e}"))
            }
        }
    }

    fn loadall(&mut self) {
        info!("Clearing existing cache before reloading");
        self.blocks.clear();
        match self.loadall_internal() {
            Ok(files) => {
                let count = files.len();
                if let Err(e) = self.cache_files(files) {
                    warn!("Failed to insert data into block cache: {e}");
                } else {
                    info!("Successfully loaded and cached {count} files");
                }
            }
            Err(e) => {
                error!("loadall failed: {e}");
            }
        }
    }

    fn loadall_internal(&mut self) -> Result<Vec<DataFile>, String> {
        let home = env::var("HOME").unwrap();
        let game_directory = format!("{home}/.steam/steam/steamapps/common/Timelapse");
        let mut files: Vec<DataFile> = Vec::new();
        for cd in vec!["LOCAL", "I", "E", "M", "A", "Z"] {
            let cd_directory = format!("{game_directory}/{cd}");
            for gamefile in fs::read_dir(cd_directory).map_err(|it| it.to_string())? {
                let gamefile_path = gamefile.map_err(|it| it.to_string())?.path();
                let parser = DataFileParser::new(gamefile_path);
                let file = parser.parse_file()?;
                info!(
                    "Parsed file {0}, {1} blocks",
                    file.filename,
                    file.blocks.len()
                );
                files.push(file);
            }
        }
        info!("Done; validated {} files", files.len());
        Ok(files)
    }

    fn cache_files(&mut self, files: Vec<DataFile>) -> Result<(), String> {
        for file in files {
            self.cache_blocks(&file.filename, file.blocks)?;
        }
        Ok(())
    }

    fn cache_blocks(&mut self, filename: &str, blocks: Vec<RawDataBlock>) -> Result<(), String> {
        for block in blocks {
            let block_id = format!("{}-{:04}", filename, block.block_number);
            self.blocks.insert(block_id.clone(), block);
            debug!("Cached {block_id}");
        }
        Ok(())
    }
}
