use crate::utils::load_from_file;
//use std::iter::Filter;

// https://adventofcode.com/2022/day/4
pub fn solve_day() {
    println!("--- Day 4: Camp Cleanup ---");
    test_example();
    let count = load_from_file("src/puzzle_inputs/day04.txt")
        .iter()
        .filter(|line| {
            let splits: Vec<&str> = line.split(",").collect();
            let left = splits[0];
            let right = splits[1];
            let left_splits: Vec<&str> = left.split("-").collect();
            let right_splits: Vec<&str> = right.split("-").collect();
            let left_start: usize = left_splits[0].parse::<usize>().unwrap_or(0);
            let left_end: usize = left_splits[1].parse::<usize>().unwrap_or(0);
            let right_start: usize = right_splits[0].parse::<usize>().unwrap_or(0);
            let right_end: usize = right_splits[1].parse::<usize>().unwrap_or(0);
            (left_start >= right_start && left_end <= right_end)
                || (right_start >= left_start && right_end <= left_end)
        })
        .collect::<Vec<_>>()
        .len();
    println!("There are {count} assignment pairs where one range fully contain the other.");
    println!("  --- Part Two ---");
    println!();
    test_example_p2();
}

fn get_example_input() -> Vec<String> {
    vec![
        String::from("2-4,6-8"),
        String::from("2-3,4-5"),
        String::from("5-7,7-9"),
        String::from("2-8,3-7"),
        String::from("6-6,4-6"),
        String::from("2-6,4-8"),
    ]
}

fn test_example() {
    let count = get_example_input()
        .iter()
        .filter(|line| {
            let splits: Vec<&str> = line.split(",").collect();
            let left = splits[0];
            let right = splits[1];
            let left_splits: Vec<&str> = left.split("-").collect();
            let right_splits: Vec<&str> = right.split("-").collect();
            let left_start: usize = left_splits[0].parse::<usize>().unwrap_or(0);
            let left_end: usize = left_splits[1].parse::<usize>().unwrap_or(0);
            let right_start: usize = right_splits[0].parse::<usize>().unwrap_or(0);
            let right_end: usize = right_splits[1].parse::<usize>().unwrap_or(0);
            (left_start >= right_start && left_end <= right_end)
                || (right_start >= left_start && right_end <= left_end)
        })
        .collect::<Vec<_>>()
        .len();
    assert_eq!(count, 2);
}

fn test_example_p2() {}
