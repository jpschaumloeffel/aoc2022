use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// day to run
   #[arg(short, long)]
   day: u8,
}

fn main() {

    let args = Args::parse();

    let file = File::open(format!("day{}/input.txt", args.day)).unwrap();
    let buf_reader = BufReader::new(file);

    let input_lines = buf_reader.lines();

    if args.day == 1 {
        day1::process(input_lines);
    } else if args.day == 2 {
        day2::process(input_lines);
    }

}
