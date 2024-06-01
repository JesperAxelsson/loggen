#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Utc};
use clap::Parser;

// Generate laravel logs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,
    // format: String,
    file_path: Option<String>,

    #[arg(short, long, default_value_t = 0)]
    extra_lines: usize,
    #[arg(short, long, default_value_t = 1)]
    batch_size: usize,
    #[arg(short, long, default_value_t = true)]
    console_output: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // let config = Args {
    //     file_path: Some("banana.log".to_owned()),
    //     delay: 10,
    //     // format: "".to_owned(),
    //     extra_lines: 4,
    //     batch_size: 1,
    //     console_output: true,
    // };

    let mut output = "".to_owned();
    let mut counter = 0;

    loop {
        // Reset vars
        output.clear();

        for _ in 0..args.batch_size {
            counter += 1;

            // Log header
            let dt: DateTime<Utc> = Utc::now();
            output.push_str(&format!(
                "[{}] local.INFO: Amazing logline {}!\n",
                dt.format("%Y-%m-%d %H:%M:%S"),
                counter,
            ));

            // If log use extra lines
            for i in 0..args.extra_lines {
                output.push_str(&format!("    More lines: {}\n", i));
            }
        }

        if let Some(ref file_path) = args.file_path {
            let mut file = File::options().append(true).create(true).open(file_path)?;
            file.write_all(output.as_bytes())?;
        }
        if args.console_output {
            print!("{}", output);
        }

        sleep(Duration::from_millis(args.delay));
    }
}
