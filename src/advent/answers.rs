use crate::advent::q12_hillwalking;

use super::{
    q01_calories,
    q02_rock_paper_scissors::{self as rps, ParseMode},
    q03_rucksack, q04_cleaning,
    q05_supply_crates::{self, Crane},
    q06_datastream, q07_directories, q08_trees, q09_rope, q10_cathode, q11_monkeys,
};

pub fn one() {
    let elf_calories =
        q01_calories::highest_total_calories_from_file("resources/01_elf_calories.txt");

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
    let rucksacks = q03_rucksack::get_rucksacks_from_file("resources/03_rucksack.txt");

    let priority_sum = q03_rucksack::get_sum_of_priorities_for_common_items(&rucksacks);
    println!("Part 1: Sum of priorities for items = {priority_sum}");

    let group_sum = q03_rucksack::get_sum_of_priorities_for_group(&rucksacks);
    println!("Part 2: Sum of priorities for group items = {group_sum}");
}

pub fn four() {
    let pairs = q04_cleaning::get_cleaning_pairs("resources/04_cleaning_pairs.txt");

    let count = q04_cleaning::get_count_of_fully_overlapping_pairs(&pairs);
    println!("Part 1: Number of fully overlapping cleaning pairs = {count}");

    let count = q04_cleaning::get_count_of_all_overlapping_pairs(&pairs);
    println!("Part 2: Number of all overlapping cleaning pairs = {count}");
}

pub fn five() {
    let top_crates = q05_supply_crates::process_supplies_plan_from_file(
        "resources/05_supplies.txt",
        Crane::CM9000,
    );
    println!("Part 1: Top crates are {top_crates}");

    let top_crates = q05_supply_crates::process_supplies_plan_from_file(
        "resources/05_supplies.txt",
        Crane::CM9001,
    );
    println!("Part 2: Top crates are {top_crates}");
}

pub fn six() {
    let index = q06_datastream::get_marker_end_index_from_file("resources/06_datastream.txt", 4);
    println!("Part 1: Index of the end of the marker is {index}");

    let index = q06_datastream::get_marker_end_index_from_file("resources/06_datastream.txt", 14);
    println!("Part 2: Index of the end of the marker is {index}");
}

pub fn seven() {
    let dirs = q07_directories::parse_directory_sizes_from_file("resources/07_directories.txt");

    let sum = q07_directories::get_sum_of_large_directories(&dirs);
    println!("Part 1: Sum of directory sizes is {sum}");

    let size_smallest = q07_directories::get_size_of_smallest_directory_to_delete(&dirs);
    println!(
        "Part 2: Size of smallest directory we can delete to free space is {}",
        size_smallest
    );
}

pub fn eight() {
    let grid = q08_trees::create_trees_grid_from_file("resources/08_trees.txt");
    let result = q08_trees::find_visible_trees(&grid);
    println!("Part 1: Number of visible trees is {result}");

    let result = q08_trees::find_highest_scenic_score(&grid);
    println!("Part 2: Highest scenic score is {result}");
}

pub fn nine() {
    let result = q09_rope::get_number_of_spaces_visited("resources/09_rope.txt", 2);
    println!("Part 1: Number of positions visited by tail is {result}");

    let result = q09_rope::get_number_of_spaces_visited("resources/09_rope.txt", 10);
    println!("Part 2: Number of positions visited by tail is {result}");
}

pub fn ten() {
    let instructions = q10_cathode::parse_instructions("resources/10_cathode.txt");
    let result = q10_cathode::get_sum_of_signal_strengths(&instructions);
    println!("Part 1: Sum of six signal strengths is {result}");

    println!("Part 2: Print of screen:");
    q10_cathode::print_to_screen(&instructions);
}

pub fn eleven() {
    let result = q11_monkeys::get_monkey_business_part1("resources/11_monkeys.txt");
    println!("Part 1: Monkey business score for top 2 monkeys is {result}");

    let result = q11_monkeys::get_monkey_business_part2("resources/11_monkeys.txt");
    println!("Part 2: Monkey business score for top 2 monkeys is {result}");
}

pub fn twelve() {
    let result = q12_hillwalking::get_smallest_distance("resources/12_hillwalking.txt");
    println!("Part 1: Fewest steps required is {result}");
}
