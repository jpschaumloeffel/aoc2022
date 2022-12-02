use std::io::{self, BufRead};

fn main() {

    let mut elves = vec![];
    let mut current_calories = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let content = line.unwrap();
        if content == "" {
            // next elf
            elves.push(current_calories);
            current_calories = 0;
            continue;
        }

        // sum calories for elf
        let content_int = content.parse::<i32>().unwrap();
        current_calories += content_int;
    }

    let max_value = elves.iter().max();
    if max_value.is_some() {
        println!("max calories is {}", max_value.unwrap());
    }
}
