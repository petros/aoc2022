use crate::spoilers;
use crate::utils::load_from_file;

pub fn solve_day() {
    println!("--- Day 2: Rock Paper Scissors ---");
    test_example();
    let strategy_guide: Vec<String> = load_from_file("src/puzzle_inputs/day02.txt");
    assert_eq!(strategy_guide.len(), 2500);
    let score = get_score(&strategy_guide);
    assert_eq!(score, spoilers::d02());
    println!("  Part one: {}", score);
    test_example_p2();
    let score = get_score(&decrypt_strategy_guide(&strategy_guide));
    assert_eq!(score, spoilers::d02_p2());
    println!("  Part two: {}", score);
}

fn test_example() {
    let strategy_guide: Vec<String> = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
    assert_eq!(strategy_guide.len(), 3);
    assert_eq!(get_score(&strategy_guide), 15);
}

fn test_example_p2() {
    let strategy_guide: Vec<String> = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
    let decrypted_guide = decrypt_strategy_guide(&strategy_guide);
    assert_eq!(strategy_guide.len(), 3);
    assert_eq!(get_score(&decrypted_guide), 12);
}

fn get_shape_score(round: &str) -> u32 {
    match round {
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

fn get_outcome_score(round: &str) -> u32 {
    // Rock, Paper, Scissors
    match round {
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

fn get_round_score(round: &str) -> u32 {
    get_shape_score(round) + get_outcome_score(round)
}

fn get_score(strategy_guide: &[String]) -> u32 {
    let mut score: u32 = 0;
    for round in strategy_guide.iter() {
        score += get_round_score(round);
    }
    score
}

fn decrypt_strategy_guide(strategy_guide: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for round in strategy_guide.iter() {
        result.push(decrypt_move(round));
    }
    result
}

fn decrypt_move(round: &str) -> String {
    // X => Lose (Rock)
    // Y => Draw (Paper)
    // Z => Win  (Scissors)
    match round {
        "A X" => "A Z".to_string(), // Rock <> Rock => Scissors
        "A Y" => "A X".to_string(), // Rock <> Paper => Rock
        "A Z" => "A Y".to_string(), // Rock <> Scissors => Paper
        "B X" => "B X".to_string(), // Paper <> Rock => Rock
        "B Y" => "B Y".to_string(), // Paper <> Paper => Paper
        "B Z" => "B Z".to_string(), // Paper <> Scissors => Scissors
        "C X" => "C Y".to_string(), // Scissors <> Rock => Paper
        "C Y" => "C Z".to_string(), // Scissors <> Paper => Scissors
        "C Z" => "C X".to_string(), // Scissors <> Scissors => Rock
        _ => "".to_string(),
    }
}
