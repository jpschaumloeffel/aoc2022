use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum RSPFigure {
    Rock,
    Scissors,
    Paper,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum RSPResult {
    Lose,
    Win,
    Draw,
}

fn rsp_win_table(a: &RSPFigure, b: &RSPFigure) -> RSPResult {
    if *a == *b {
        return RSPResult::Draw;
    };
    if (*a == RSPFigure::Rock && *b == RSPFigure::Paper)
        || (*a == RSPFigure::Scissors && *b == RSPFigure::Rock)
        || (*a == RSPFigure::Paper && *b == RSPFigure::Scissors)
    {
        return RSPResult::Lose;
    };
    return RSPResult::Win;
}

fn make_result_tables() -> HashMap<RSPFigure, HashMap<RSPFigure, RSPResult>> {
    fn make_result_table(figure: RSPFigure) -> HashMap<RSPFigure, RSPResult> {
        let mut result_table: HashMap<RSPFigure, RSPResult> = HashMap::new();
        for figure2 in [RSPFigure::Rock, RSPFigure::Paper, RSPFigure::Scissors] {
            result_table.insert(figure2, rsp_win_table(&figure, &figure2));
        }
        result_table
    }

    let mut result_tables: HashMap<RSPFigure, HashMap<RSPFigure, RSPResult>> = HashMap::new();
    for figure in [RSPFigure::Rock, RSPFigure::Paper, RSPFigure::Scissors] {
        result_tables.insert(figure, make_result_table(figure));
    }
    result_tables
}

fn make_figure_tables() -> HashMap<RSPFigure, HashMap<RSPResult, RSPFigure>> {
    fn make_figure_table(figure: RSPFigure) -> HashMap<RSPResult, RSPFigure> {
        let mut figure_table: HashMap<RSPResult, RSPFigure> = HashMap::new();
        for figure2 in [RSPFigure::Rock, RSPFigure::Paper, RSPFigure::Scissors] {
            figure_table.insert(rsp_win_table(&figure2, &figure), figure2);
        }
        figure_table
    }

    let mut figure_tables: HashMap<RSPFigure, HashMap<RSPResult, RSPFigure>> = HashMap::new();
    for figure in [RSPFigure::Rock, RSPFigure::Paper, RSPFigure::Scissors] {
        figure_tables.insert(figure, make_figure_table(figure));
    }
    figure_tables
}

lazy_static! {
    static ref RSP_RESULT_TABLES: HashMap<RSPFigure, HashMap<RSPFigure, RSPResult>> =
        make_result_tables();
    static ref RSP_FIGURE_TABLES: HashMap<RSPFigure, HashMap<RSPResult, RSPFigure>> =
        make_figure_tables();
}

fn required_figure(figure: &RSPFigure, result: &RSPResult) -> RSPFigure {
    RSP_FIGURE_TABLES
        .get(figure)
        .unwrap()
        .get(result)
        .unwrap()
        .clone()
}

fn parse_line(content: &String) -> (RSPFigure, RSPFigure) {
    let parts = content.split(" ").collect::<Vec<_>>();
    (
        match parts[0] {
            "A" => RSPFigure::Rock,
            "B" => RSPFigure::Paper,
            "C" => RSPFigure::Scissors,
            _ => panic!(),
        },
        match parts[1] {
            "X" => RSPFigure::Rock,
            "Y" => RSPFigure::Paper,
            "Z" => RSPFigure::Scissors,
            _ => panic!(),
        },
    )
}

fn parse_line_b(content: &String) -> (RSPFigure, RSPResult) {
    let parts = content.split(" ").collect::<Vec<_>>();
    (
        match parts[0] {
            "A" => RSPFigure::Rock,
            "B" => RSPFigure::Paper,
            "C" => RSPFigure::Scissors,
            _ => panic!(),
        },
        match parts[1] {
            "X" => RSPResult::Lose,
            "Y" => RSPResult::Draw,
            "Z" => RSPResult::Win,
            _ => panic!(),
        },
    )
}

fn figure_score(figure: &RSPFigure) -> u32 {
    match figure {
        RSPFigure::Rock => 1,
        RSPFigure::Paper => 2,
        RSPFigure::Scissors => 3,
    }
}

fn result_score(result: &RSPResult) -> u32 {
    match result {
        RSPResult::Lose => 0,
        RSPResult::Draw => 3,
        RSPResult::Win => 6,
    }
}

fn score(fig_self: &RSPFigure, fig_opp: &RSPFigure) -> u32 {
    // let result = rsp_win_table(fig_self, fig_opp);
    let result = RSP_RESULT_TABLES
        .get(fig_self)
        .unwrap()
        .get(fig_opp)
        .unwrap();
    figure_score(&fig_self) + result_score(&result)
}

pub fn process(input_lines: impl IntoIterator<Item = Result<String, std::io::Error>>) {
    // println!("debug RSP_RESULT_TABLES: {:?}", make_result_tables());
    // println!("debug RSP_FIGURE_TABLES: {:?}", make_figure_tables());

    let mut score_sum = 0;
    let mut score_sum_b = 0;

    for line in input_lines {
        let content = line.unwrap();

        let (figure_opp, figure_self) = parse_line(&content);
        score_sum += score(&figure_self, &figure_opp);

        // part 2
        let (figure_opp_b, required_result) = parse_line_b(&content);
        let figure_self_b = required_figure(&figure_opp_b, &required_result);

        score_sum_b += score(&figure_self_b, &figure_opp_b);
    }

    println!("expected points: {}", score_sum);
    println!("expected points b: {}", score_sum_b);
}
