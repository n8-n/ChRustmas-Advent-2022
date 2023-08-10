use super::calories::*;
use super::rock_paper_scissors::calculate_score_for_file;

pub fn one() {
    let elf_calories = highest_total_calories_from_file("resources/elf_calories.txt");

    let length = elf_calories.len();

    // Part 1
    println!("Highest total calories = {}", elf_calories[length - 1]);

    // Part 2
    let top3: &[u32] = &elf_calories[length - 3..length];
    println!("Sum of highest 3 calories = {}", top3.iter().sum::<u32>());
}

pub fn two() {
    let rps_score_total = calculate_score_for_file("resources/rock_paper_scissors.txt");
    println!("Total score for Rock Paper Scissors = {rps_score_total}");
}