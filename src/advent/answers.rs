use super::calories;
use super::cleaning;
use super::datastream;
use super::directories;
use super::rock_paper_scissors::{self as rps, ParseMode};
use super::rucksack;
use super::supply_crates::{self, Crane};

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
    let rps_score_total = rps::calculate_score_for_file(filename, ParseMode::Choice);
    println!("Part 1: Total score for Rock Paper Scissors = {rps_score_total}");

    let rps_score_total = rps::calculate_score_for_file(filename, ParseMode::Result);
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
    let pairs = cleaning::get_cleaning_pairs("resources/04_cleaning_pairs.txt");

    let count = cleaning::get_count_of_fully_overlapping_pairs(&pairs);
    println!("Part 1: Number of fully overlapping cleaning pairs = {count}");

    let count = cleaning::get_count_of_all_overlapping_pairs(&pairs);
    println!("Part 2: Number of all overlapping cleaning pairs = {count}");
}

pub fn five() {
    let top_crates =
        supply_crates::process_supplies_plan_from_file("resources/05_supplies.txt", Crane::CM9000);
    println!("Part 1: Top crates are {top_crates}");

    let top_crates =
        supply_crates::process_supplies_plan_from_file("resources/05_supplies.txt", Crane::CM9001);
    println!("Part 2: Top crates are {top_crates}");
}

pub fn six() {
    let index = datastream::get_marker_end_index_from_file("resources/06_datastream.txt", 4);
    println!("Part 1: Index of the end of the marker is {index}");

    let index = datastream::get_marker_end_index_from_file("resources/06_datastream.txt", 14);
    println!("Part 2: Index of the end of the marker is {index}");
}

pub fn seven() {
    let sum = directories::get_sum_of_large_directories("resources/07_directories.txt");
    println!("Part 1: Sum of directory sizes is {}", sum);
}
