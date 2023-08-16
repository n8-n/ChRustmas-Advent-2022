use super::calories;
use super::rock_paper_scissors as rps;
use super::rock_paper_scissors::ParseMode;
use super::rucksack;

pub fn one() {
    let elf_calories = calories::highest_total_calories_from_file("resources/01_elf_calories.txt");

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
    let rps_score_total = rps::calculate_score_for_file(filename, &ParseMode::Choice);
    println!("Part 1: Total score for Rock Paper Scissors = {rps_score_total}");

    let rps_score_total = rps::calculate_score_for_file(filename, &ParseMode::Result);
    println!("Part 2: Total score for Rock Paper Scissors = {rps_score_total}");
}

pub fn three() {
    let rucksacks = rucksack::get_rucksacks_from_file("resources/03_rucksack.txt");

    let priority_sum = rucksack::get_sum_of_priorities_for_common_items(&rucksacks);
    println!("Part 1: Sum of priorities for items = {priority_sum}");

    let group_sum = rucksack::get_sum_of_priorities_for_group(&rucksacks);
    println!("Part 2: Sum of priorities for group items = {group_sum}");
}

pub fn four() {
    let answer = "blah";
    println!("Part 1: number of overlapping cleaning pairs = {:?}", answer);
}