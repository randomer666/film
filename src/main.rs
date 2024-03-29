use clap::Parser;
use core::f64;
use std::collections::HashMap;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Args {
    #[arg(short, default_value_t = 24.0)]
    fps: f64,
    #[arg(short)]
    run_time: f64,
    #[arg(long)]
    format: i32,
    // #[arg(short, long)]
    //film_type: String,
}

fn main() {
    let args = Args::parse();
    let formats = HashMap::from([
        (1, ("8mm", 80.0)),
        (2, ("Super 8", 72.0)),
        (3, ("16mm", 40.0)),
        (4, ("35x2 perf", 32.0)),
        (5, ("35x3 perf", 21.33)),
        (6, ("35x4 perf", 16.0)),
        (7, ("65", 12.8)),
    ]);

    if let Some((format_name, frames_per_foot)) = formats.get(&args.format) {
        println!(
            "Selected format: {} and its frames per foot is {}",
            format_name, frames_per_foot
        );
        println!(
            "the length needed for the project is {}",
            calc_length(args.run_time, args.fps, *frames_per_foot)
        );
    } else {
        println!("Invalid format number {}", args.format);
    }
    /* function to calculate the length of film needed
    let mut frames_per_foot: f64 = [selectedFormat].frames_per_foot;
            float length = (run_time * ((fps * 60) / frames_per_foot));
            length = ceil(length); // Round up to the next whole number
    */
    fn calc_length(run_time: f64, fps: f64, frames_per_foot: f64) -> f64 {
        let total_frames = run_time * (fps * 60.0);
        let length = total_frames / frames_per_foot;
        length.ceil()
    }
}
