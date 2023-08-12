#![allow(dead_code)]

use std::io;

mod advent;

fn main() {
    println!("Which exercise answer would you like? [1-25]");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line");

    let choice: u8 = choice
        .trim()
        .parse()
        .expect("Not a number, or number is too large.");

    if choice < 1 || choice > 25 {
        println!("Not a valid exercise number. Should be 1-25.");
    } else {
        run_fn_for_exercise(choice);
    }
}

fn run_fn_for_exercise(n: u8) {
    use advent::answers::*;

    match n {
        1 => one(),
        2 => two(),
        3 => three(),
        _ => println!("Exercise not yet implemented."),
    }
}
