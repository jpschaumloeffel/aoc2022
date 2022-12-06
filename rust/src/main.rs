use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate lazy_static;
extern crate core;

mod day1;
mod day2;
mod day3;
mod day4;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// day to run
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    let file = File::open(format!("src/day{}/input", args.day)).unwrap();
    let buf_reader = BufReader::new(file);

    let input_lines = buf_reader.lines();

    if args.day == 1 {
        day1::process(input_lines);
    } else if args.day == 2 {
        day2::process(input_lines);
    } else if args.day == 3 {
        day3::process(input_lines);
    } else if args.day == 4 {
        day4::process(input_lines);
    }
}
