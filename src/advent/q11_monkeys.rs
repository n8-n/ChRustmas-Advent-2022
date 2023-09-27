use crate::common::io;
use std::num::ParseIntError;

type Monction = Box<dyn Fn(u64) -> u64>; // Monkey function

pub fn get_monkey_business_part1(filename: &str) -> u64 {
    let mut monkeys = parse_file_into_monkeys(&filename);
    let worry_handler: Monction = Box::new(|worry: u64| (worry as f32 / 3.0).floor() as u64);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkey_takes_turn(i, &mut monkeys, &worry_handler)
        }
    }

    monkey_business_value(&mut monkeys)
}

pub fn get_monkey_business_part2(filename: &str) -> u64 {
    let mut monkeys = parse_file_into_monkeys(&filename);
    let product_of_divisors: u64 = monkeys.iter().map(|m| m.test.divisible).product();
    let worry_handler: Monction = Box::new(move |worry: u64| worry % product_of_divisors);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkey_takes_turn(i, &mut monkeys, &worry_handler)
        }
    }

    monkey_business_value(&mut monkeys)
}

fn monkey_business_value(monkeys: &mut Vec<Monkey>) -> u64 {
    monkeys.sort_by_key(|m| m.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections as u64)
        .product::<u64>()
}

fn parse_file_into_monkeys(filename: &str) -> Vec<Monkey> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    lines
        .chunks(7)
        .map(|monkey_lines| Monkey::from_lines(monkey_lines))
        .collect()
}

fn monkey_takes_turn(monkey_index: usize, monkeys: &mut Vec<Monkey>, worry_handler: &Monction) {
    let monkey = &mut monkeys[monkey_index];
    let items = &monkey.items;
    let mut monkey_moves: Vec<(usize, u64)> = Vec::new();

    if items.is_empty() {
        return;
    }

    for item in items {
        monkey.inspections += 1;
        let operation_worry = (monkey.operation)(*item);
        let handled_worry = worry_handler(operation_worry);
        let throw_monkey = monkey.test(handled_worry);

        // temporarily store the item moves because we can't borrow mutably multiple times
        monkey_moves.push((throw_monkey, handled_worry));
    }

    monkey.items = Vec::new();

    for (i, item) in monkey_moves {
        let monkey = &mut monkeys[i];
        monkey.items.push(item);
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Monction,
    test: MonkeyTest,
    inspections: u64,
}

struct MonkeyTest {
    divisible: u64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn from_lines(lines: &[String]) -> Monkey {
        Monkey {
            items: parse_starting_items(&lines[1]),
            operation: parse_operation(&lines[2]).expect("Should parse operation"),
            test: parse_monkey_test(&lines[3..=5]).expect("Should parse test"),
            inspections: 0,
        }
    }

    fn test(&self, item: u64) -> usize {
        let test = &self.test;

        if item % test.divisible == 0 {
            test.if_true
        } else {
            test.if_false
        }
    }
}

fn parse_starting_items(items: &str) -> Vec<u64> {
    items
        .trim()
        .split(' ')
        .skip(2)
        .map(|item| item.trim_end_matches(","))
        .flat_map(|item| item.parse())
        .collect()
}

fn parse_operation(operation: &str) -> Result<Monction, &'static str> {
    let split: Vec<&str> = operation.trim().split(' ').collect();

    if split.len() != 6 {
        return Err("Cannot parse operation line; not enough tokens.");
    }

    let (operand, op_value) = (split[4], split[5]);
    let op_value_int: Result<u64, ParseIntError> = op_value.parse();

    let func: Monction = match (operand, op_value) {
        ("*", "old") => Box::new(|x: u64| x * x),
        ("+", "old") => Box::new(|x: u64| x + x),
        ("*", _) => Box::new(move |x: u64| x * op_value_int.as_ref().unwrap()),
        ("+", _) => Box::new(move |x: u64| x + op_value_int.as_ref().unwrap()),
        (_, _) => return Err("Invalid function operand"),
    };

    Ok(func)
}

fn parse_monkey_test(test: &[String]) -> Result<MonkeyTest, &'static str> {
    let test_line: Vec<&str> = test[0].trim().split(' ').collect();
    let true_line: Vec<&str> = test[1].trim().split(' ').collect();
    let false_line: Vec<&str> = test[2].trim().split(' ').collect();

    if (test_line.len() != 4) || true_line.len() != 6 || false_line.len() != 6 {
        return Err("Cannot parse test lines; not enough tokens.");
    }

    let divisible: u64 = test_line[3].parse().expect("Should parse int");
    let true_monkey: usize = true_line[5].parse().expect("Should parse int");
    let false_monkey: usize = false_line[5].parse().expect("Should parse int");

    Ok(MonkeyTest {
        divisible: divisible,
        if_true: true_monkey,
        if_false: false_monkey,
    })
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
        let test = ["Test: divisible by 13".to_string(), 
            "If true: throw to monkey 1".into(), "If false: throw to monkey 3".into()];
        let test = parse_monkey_test(&test).unwrap();
        assert_eq!(1, test.if_true);
        assert_eq!(3, test.if_false);
        assert_eq!(13, test.divisible);
    }

    #[test]
    fn test_parse_items() {
        let result = parse_starting_items(&"Starting items: 79, 60, 97");
        assert_eq!(vec![79, 60, 97], result);
        let result = parse_starting_items(&"Starting items: 97");
        assert_eq!(vec![97], result);
    }

    #[test]
    fn test_parse_file_into_monkeys() {
        let result = parse_file_into_monkeys("resources/test/11_monkeys.txt");
        assert_eq!(4, result.len());
        let first = &result[0];
        assert_eq!(vec![79, 98], first.items);
    }

    #[test]
    fn test_get_monkey_business() {
        assert_eq!(10605, get_monkey_business_part1("resources/test/11_monkeys.txt"));
        assert_eq!(2713310158, get_monkey_business_part2("resources/test/11_monkeys.txt"));
    }
}
