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
    /// Select the film format,
        /// use --format followed by the number associated with film format
    /// 1 - 8mm, 2 - Super 8, 3 - 16mm,
    /// 4 - 35mm 2 perf, 5 - 35mm 3 perf
    /// 6 - 35mm 4 perf, 7 - 65mm
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

    //let format_number = &args.format;
    println!("Frames - {}", args.run_time * (args.fps * 60.0));

    match formats.get(&args.format) {
        Some((_ ,frames_per_foot)) => {
            println!(
                "The length needed for the project is {}",
                calc_length(args.run_time, args.fps, *frames_per_foot)
            )
        }
        None => {
            println!(
                "Wrong format!"
            )
        }
    };

    /*if let Some((format_name, frames_per_foot)) = formats.get(&args.format) {
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
    */
        fn calc_length(run_time: f64, fps: f64, frames_per_foot: f64) -> f64 {
        let total_frames = run_time * (fps * 60.0);
        let length = total_frames / frames_per_foot;
        //length.ceil()
        length
    }
}
