#![allow(dead_code)]

use std::io;

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
        println!("{:?}", choice);
    }
}
