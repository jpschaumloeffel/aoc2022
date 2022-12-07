use regex::Regex;
use std::collections::VecDeque;

pub fn process(input_lines: impl IntoIterator<Item = Result<String, std::io::Error>>) {
    let mut input_iter = input_lines.into_iter();
    let mut init_lines: Vec<String> = input_iter
        .by_ref()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();
    init_lines.reverse();

    let re = Regex::new(r"\s+").unwrap();
    let first_line = init_lines.remove(0);
    let splits = re.split(first_line.trim());

    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut stacks2: Vec<VecDeque<String>> = Vec::new();

    for _number in splits {
        // create the list of cargo stacks
        stacks.push(VecDeque::new());
        stacks2.push(VecDeque::new());
    }

    // extract the crate content
    for content in init_lines {
        let mut i = 0;
        for stack in stacks.iter_mut() {
            let crate_content = &content[(i + 1)..(i + 2)];
            i += 4;

            if crate_content != " " {
                stack.push_front(String::from(crate_content));
            }
        }

        // create it twice for part 2
        i = 0;
        for stack in stacks2.iter_mut() {
            let crate_content = &content[(i + 1)..(i + 2)];
            i += 4;

            if crate_content != " " {
                stack.push_front(String::from(crate_content));
            }
        }
    }

    for line in input_iter {
        // crane stuff
        let content = line.unwrap();

        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(content.as_str()).unwrap();
        let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let from_stack_idx = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to_stack_idx = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        // move-master 9000
        for _ in 0..count {
            let from_stack = stacks.get_mut(from_stack_idx - 1).unwrap();
            let crate_content = from_stack.pop_front().unwrap();
            let to_stack = stacks.get_mut(to_stack_idx - 1).unwrap();
            to_stack.push_front(crate_content);
        }

        // move-master 9001
        let from_stack = stacks2.get_mut(from_stack_idx - 1).unwrap();

        let mut moved_crates = VecDeque::new();
        for _ in 0..count {
            moved_crates.push_front(from_stack.pop_front().unwrap());
        }
        let to_stack = stacks2.get_mut(to_stack_idx - 1).unwrap();
        for mc in moved_crates {
            to_stack.push_front(mc);
        }
    }

    println!();
    println!("end configuration 1");
    print_stacks(&stacks);
    println!();
    println!("end configuration 2");
    print_stacks(&stacks2);
    println!();

    let mut result1 = String::from("");
    stacks
        .iter()
        .map(|s| {
            let x = s.get(0).unwrap();
            x
        })
        .for_each(|s| result1.push_str(s));

    let mut result2 = String::from("");
    stacks2
        .iter()
        .map(|s| {
            let x = s.get(0).unwrap();
            x
        })
        .for_each(|s| result2.push_str(s));

    println!("result 1: {}", result1);
    println!("result 2: {}", result2);
}

fn print_stacks(stacks: &Vec<VecDeque<String>>) {
    let mut i = 1;
    for stack in stacks {
        println!("stack {:?}: {:?}", i, stack);
        i += 1;
    }
}
