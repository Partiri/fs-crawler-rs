use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct FileStruc {
    path: String,
    size: u64,
    lmod: u64,
}

#[derive(Serialize)]
struct FilesList {
    files: Vec<FileStruc>,
}

pub fn start_crawler(folder_path: &String) {
    println!("Crawling {}", folder_path);
    log::info!("Crawling {}", folder_path);

    let path = Path::new(folder_path);

    let mut file_list = FilesList { files: Vec::new() };

    match read_folder(&path, &mut file_list) {
        Ok(()) => {
            log::info!("Folder {} read, {} files found", &folder_path, file_list.files.len());

            match save_output(&mut file_list) {
                Ok(()) => log::info!("Output saved"),
                Err(err) => {
                    log::error!("Error reading folder: {:?}", &path);
                    log::error!("{}", err);
                }
            }
        }
        Err(err) => log::error!("{}", err),
    }
}

fn read_folder(folder_path: &Path, file_list: &mut FilesList) -> Result<(), std::io::Error> {
    let paths = fs::read_dir(folder_path)?;

    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            if let Err(err) = read_folder(&path, file_list) {
                log::error!("Error reading folder: {:?}", &path);
                log::error!("{}", err);
            }
        } else {
            let metadata = fs::metadata(&path)?;

            let mod_date = metadata.modified()?;
            let duration_since_epoch = mod_date.duration_since(UNIX_EPOCH).unwrap();
            let tv_sec = duration_since_epoch.as_secs();

            let file = FileStruc {
                path: path.display().to_string(),
                size: metadata.len(),
                lmod: tv_sec,
            };

            file_list.files.push(file)
        }
    }
    Ok(())
}

fn save_output(file_list: &mut FilesList) -> Result<(), std::io::Error> {
    let json_data = serde_json::to_string_pretty(file_list).unwrap();

    let current_time = SystemTime::now();
    let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration_since_epoch.as_micros().to_string();

    let output_file_path = format!("./output/{}_output.json", timestamp);
    log::info!("Saving file {}", &output_file_path);

    if let Err(err) = fs::write(&output_file_path, json_data) {
        log::info!("Error saving file {}", output_file_path);
        return Err(err);
    }

    Ok(())
}
