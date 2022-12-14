extern crate core;

#[cfg(test)]
mod test;

use std::fmt;
use bytes::Bytes;
use saidl_helper::file::{create_output_folder, remove_download_folder, write_data_file};
use saidl_helper::{get_format_msg, run_os_command, http::send_wrapped_request};
use reqwest::{header::HeaderMap};
use tokio;

pub async fn get_response_bytes(url: &str, headers: &Option<HeaderMap>, h2: bool, delay: Option<u64>) -> Result<Bytes, fmt::Error> {
    let response = send_wrapped_request(url, headers, h2, delay).await?;
    let data = response.bytes().await.expect(&get_format_msg("Unpack data failed: {}", url));
    return Ok(data);
}

pub fn strip_png(data: Bytes) -> Bytes {
    data.slice(8..)
}

pub async fn download(input: &Vec<String>, png: bool, h2: bool, multi_thread:bool, keep: bool, headers: &Option<HeaderMap>, output: Option<String>, delay: Option<u64>) {
    let list_file = "list.txt";
    let dir: String = create_output_folder();
    let mut downloaded_file = String::new();

    let mut fragments = Vec::new();
    // Download all file
    for (index, url) in input.iter().enumerate() {
        let mut file_loc = String::new();
        let mut file_name = index.to_string();
        file_name.push_str(".html");
        file_loc.push_str("file ./");
        file_loc.push_str(&file_name);
        file_loc.push_str("\n");
        downloaded_file.push_str(file_loc.as_str());
        // download_and_write_fragment(url, headers, png, h2, file_name, &dir).await;
        fragments.push(HLSFragmentHandler::new(url.to_string(), headers.clone(), png, h2, file_name, dir.clone(), delay));

    }
    if multi_thread {
        let tasks: Vec<_> = fragments.into_iter().map(|frag| tokio::spawn(async move { frag.download_and_write().await; })).collect();
        for task in tasks {
            task.await.unwrap();
        }
    } else {
        for f in fragments {
            f.download_and_write().await;
        }
    }

    let list_file_data = downloaded_file.as_bytes();
    write_data_file(list_file_data, dir.as_ref(), list_file);
    // let _ = try_join_all(tasks.iter()).await;
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

pub struct HLSFragmentHandler {
    url: String,
    headers: Option<HeaderMap>,
    png: bool,
    h2: bool,
    file_name: String,
    dir: String,
    delay: Option<u64>,
}

impl HLSFragmentHandler {
    fn new(url: String, headers: Option<HeaderMap>, png: bool, h2: bool, file_name: String, dir: String, delay: Option<u64>) -> Self {
        Self { url, headers, png, h2, file_name, dir, delay }
    }

    async fn download_and_write (self) {
        let mut data = get_response_bytes(&self.url, &self.headers, self.h2, self.delay).await.unwrap();
        if self.png {
            data = strip_png(data);
        }
        write_data_file(&data, &self.dir, &self.file_name);
    }
}
