use indicatif::ProgressStyle;
use std::io::{Write, Result, BufWriter};

const IMAGE_WIDTH: u64 = 1024;
const IMAGE_HEIGHT: u64 = 1024;

pub mod vec3;

fn main() -> Result<()> {
    let stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

    write!(&mut writer, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")?;

    let progress_bar = indicatif::ProgressBar::new(IMAGE_WIDTH * IMAGE_HEIGHT)
        .with_message("Pixels written")
        .with_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan} {msg}: {percent:>3}%").unwrap());

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let color = vec3::Color3::new(
                (i as f32) / ((IMAGE_WIDTH - 1) as f32),
                (j as f32) / ((IMAGE_HEIGHT - 1) as f32), 0.0);

            color.write_ppm(&mut writer)?;
        }
        progress_bar.inc(IMAGE_WIDTH);
    }

    progress_bar.finish_and_clear();

    eprintln!("Done!");

    Ok(())
}
