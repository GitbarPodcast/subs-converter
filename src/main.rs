extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct SubtitleEntry {
    start: String,
    end: String,
    text: String,
}

fn main() -> io::Result<()> {
    // Check for command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} input.srt output.kjson", args[0]);
        std::process::exit(1);
    }

    // Open the SRT file for reading
    let input_file = File::open(&args[1])?;
    let reader = BufReader::new(input_file);

    // Initialize variables to store subtitle data
    let mut subtitles = Vec::new();
    let mut current_subtitle = SubtitleEntry {
        start: String::new(),
        end: String::new(),
        text: String::new(),
    };

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            if !current_subtitle.start.is_empty() {
                subtitles.push(current_subtitle);
                current_subtitle = SubtitleEntry {
                    start: String::new(),
                    end: String::new(),
                    text: String::new(),
                };
            }
        } else if current_subtitle.start.is_empty() {
            current_subtitle.start = line.clone();
        } else if current_subtitle.end.is_empty() {
            current_subtitle.end = line.clone();
        } else {
            if !current_subtitle.text.is_empty() {
                current_subtitle.text.push_str("\n");
            }
            current_subtitle.text.push_str(&line);
        }
    }

    let output_file = File::create(&args[2])?;
    let kjson = serde_json::to_string_pretty(&subtitles)?;

    io::Write::write_all(&mut io::BufWriter::new(output_file), kjson.as_bytes())?;

    Ok(())
}