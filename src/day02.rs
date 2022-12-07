use crate::utils::load_from_file;

pub fn solve_day_02() {
    test_solve_day_02();
    println!("--- Day 2: Rock Paper Scissors ---");
    let strategy_guide: Vec<String> = load_from_file("src/puzzle_inputs/day02.txt");
    assert_eq!(strategy_guide.len(), 2500);
    println!(
        "  The total score for this strategy guide would be {}",
        get_score(strategy_guide)
    );
    println!("  --- Part Two ---");
    println!();
}

fn test_solve_day_02() {
    let strategy_guide: Vec<String> = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
    assert_eq!(strategy_guide.len(), 3);
    assert_eq!(get_score(strategy_guide), 15);
}

fn get_shape_score(round: &String) -> u32 {
    match round.as_str() {
        "A X" => 1,
        "A Y" => 2,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 2,
        "B Z" => 3,
        "C X" => 1,
        "C Y" => 2,
        "C Z" => 3,
        _ => 0,
    }
}

fn get_outcome_score(round: &String) -> u32 {
    // Rock, Paper, Scissors
    match round.as_str() {
        "A X" => 3, // Rock <> Rock
        "A Y" => 6, // Rock <> Paper
        "A Z" => 0, // Rock <> Scissors
        "B X" => 0, // Paper <> Rock
        "B Y" => 3, // Paper <> Paper
        "B Z" => 6, // Paper <> Scissors
        "C X" => 6, // Scissors <> Rock
        "C Y" => 0, // Scissors <> Paper
        "C Z" => 3, // Scissors <> Scissors
        _ => 0,
    }
}

fn get_round_score(round: &String) -> u32 {
    return get_shape_score(&round) + get_outcome_score(&round);
}

fn get_score(strategy_guide: Vec<String>) -> u32 {
    let mut score: u32 = 0;
    for round in strategy_guide.iter() {
        score += get_round_score(round);
    }
    return score;
}
