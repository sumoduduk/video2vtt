use image::{GenericImageView, RgbImage};
use std::{fs, path::Path};

pub fn create_sprite_sheet(input_dir: &str, output_image: &Path) -> eyre::Result<(u32, u32)> {
    let mut images = vec![];

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "jpg") {
            let img = image::ImageReader::open(&path)?.decode()?;
            images.push(img);
        }
    }

    let frame_count = images.len();
    let columns = (frame_count as f32).sqrt().ceil() as u32;
    let rows = (frame_count as f32 / columns as f32).ceil() as u32;

    let (frame_width, frame_height) = images[0].dimensions();

    let mut sprite_sheet = RgbImage::new(frame_width * columns, frame_height * rows);

    for (i, img) in images.into_iter().enumerate() {
        let x = (i as u32 % columns) * frame_width;
        let y = (i as u32 / columns) * frame_height;
        image::imageops::overlay(&mut sprite_sheet, &img.to_rgb8(), x.into(), y.into());
    }

    sprite_sheet.save(output_image)?;

    Ok((frame_width, frame_height))
}

#[cfg(test)]
mod tests {
    use fs::create_dir_all;

    use super::*;

    const FRAMEPATH: &str = "assets/helloworld/frames";
    const FOLDER_RESULT: &str = "assets_result";

    #[test]
    fn test_sprite_create() -> eyre::Result<()> {
        let target_folder = Path::new(FOLDER_RESULT);

        if !target_folder.exists() {
            create_dir_all(target_folder)?;
        }

        let output_image_path = target_folder.join("storyboard.jpg");

        let (w, h) = create_sprite_sheet(FRAMEPATH, &output_image_path)?;
        dbg!(w, h);

        assert!(w > 0);
        assert!(h > 0);

        Ok(())
    }
}
