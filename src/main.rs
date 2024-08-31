use std::{
    env::args,
    fs::{self, create_dir_all},
    path::Path,
};

use eyre::OptionExt;
use video2vtt::{
    storyboard::{
        frames::extract_frames, sprite_sheet::create_sprite_sheet, vtt_storyboard::create_webvtt,
    },
    utils::extract_name_file,
};

fn main() -> eyre::Result<()> {
    let mut args = args();
    args.next();

    if let Some(arg) = args.next() {
        match arg.trim() {
            "--path" | "-P" => {
                if let Some(path_str) = args.next() {
                    let path_vid = Path::new(&path_str);

                    if !path_vid.exists() {
                        eprintln!("ERROR: path not exist");
                    } else {
                        let file_name = extract_name_file(path_vid)
                            .ok_or_eyre("ERROR: cant extract file name")?;

                        let parent_folder = Path::new("assets_result").join(file_name);
                        let folder_frames = &parent_folder.join("frames");

                        if !folder_frames.exists() {
                            create_dir_all(folder_frames)?;
                        }

                        let folder_frames_str = &folder_frames
                            .to_str()
                            .ok_or_eyre("ERROR: failed to get frames folder string")?;

                        extract_frames(&path_str, folder_frames_str, "180:-1")?;

                        let out_sprite = &parent_folder.join("storyboard.jpg");

                        let (w, h) = create_sprite_sheet(folder_frames_str, out_sprite)?;

                        let frame_count = fs::read_dir(folder_frames)?.count();

                        let out_vtt = &parent_folder.join("storyboard.vtt");
                        let out_vtt_str = &out_vtt
                            .to_str()
                            .ok_or_eyre("ERROR: failed to get frames folder string")?;

                        create_webvtt(out_vtt_str, "storyboard.jpg", 2.0, frame_count, w, h)?;

                        println!("finished");
                    }
                }
            }

            "--version" | "-V" => {
                println!("video2vtt v0.0.1");
            }

            _ => {
                println!("INFO: provide path after --path");
            }
        }
    }

    Ok(())
}
