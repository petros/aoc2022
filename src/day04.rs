use crate::spoilers;
use crate::utils::load_from_file;
use std::collections::HashSet;

// https://adventofcode.com/2022/day/4
pub fn solve_day() {
    assert_eq!(solution(&puzzle_input_example()), 2);
    assert_eq!(solution(&puzzle_input()), spoilers::d04());
    assert_eq!(solution_p2(&puzzle_input_example()), 4);
    assert_eq!(solution_p2(&puzzle_input()), spoilers::d04_p2());
    println!("--- Day 4: Camp Cleanup ---");
    println!("  Part one: {}", solution(&puzzle_input()));
    println!("  Part two: {}", solution_p2(&puzzle_input()));
}

fn solution(puzzle_input: &[String]) -> usize {
    puzzle_input
        .iter()
        .filter(|line| {
            let splits: Vec<&str> = line.split(',').collect();
            let left = splits[0];
            let right = splits[1];
            let left_splits: Vec<&str> = left.split('-').collect();
            let right_splits: Vec<&str> = right.split('-').collect();
            let left_start: usize = left_splits[0].parse::<usize>().unwrap_or(0);
            let left_end: usize = left_splits[1].parse::<usize>().unwrap_or(0);
            let right_start: usize = right_splits[0].parse::<usize>().unwrap_or(0);
            let right_end: usize = right_splits[1].parse::<usize>().unwrap_or(0);
            (left_start >= right_start && left_end <= right_end)
                || (right_start >= left_start && right_end <= left_end)
        })
        .collect::<Vec<_>>()
        .len()
}

fn solution_p2(puzzle_input: &[String]) -> usize {
    puzzle_input
        .iter()
        .filter(|line| {
            let splits: Vec<&str> = line.split(',').collect();
            let left = splits[0];
            let right = splits[1];
            let left_splits: Vec<&str> = left.split('-').collect();
            let right_splits: Vec<&str> = right.split('-').collect();
            let left_start: usize = left_splits[0].parse::<usize>().unwrap_or(0);
            let left_end: usize = left_splits[1].parse::<usize>().unwrap_or(0);
            let right_start: usize = right_splits[0].parse::<usize>().unwrap_or(0);
            let right_end: usize = right_splits[1].parse::<usize>().unwrap_or(0);
            let list_a: Vec<_> = (left_start..=left_end).collect();
            let list_b: Vec<_> = (right_start..=right_end).collect();
            let a: HashSet<_> = list_a.into_iter().collect();
            let b: HashSet<_> = list_b.into_iter().collect();
            !a.intersection(&b).collect::<Vec<_>>().is_empty()
        })
        .collect::<Vec<_>>()
        .len()
}

fn puzzle_input() -> Vec<String> {
    load_from_file("src/puzzle_inputs/day04.txt")
}

fn puzzle_input_example() -> Vec<String> {
    vec![
        String::from("2-4,6-8"),
        String::from("2-3,4-5"),
        String::from("5-7,7-9"),
        String::from("2-8,3-7"),
        String::from("6-6,4-6"),
        String::from("2-6,4-8"),
    ]
}
