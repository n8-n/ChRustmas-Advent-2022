use std::{num::ParseIntError};

use crate::common::io;

type Monction = Box<dyn Fn(u32) -> u32>; // Monkey Function

pub fn get_monkey_business(filename: &str) -> u32 {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    0
}

struct Monkey {
    items: Vec<u32>,
    operation: Monction,
    test: Monction
}

impl Monkey {
    fn from_lines(lines: [&str; 7]) -> Monkey {
        Monkey {
            items: parse_starting_items(lines[1]),
            operation: parse_operation(lines[2]).expect("Should parse operation"),
            test: parse_monkey_test(&lines[3..=5]).expect("Should parse test")
        }
    }
}

fn parse_starting_items(items: &str) -> Vec<u32> {
    items.split(' ')
        .skip(2)
        .map(|item| item.trim_end_matches(","))
        .flat_map(|item| item.parse())
        .collect()
}

fn parse_operation(operation: &str) -> Result<Monction, &'static str> {
    let split: Vec<&str> = operation.split(' ').collect();

    if split.len() != 6 {
        return Err("Cannot parse operation line; not enough tokens.");
    }

    let (operand, op_value) = (split[4], split[5]);
    let op_value_int: Result<u32, ParseIntError> = op_value.parse();

    let func: Monction = match (operand, op_value) {
        ("*", "old") => Box::new(|x: u32| x * x),
        ("+", "old") => Box::new(|x: u32| x + x),
        ("*", _) => Box::new(move |x: u32| x * op_value_int.as_ref().unwrap()),
        ("+", _) => Box::new(move |x: u32| x + op_value_int.as_ref().unwrap()),
        (_, _) => return Err("Invalid function operand")
    };

    Ok(func)
}

fn parse_monkey_test(test: &[&str]) -> Result<Monction, &'static str> {
    let test_line: Vec<&str> = test[0].split(' ').collect();
    let true_line: Vec<&str> = test[1].split(' ').collect();
    let false_line: Vec<&str> = test[2].split(' ').collect();

    if (test_line.len() != 4) || true_line.len() != 6 || false_line.len() != 6 {
        return Err("Cannot parse test lines; not enough tokens.");
    }

    let divisible: u32 = test_line[3].parse().expect("Should parse int");
    let true_monkey: u32 = true_line[5].parse().expect("Should parse int");
    let false_monkey: u32 = false_line[5].parse().expect("Should parse int");

    Ok(Box::new(move |item: u32|  {
        if item % divisible == 0 {
            true_monkey
        } else {
            false_monkey
        }
    }))
}

//
//
//
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        let func = parse_operation(&"Operation: new = old + 6").unwrap();
        assert_eq!(12, func(6));
        let func = parse_operation(&"Operation: new = old * 3").unwrap();
        assert_eq!(12, func(4));
        let func = parse_operation(&"Operation: new = old * old").unwrap();
        assert_eq!(16, func(4));
    }

    #[test]
    fn test_parse_monkey_test() {
        let test = ["Test: divisible by 13", "If true: throw to monkey 1", "If false: throw to monkey 3"];
        let func = parse_monkey_test(&test).unwrap();
        assert_eq!(1, func(26));
        assert_eq!(3, func(22));
    }

    #[test]
    fn test_parse_items() {
        let result = parse_starting_items(&"Starting items: 79, 60, 97");
        assert_eq!(vec![79, 60, 97], result);
        let result = parse_starting_items(&"Starting items: 97");
        assert_eq!(vec![97], result);
    }
}