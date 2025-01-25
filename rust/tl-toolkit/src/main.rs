use crate::datafile::DataFileParser;
use log::{error, info};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::env;
use std::fs;

mod datafile;

fn main() {
    log_setup();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    help();
    loop {
        let command = rl.readline(">> ").unwrap();
        match command.trim() {
            "exit" => break,
            "loadall" => loadall(),
            "help" => help(),
            x => println!("Unknown command {x}"),
        }
    }
}

fn help() {
    println!("Supported commands: help, loadall, exit");
}

fn loadall() {
    match loadall_internal() {
        Ok(_) => {}
        Err(e) => {
            error!("loadall failed: {e}");
        }
    }
}

fn loadall_internal() -> Result<(), String> {
    let home = env::var("HOME").unwrap();
    let game_directory = format!("{home}/.steam/steam/steamapps/common/Timelapse");
    let mut count = 0;
    for cd in vec!["LOCAL", "I", "E", "M", "A", "Z"] {
        let cd_directory = format!("{game_directory}/{cd}");
        for gamefile in fs::read_dir(cd_directory).map_err(|it| it.to_string())? {
            let gamefile_path = gamefile.map_err(|it| it.to_string())?.path();
            let gamefile_path_str = gamefile_path
                .as_path()
                .to_str()
                .expect("Unexpected: path for {gamefile} is not valid UTF-8");
            let parser = DataFileParser::new(&gamefile_path_str);
            let file = parser.parse_file()?;
            info!(
                "Parsed file {0}, {1} blocks",
                file.filename,
                file.blocks.len()
            );
            count = count + 1;
        }
    }
    info!("Done; validated {count} files");
    Ok(())
}

/// Set up global logger.
fn log_setup() {
    let log_conf = ConfigBuilder::new().set_time_format_rfc3339().build();
    let _ = SimpleLogger::init(LevelFilter::Info, log_conf);
}
