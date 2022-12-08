use crate::utils::load_from_file;
use std::collections::BTreeMap;

struct Racksuck {
    priority: u32,
}

// https://adventofcode.com/2022/day/3
pub fn solve_day_03() {
    println!("--- Day 3: Rucksack Reorganization ---");
    let priorities_table: BTreeMap<String, u32> = build_priorities_table();
    test_example(&priorities_table);
    let racksucks = build_racksucks(&priorities_table, &get_input());
    assert_eq!(racksucks.len(), 300);
    println!(
        "  The sum of the priorities of all duplicate item types is {}",
        calculate_priority_sum(racksucks)
    );
    println!("  --- Part Two ---");
    test_example_p2(&priorities_table);
    let racksucks = build_racksucks_p2(&priorities_table, &get_input());
    println!(
        "  The sum of the priorities of those types is {}",
        calculate_priority_sum(racksucks)
    );
}

fn test_example(priorities_table: &BTreeMap<String, u32>) {
    let racksucks = build_racksucks(&priorities_table, &get_example_input());
    assert_eq!(calculate_priority_sum(racksucks), 157);
}

fn test_example_p2(priorities_table: &BTreeMap<String, u32>) {
    let racksucks = build_racksucks_p2(&priorities_table, &get_example_input());
    assert_eq!(calculate_priority_sum(racksucks), 70);
}

fn get_priority(priorities_table: &BTreeMap<String, u32>, duplicate_item_type: &String) -> u32 {
    let priority: u32 = match priorities_table.get(duplicate_item_type) {
        Some(&priority) => priority,
        _ => 0,
    };
    return priority.clone();
}

fn build_priorities_table() -> BTreeMap<String, u32> {
    let mut table: BTreeMap<String, u32> = BTreeMap::new();
    let mut counter = 0;
    for chr in 'a'..='z' {
        counter += 1;
        table.insert(chr.to_string(), counter);
    }
    for chr in 'A'..='Z' {
        counter += 1;
        table.insert(chr.to_string(), counter);
    }
    return table;
}

fn get_example_input() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    result.push(String::from("vJrwpWtwJgWrhcsFMMfFFhFp"));
    result.push(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    result.push(String::from("PmmdzqPrVvPwwTWBwg"));
    result.push(String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
    result.push(String::from("ttgJtRGJQctTZtZT"));
    result.push(String::from("CrZsJsPPZsGzwwsLwLmpwMDw"));
    return result;
}

fn get_input() -> Vec<String> {
    return load_from_file("src/puzzle_inputs/day03.txt");
}

fn build_racksucks(priorities_table: &BTreeMap<String, u32>, input: &Vec<String>) -> Vec<Racksuck> {
    let mut result: Vec<Racksuck> = Vec::new();
    for line in input {
        let length = line.len();
        let start = 0;
        let middle = length / 2;
        let end = length;

        let compartment_one: String = line[start..middle].to_string();
        let compartment_two: String = line[middle..end].to_string();
        result.push(build_racksuck(
            priorities_table,
            &compartment_one,
            &compartment_two,
        ));
    }
    return result;
}

fn build_racksucks_p2(
    priorities_table: &BTreeMap<String, u32>,
    input: &Vec<String>,
) -> Vec<Racksuck> {
    let mut result: Vec<Racksuck> = Vec::new();
    for (pos, line) in input.iter().enumerate().step_by(3) {
        let group_line1 = line;
        let group_line2 = &input[pos + 1];
        let group_line3 = &input[pos + 2];

        for i in group_line1.chars() {
            if group_line2.contains(i) && group_line3.contains(i) {
                result.push(build_racksuck_p2(priorities_table, &i.to_string()));
                break;
            }
        }
    }
    return result;
}

fn build_racksuck(
    priorities_table: &BTreeMap<String, u32>,
    compartment_one: &String,
    compartment_two: &String,
) -> Racksuck {
    let duplicate_item_type: String = get_duplicate_item_type(&compartment_one, &compartment_two);
    let priority = get_priority(priorities_table, &duplicate_item_type);
    Racksuck { priority }
}

fn build_racksuck_p2(priorities_table: &BTreeMap<String, u32>, badge: &String) -> Racksuck {
    let priority = get_priority(priorities_table, &badge);
    Racksuck { priority }
}

fn get_duplicate_item_type(compartment_one: &String, compartment_two: &String) -> String {
    let mut result: String = String::new();
    for i in compartment_one.chars() {
        for y in compartment_two.chars() {
            if i == y {
                result = i.to_string();
                break;
            }
        }
    }
    return result;
}

fn calculate_priority_sum(racksucks: Vec<Racksuck>) -> u32 {
    let mut result: u32 = 0;
    for racksuck in racksucks {
        result += racksuck.priority;
    }
    return result;
}
