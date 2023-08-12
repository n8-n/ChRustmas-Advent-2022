use super::calories::*;
use super::rock_paper_scissors::*;
use super::rucksack::*;

pub fn one() {
    let elf_calories = highest_total_calories_from_file("resources/01_elf_calories.txt");

    let length = elf_calories.len();

    println!(
        "Part 1: Highest total calories = {}",
        elf_calories[length - 1]
    );

    let top3: &[u32] = &elf_calories[length - 3..length];
    println!(
        "Part 2: Sum of highest 3 calories = {}",
        top3.iter().sum::<u32>()
    );
}

pub fn two() {
    let filename = "resources/02_rock_paper_scissors.txt";
    let rps_score_total = calculate_score_for_file(filename, &ParseMode::Choice);
    println!("Part 1: Total score for Rock Paper Scissors = {rps_score_total}");

    let rps_score_total = calculate_score_for_file(filename, &ParseMode::Result);
    println!("Part 2: Total score for Rock Paper Scissors = {rps_score_total}");
}

pub fn three() {
    let priority_sum = get_sum_of_priorities_for_common_items("resources/03_rucksack.txt");
    println!("Sum of priorities for items is {priority_sum}");
}
