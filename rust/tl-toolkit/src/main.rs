use crate::datafile::{DataFile, DataFileParser, RawDataBlock};
use log::{debug, error, info, warn};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::collections::HashMap;
use std::env;
use std::fs;

mod datafile;

fn main() {
    log_setup();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut tk = TLToolkit::new();
    help();
    loop {
        let command = rl.readline(">> ").unwrap();
        match command.trim() {
            "exit" => break,
            "loadall" => tk.loadall(),
            "help" => help(),
            x => println!("Unknown command {x}"),
        }
    }
}

fn help() {
    println!("Supported commands: help, loadall, exit");
}

/// Set up global logger.
fn log_setup() {
    let log_conf = ConfigBuilder::new().set_time_format_rfc3339().build();
    let _ = SimpleLogger::init(LevelFilter::Info, log_conf);
}

struct TLToolkit {
    blocks: HashMap<String, RawDataBlock>,
}

impl TLToolkit {
    fn new() -> Self {
        Self {
            blocks: HashMap::new(),
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
            let block_id = format!("{}-{}", filename, block.block_number);
            self.blocks.insert(block_id.clone(), block);
            debug!("Cached {block_id}");
        }
        Ok(())
    }
}
