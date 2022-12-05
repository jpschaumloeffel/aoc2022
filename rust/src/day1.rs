pub fn process(input_lines: impl IntoIterator<Item = Result<String, std::io::Error>>) {
    let mut elves = vec![];
    let mut current_calories = 0;

    for line in input_lines {
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

    // sort elves descending
    elves.sort_by(|a, b| b.cmp(a));
    println!("Part 1: #1 elf carrying {} calories", elves.get(0).unwrap());

    println!(
        "Part 2: Top 3 elves carrying {} calories",
        elves[..3].iter().sum::<i32>()
    );
}
