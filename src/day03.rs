use crate::spoilers;
use crate::utils::load_from_file;
use std::collections::BTreeMap;

struct Racksuck {
    priority: usize,
}

// https://adventofcode.com/2022/day/3
pub fn solve_day() {
    println!("--- Day 3: Rucksack Reorganization ---");
    let priorities_table = build_priorities_table();
    test_example(&priorities_table);
    let racksucks = build_racksucks(&priorities_table, &get_input());
    assert_eq!(racksucks.len(), 300);
    let sum = calculate_priority_sum(&racksucks);
    assert_eq!(sum, spoilers::d03());
    println!("  Part one: {}", sum);
    test_example_p2(&priorities_table);
    let racksucks = build_racksucks_p2(&priorities_table, &get_input());
    let sum = calculate_priority_sum(&racksucks);
    assert_eq!(sum, spoilers::d03_p2());
    println!("  Part two: {}", sum);
}

fn test_example(priorities_table: &BTreeMap<char, usize>) {
    let racksucks = build_racksucks(&priorities_table, &get_example_input());
    assert_eq!(calculate_priority_sum(&racksucks), 157);
}

fn test_example_p2(priorities_table: &BTreeMap<char, usize>) {
    let racksucks = build_racksucks_p2(&priorities_table, &get_example_input());
    assert_eq!(calculate_priority_sum(&racksucks), 70);
}

fn get_priority(priorities_table: &BTreeMap<char, usize>, duplicate_item_type: char) -> usize {
    priorities_table
        .get(&duplicate_item_type)
        .copied()
        .unwrap_or(0)
}

fn build_priorities_table() -> BTreeMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect()
}

fn get_example_input() -> Vec<String> {
    vec![
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        String::from("PmmdzqPrVvPwwTWBwg"),
        String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        String::from("ttgJtRGJQctTZtZT"),
        String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ]
}

fn get_input() -> Vec<String> {
    load_from_file("src/puzzle_inputs/day03.txt")
}

fn build_racksucks(priorities_table: &BTreeMap<char, usize>, input: &[String]) -> Vec<Racksuck> {
    input
        .iter()
        .map(|line| {
            let (head, tail) = line.split_at(line.len() / 2);
            build_racksuck(priorities_table, head, tail)
        })
        .collect()
}

fn build_racksucks_p2(priorities_table: &BTreeMap<char, usize>, input: &[String]) -> Vec<Racksuck> {
    input
        .chunks(3)
        .filter_map(|lines| {
            let [group_line1, group_line2, group_line3] = <&[_; 3]>::try_from(lines).unwrap();

            group_line1
                .chars()
                .find(|&c| group_line2.contains(c) && group_line3.contains(c))
                .map(|c| build_racksuck_p2(priorities_table, c))
        })
        .collect()
}

fn build_racksuck(
    priorities_table: &BTreeMap<char, usize>,
    compartment_one: &str,
    compartment_two: &str,
) -> Racksuck {
    let priority = match get_duplicate_item_type(&compartment_one, &compartment_two) {
        Some(duplicate_item_type) => get_priority(priorities_table, duplicate_item_type),
        None => 0,
    };

    Racksuck { priority }
}

fn build_racksuck_p2(priorities_table: &BTreeMap<char, usize>, badge: char) -> Racksuck {
    let priority = get_priority(priorities_table, badge);
    Racksuck { priority }
}

fn get_duplicate_item_type(compartment_one: &str, compartment_two: &str) -> Option<char> {
    compartment_one
        .chars()
        .find(|&c| compartment_two.contains(c))
}

fn calculate_priority_sum(racksucks: &[Racksuck]) -> usize {
    racksucks.iter().map(|r| r.priority).sum()
}
