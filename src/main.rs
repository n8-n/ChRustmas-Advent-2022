#![allow(dead_code)]

use advent::one;
use std::io;

pub mod advent;

fn main() {
    println!("Which exercise answer would you like? [1-25]");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line");

    let choice: u32 = choice.trim().parse().expect("Not a number!");

    if choice < 1 || choice > 25 {
        println!("Not a valid exercise number. Should be 1-25.");
    } else {
        run_fn_for_exercise(choice);
    }
}

fn run_fn_for_exercise(n: u32) {
    match n {
        1 => one::highest_total_calories(),
        _ => println!("Exercise not yet implemented."),
    }
}
