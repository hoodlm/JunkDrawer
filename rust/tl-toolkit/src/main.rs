use log::{info};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::env;
use std::fs;
use crate::datafile::validate_file;

mod datafile;

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
