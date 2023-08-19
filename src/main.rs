#![allow(dead_code)]

use advent::answers;
use std::io;

mod advent;
mod util;

fn main() {
    loop {
        prompt_user_for_choice();
    }
}

fn prompt_user_for_choice() {
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

    print!("\n");
}

fn run_fn_for_exercise(n: u8) {
    match n {
        1 => answers::one(),
        2 => answers::two(),
        3 => answers::three(),
        4 => answers::four(),
        _ => println!("Exercise not yet implemented."),
    }
}
