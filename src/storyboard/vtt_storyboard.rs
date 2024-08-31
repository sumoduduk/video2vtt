use std::fs::File;
use std::io::Write;

pub fn create_webvtt(
    output_vtt: &str,
    sprite_sheet_name: &str,
    frame_duration: f32,
    frame_count: usize,
    frame_width: u32,
    frame_height: u32,
) -> eyre::Result<()> {
    let mut file = File::create(output_vtt)?;

    writeln!(file, "WEBVTT")?;

    let columns = (frame_count as f32).sqrt().ceil() as u32;

    for i in 0..frame_count {
        let start_time = i as f32 * frame_duration;
        let end_time = start_time + frame_duration;
        let x = (i as u32 % columns) * frame_width;
        let y = (i as u32 / columns) * frame_height;
        writeln!(
            file,
            "\n{:02}:{:02}:{:02}.000 --> {:02}:{:02}:{:02}.000",
            (start_time / 3600.) as u32,
            (start_time / 60.) as u32 % 60,
            (start_time % 60.) as u32,
            (end_time / 3600.) as u32,
            (end_time / 60.) as u32 % 60,
            (end_time % 60.) as u32
        )?;

        writeln!(
            file,
            "{}#xywh={},{},{},{}",
            sprite_sheet_name, x, y, frame_width, frame_height
        )?;
    }

    Ok(())
}
