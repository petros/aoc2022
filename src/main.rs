use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    solve_day_01();
}

fn solve_day_01() {
    println!("--- Day 1: Calorie Counting ---");
    let calories: Vec<String> = load_from_file("src/01a-puzzle-input.txt");
    let mut group_sum: u32 = 0;
    let mut elf_calories: u32;
    let mut elf_calory_sums: Vec<u32> = Vec::new();
    for (_pos, e) in calories.iter().enumerate() {
        if e == "" {
            elf_calory_sums.push(group_sum);
            group_sum = 0;
        } else {
            elf_calories = e.parse::<u32>().unwrap();
            group_sum += elf_calories;
        }
    }
    let max_total_calories: &u32 = elf_calory_sums.iter().max().unwrap();
    println!("The total calories of the Elf carrying the most calories are {max_total_calories}");
    println!("--- Part Two ---");
    elf_calory_sums.sort_by(|a, b| b.cmp(a));
    let total: u32 = elf_calory_sums[0] + elf_calory_sums[1] + elf_calory_sums[2];
    println!("The top three most calories carrying Elfs have a total of {total} calories");
    println!();
}

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();
    return numbers;
}
