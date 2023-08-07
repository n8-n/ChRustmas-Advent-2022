use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn highest_total_calories() {
    let filename = "resources/elf_calories.txt";
    let file = File::open(filename).expect("Cannot read {filename}");

    let reader = BufReader::new(file);

    let mut elf_calories: Vec<u32> = Vec::new();
    let mut cal_add: u32 = 0;

    for line in reader.lines() {
        let l = line.unwrap();

        if l.is_empty() {
            elf_calories.push(cal_add);
            cal_add = 0;
        } else {
            let cal: u32 = l.trim().parse().expect("Cannot parse line as number");
            cal_add += cal;
        }
    }

    match elf_calories.iter().max() {
        Some(max) => println!("Highest total calories = {max}"),
        None => println!("Cannot find max of vector"),
    }
}
