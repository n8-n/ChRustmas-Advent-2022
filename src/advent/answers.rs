use super::calories::*;

pub fn one() {
    let elf_calories = highest_total_calories_from_file("resources/elf_calories.txt");

    let length = elf_calories.len();

    // Part 1
    println!("Highest total calories = {}", elf_calories[length - 1]);

    // Part 2
    let top3: &[u32] = &elf_calories[length-3..length];
    println!("Sum of highest 3 calories = {}", top3.iter().sum::<u32>());
}
