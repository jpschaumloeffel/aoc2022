struct Section {
    start: u32,
    stop: u32,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        (self.start <= other.start)
            && (other.start <= self.stop)
            && (self.start <= other.stop)
            && (other.stop <= self.stop)
    }

    fn overlaps(&self, other: &Section) -> bool {
        !((self.stop < other.start) || (self.start > other.stop))
    }
}

fn parse_sections(line: String) -> (Section, Section) {
    let (s1, s2) = line.split_once(",").unwrap();
    let (s1_start, s1_stop) = s1.split_once("-").unwrap();
    let (s2_start, s2_stop) = s2.split_once("-").unwrap();
    (
        Section {
            start: s1_start.parse().unwrap(),
            stop: s1_stop.parse().unwrap(),
        },
        Section {
            start: s2_start.parse().unwrap(),
            stop: s2_stop.parse().unwrap(),
        },
    )
}

pub fn process(input_lines: impl IntoIterator<Item = Result<String, std::io::Error>>) {
    let mut contain_count = 0;
    let mut overlap_count = 0;

    for line in input_lines {
        let content = line.unwrap();
        let (section1, section2) = parse_sections(content);

        if section1.contains(&section2) || section2.contains(&section1) {
            contain_count += 1;
        }

        if section1.overlaps(&section2) {
            overlap_count += 1;
        }
    }

    println!("Part 1: {} pairs are fully overlapping", contain_count);
    println!("Part 2: {} pairs overlap", overlap_count);
}
