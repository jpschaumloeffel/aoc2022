pub fn process(input_lines: impl IntoIterator<Item = Result<String, std::io::Error>>) {
    let mut priority_sum = 0;
    let mut priority_sum_2 = 0;

    let mut group_strings = vec![];

    for line in input_lines {
        let content = line.unwrap();

        let length = content.len();
        let p1 = &content[..length / 2];
        let p2 = &content[length / 2..];

        println!("{:?} -- {:?}", p1, p2);

        let common = common_char(vec![p1.to_string(), p2.to_string()]);

        let priority = item_priority(common.unwrap());
        println!("common: {:?}, priority: {:?}", common, priority);

        priority_sum += priority;

        group_strings.push(content);
        if group_strings.len() == 3 {
            let common_2 = common_char(group_strings.to_vec());
            priority_sum_2 += item_priority(common_2.unwrap());

            group_strings = vec![];
        }
    }

    println!("part 1: priority sum is {}", priority_sum);
    println!("part 2: priority sum is {}", priority_sum_2);
}

fn common_char(strings: Vec<String>) -> Option<char> {
    if strings.len() > 0 {
        for char in strings[0].chars() {
            let mut failed = false;
            for s in strings[1..].iter() {
                if s.find(char).is_none() {
                    failed = true;
                    break;
                }
            }
            if failed {
                continue;
            }
            return Some(char);
        }
    }

    None
}

fn item_priority(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        return (item as u32) - ('a' as u32) + 1;
    } else if item >= 'A' && item <= 'Z' {
        return (item as u32) - ('A' as u32) + 27;
    }

    panic!("at the discotheque");
}
