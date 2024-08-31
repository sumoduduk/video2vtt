use std::process::Command;

use eyre::eyre;

pub fn extract_frames(input_video: &str, output_dir: &str, scale: &str) -> eyre::Result<()> {
    let output = Command::new("ffmpeg")
        .args([
            "-i",
            input_video,
            "-vf",
            &format!("fps=1/2,scale={}", scale),
            format!("{}/frame%03d.jpg", output_dir).as_str(),
        ])
        .output()?;

    if !output.status.success() {
        return Err(eyre!(String::from_utf8_lossy(&output.stderr).to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::create_dir_all, path::Path};

    const VIDPATH: &str = "assets/helloworld.mp4";
    const FOLDER_RESULT: &str = "assets_result";

    #[test]
    fn test_frame_extract() -> eyre::Result<()> {
        let target_folder = Path::new(FOLDER_RESULT);

        if !target_folder.exists() {
            create_dir_all(target_folder)?;
        }

        let res = extract_frames(VIDPATH, FOLDER_RESULT, "180:-1");
        dbg!(&res);

        assert!(res.is_ok());

        Ok(())
    }
}
