use super::calories::*;
use super::rock_paper_scissors::{calculate_score_for_file, ParseMode};

pub fn one() {
    let elf_calories = highest_total_calories_from_file("resources/01_elf_calories.txt");

    let length = elf_calories.len();

    // Part 1
    println!("Highest total calories = {}", elf_calories[length - 1]);

    // Part 2
    let top3: &[u32] = &elf_calories[length - 3..length];
    println!("Sum of highest 3 calories = {}", top3.iter().sum::<u32>());
}

pub fn two() {
    let filename = "resources/02_rock_paper_scissors.txt";
    let rps_score_total = calculate_score_for_file(filename, &ParseMode::Choice);
    println!("Total score for Rock Paper Scissors = {rps_score_total}");

    let rps_score_total = calculate_score_for_file(filename, &ParseMode::Result);
    println!("Total score for Rock Paper Scissors part 2 = {rps_score_total}");
}

pub fn three() {
    let priority_sum = "not yet implemented";
    println!("Sum of priorities for items is {priority_sum}");
}
