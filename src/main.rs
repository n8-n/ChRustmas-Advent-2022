#![allow(dead_code)]

use advent::answers;
use std::io;

mod advent;

fn main() {
    println!("Which exercise answer would you like? [1-25]");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line");

    let choice: i8 = choice.trim().parse().expect("Input is not a number, or is too large.");

    if choice < 1 || choice > 25 {
        println!("Not a valid exercise number. Should be 1-25.");
    } else {
        run_fn_for_exercise(choice);
    }
}

fn run_fn_for_exercise(n: i8) {
    match n {
        1 => answers::one(),
        2 => answers::two(),
        _ => println!("Exercise not yet implemented."),
    }
}
