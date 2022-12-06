use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Solving puzzle 01a...");
    solve_puzzle_01a();
}

// Open the text file and read it into a vector of strings — DONE
// Print the contents — DONE
// Create a vector of integers that holds the sums of each Elf — DONE
// Find the largerst number in that vector
fn solve_puzzle_01a() {
    let calories: Vec<String> = load_from_file("src/01a-puzzle-input.txt");
    let mut group_sum: u32 = 0;
    let mut elf_calories: u32;
    let mut elf_calory_sums: Vec<u32> = Vec::new();
    for (pos, e) in calories.iter().enumerate() {
        println!("{}: {:?}", pos, e);
        if e == "" {
            elf_calory_sums.push(group_sum);
            println!("Sum {group_sum}");
            group_sum = 0;
        } else {
            elf_calories = e.parse::<u32>().unwrap();
            group_sum += elf_calories;
        }
    }
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
