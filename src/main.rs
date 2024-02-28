mod crawler;

use crawler::start_crawler;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use simplelog::*;

fn main() {
    setup_folders().unwrap();
    setup_logs().unwrap();

    let path = "./folders.txt";

    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening folders.txt: {}", error);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut paths: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                paths = line.split(';').map(String::from).collect();
            }
            Err(error) => println!("Error reading line: {}", error),
        }
    }

    for path in paths {
        let npath = path.clone();
        let spawned = thread::spawn(move || start_crawler(&path));
        match spawned.join() {
            Ok(()) => {
                println!("Thread {} finished", npath);
            }
            Err(err) => {
                println!("Thread {} panic!", npath);
                println!("{:?}", err)
            }
        }
    }
}

fn setup_folders() -> Result<(), std::io::Error> {
    if !std::path::Path::new("./output").exists() {
        match fs::create_dir("./output") {
            Ok(()) => {
                println!("Setup output folder");
            }
            Err(err) => {
                println!("Error setting up output folder");
                return Err(err);
            }
        }
    }
    if !std::path::Path::new("./logs").exists() {
        match fs::create_dir("./logs") {
            Ok(()) => {
                println!("Setup logs folder");
            }
            Err(err) => {
                println!("Error setting up logs folder");
                return Err(err);
            }
        }
    }
    Ok(())
}

fn setup_logs() -> Result<(), std::io::Error> {
    // Get timestamp
    let current_time = SystemTime::now();
    let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration_since_epoch.as_secs().to_string();

    let log_file_path = format!("./logs/log_{}.log", timestamp);

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file_path).unwrap(),
        ),
    ])
    .unwrap();

    Ok(())
}
