use crate::spoilers;
use crate::utils::load_from_file;

pub fn solve_day() {
    println!("--- Day 1: Calorie Counting ---");
    let calories: Vec<String> = load_from_file("src/puzzle_inputs/day01.txt");
    let mut group_sum: usize = 0;
    let mut elf_calories: usize;
    let mut elf_calory_sums: Vec<usize> = Vec::new();
    for (_pos, e) in calories.iter().enumerate() {
        if e.is_empty() {
            elf_calory_sums.push(group_sum);
            group_sum = 0;
        } else {
            elf_calories = e.parse::<usize>().unwrap();
            group_sum += elf_calories;
        }
    }
    let max_total_calories: _ = elf_calory_sums.iter().max().unwrap();
    assert_eq!(max_total_calories, &spoilers::d01());
    println!("  Part one: {}", max_total_calories);
    elf_calory_sums.sort_by(|a, b| b.cmp(a));
    let total: usize = elf_calory_sums[0] + elf_calory_sums[1] + elf_calory_sums[2];
    assert_eq!(total, spoilers::d01_p2());
    println!("  Part two: {}", total);
}
