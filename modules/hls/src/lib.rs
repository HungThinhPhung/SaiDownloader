extern crate core;

#[cfg(test)]
mod test;

use std::fmt;
use bytes::Bytes;
use saidl_helper::file::{create_output_folder, remove_download_folder, write_data_file};
use saidl_helper::{get_format_msg, run_os_command, http::send_request};
use reqwest::{header::HeaderMap};

pub fn get_response_bytes(url: &str, headers: &Option<HeaderMap>) -> Result<Bytes, fmt::Error> {
    let response = send_request(url, headers)?;
    let data = response.bytes().expect(&get_format_msg("Unpack data failed: {}", url));
    return Ok(data);
}

pub fn strip_png(data: Bytes) -> Bytes {
    data.slice(8..)
}

pub fn download(input: &Vec<String>, png: bool, keep: bool, headers: &Option<HeaderMap>, output: Option<String>) {
    let list_file = "list.txt";
    let dir = create_output_folder();
    let mut downloaded_file = String::new();

    // Download all file
    for (index, url) in input.iter().enumerate() {
        let http_result = get_response_bytes(url, headers);
        let mut data: Bytes;
        match http_result {
            Err(e) => {
                println!("{e}");
                return;
            }
            Ok(b) => data = b
        };
        if png {
            data = strip_png(data);
        }
        let mut file_loc = String::new();
        let mut file_name = index.to_string();
        file_name.push_str(".html");
        write_data_file(&data, dir.as_ref(), &file_name);
        file_loc.push_str("file ./");
        file_loc.push_str(&file_name);
        file_loc.push_str("\n");
        downloaded_file.push_str(file_loc.as_str());
    }
    let list_file_data = downloaded_file.as_bytes();
    write_data_file(list_file_data, dir.as_ref(), list_file);

    // Create output video file name
    let mut output_video_name: String;
    match output {
        None => {
            output_video_name = dir[10..dir.len() - 1].to_string();
        }
        Some(name) => {
            output_video_name = name;
        }
    }
    output_video_name.push_str(".mp4");

    // Create command string
    let mut ffmpeg_cmd = String::from("ffmpeg -f concat -safe 0 -i ");
    ffmpeg_cmd.push_str(&dir);
    ffmpeg_cmd.push_str(list_file);
    ffmpeg_cmd.push_str(" -c copy ");
    ffmpeg_cmd.push_str(&output_video_name);

    // Run command and delete temp folder
    run_os_command(&ffmpeg_cmd);
    if !keep {
        remove_download_folder(&dir);
    }
}
