#[cfg(test)]
mod test;

use bytes::Bytes;
use saidl_helper::file::{create_output_folder, remove_download_folder, write_data_file};
use saidl_helper::{get_format_msg, run_os_command};

pub fn send_request(url: &str) -> Bytes {
    let response = reqwest::blocking::get(url).expect(&get_format_msg("Send request failed", url));
    let data = response.bytes().expect(&get_format_msg("Unpack data failed: {}", url));
    return data;
}

pub fn strip_png(data: Bytes) -> Bytes {
    data.slice(8..)
}

pub fn download(input: Vec<&str>, png: bool) {
    let list_file = "list.txt";
    let dir = create_output_folder();
    let mut downloaded_file = String::new();

    for (index, url) in input.iter().enumerate() {
        let mut data = send_request(url);
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
    let mut output_video_name = dir[10..dir.len() - 1].to_string();
    output_video_name.push_str(".mp4");

    // Create command string
    let mut ffmpeg_cmd = String::from("ffmpeg -f concat -safe 0 -i ");
    ffmpeg_cmd.push_str(&dir);
    ffmpeg_cmd.push_str(list_file);
    ffmpeg_cmd.push_str(" -c copy ");
    ffmpeg_cmd.push_str(&output_video_name);

    // Run command and delete temp folder
    run_os_command(&ffmpeg_cmd);
    remove_download_folder(&dir);
}
