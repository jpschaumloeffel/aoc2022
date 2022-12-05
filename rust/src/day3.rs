use std::collections::HashSet;

pub fn process(input_lines: impl IntoIterator<Item = Result<String, std::io::Error>>) {
    let mut priority_sum = 0;

    for line in input_lines {
        let content = line.unwrap();

        let length = content.len();
        let p1 = &content[..length / 2];
        let p2 = &content[length / 2..];

        println!("{:?} -- {:?}", p1, p2);

        for char in p1.chars() {
            if p2.find(char).is_some() {
                let priority = item_priority(char);

                println!("common: {:?}, priority: {:?}", char, priority);

                priority_sum += priority;
                break;
            }
        }
    }

    println!("part 1: priority sum is {}", priority_sum);
}

fn item_priority(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        return (item as u32) - ('a' as u32) + 1;
    } else if item >= 'A' && item <= 'Z' {
        return (item as u32) - ('A' as u32) + 27;
    }

    panic!("at the discotheque");
}
