use indicatif::ProgressStyle;

const IMAGE_WIDTH: u64 = 1024;
const IMAGE_HEIGHT: u64 = 1024;

fn main() {
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let progress_bar = indicatif::ProgressBar::new(IMAGE_WIDTH * IMAGE_HEIGHT)
        .with_message("Pixels written")
        .with_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan} {msg}: {percent:>3}%").unwrap());

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32) / ((IMAGE_WIDTH - 1) as f32);
            let b = 0.0;

            let ir = (255.999f32 * r) as u8;
            let ig = (255.999f32 * g) as u8;
            let ib = (255.999f32 * b) as u8;

            println!("{ir} {ig} {ib}");

        }
        progress_bar.inc(IMAGE_WIDTH);
    }

    progress_bar.finish_and_clear();

    eprintln!("Done!")
}
