use std::{error::Error, io, process};

pub mod answers;
mod q01_calories;
mod q02_rock_paper_scissors;
mod q03_rucksack;
mod q04_cleaning;
mod q05_supply_crates;
mod q06_datastream;
mod q07_directories;

pub fn run() {
    loop {
        if let Err(e) = prompt_user_for_choice() {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}

fn prompt_user_for_choice() -> Result<(), Box<dyn Error>> {
    println!("Which exercise answer would you like? [1-25]");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let choice: u8 = choice.trim().parse()?;

    if choice < 1 || choice > 25 {
        println!("Not a valid exercise number. Should be 1-25.");
    } else {
        run_fn_for_exercise(choice);
    }

    print!("\n");

    Ok(())
}

fn run_fn_for_exercise(n: u8) {
    match n {
        1 => answers::one(),
        2 => answers::two(),
        3 => answers::three(),
        4 => answers::four(),
        5 => answers::five(),
        6 => answers::six(),
        7 => answers::seven(),
        _ => println!("Exercise not yet implemented."),
    }
}
