use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::get_format_msg;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn write_data_file(data: &[u8], directory_name: &str, file_name: &str) {
    let mut full_file_path = String::new();
    full_file_path.push_str(directory_name);
    full_file_path.push_str(file_name);
    let path = Path::new(&full_file_path);
    if path.exists() {
        println!("File already exists {}", full_file_path);
    } else {
        let mut file = File::create(path).expect(&get_format_msg("Cannot create file", file_name));
        file.write_all(data).expect(&get_format_msg("Cannot write file", file_name));
    }
}

pub fn create_output_folder() -> String {
    // Create sub-folder in current folder by timestamp
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_epoch.as_millis().to_string();
    let mut path: String = String::from("sai-output");
    path.push_str(&timestamp);
    path.push_str("/");
    fs::create_dir(&path).expect(&get_format_msg("Cannot create folder", &path));
    return path;
}

pub fn remove_download_folder(dir: &str) {
    fs::remove_dir_all(dir).expect(&get_format_msg("Cannot remove folder", dir));
}

pub fn get_raw_file_content(path: PathBuf) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn get_lines(path: PathBuf) -> Vec<String> {
    let content = get_raw_file_content(path);
    let lines = content.split(LINE_ENDING);

    // Remove empty lines
    lines.filter_map(|mut line| {
        line = line.trim();
        if line.is_empty() {
            None
        } else { Some(line.to_string()) }
    }).collect()
}
