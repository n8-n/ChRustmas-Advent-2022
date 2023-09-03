use crate::common::io;
use crate::common::str;
use std::fmt;
use std::iter;

pub enum Crane {
    CM9000,
    CM9001,
}

pub fn process_supplies_plan_from_file(filename: &str, crane_type: Crane) -> String {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let (mut supplies, move_start_line) =
        parse_populate_supply_stacks(&lines).expect("Could not parse supply crates");

    parse_apply_move_commands(
        &lines[move_start_line..].to_vec(),
        &mut supplies,
        crane_type,
    );

    supplies.get_top_of_stacks()
}

fn parse_populate_supply_stacks(lines: &Vec<String>) -> Option<(Supplies, usize)> {
    for (i, l) in lines.iter().enumerate() {
        if l.is_empty() {
            let stacks = Supplies::create_supply_stacks(lines[0..i - 1].to_vec());
            return Some((stacks, i + 1));
        }
    }

    None
}

fn parse_apply_move_commands(lines: &Vec<String>, stacks: &mut Supplies, crane_type: Crane) {
    for l in lines {
        let mv = Move::from_line(l);
        stacks.move_crates(mv, &crane_type);
    }
}

#[derive(Debug)]
struct Supplies {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Move {
    amount: u8,
    from: usize,
    to: usize,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(move {} from {} to {})",
            self.amount, self.from, self.to
        )
    }
}

impl Supplies {
    const CRATE_SPACES: usize = 4; // four spaces for each crate in line of file

    fn new(num: usize) -> Supplies {
        let sts = iter::repeat_with(|| Vec::<char>::new())
            .take(num)
            .collect::<Vec<Vec<char>>>();

        Supplies { stacks: sts }
    }

    fn create_supply_stacks(lines: Vec<String>) -> Supplies {
        let line_len = lines[0].len() + 1; // plus one space to make line divisible by 4
        let mut supplies = Supplies::new(line_len / Supplies::CRATE_SPACES);

        for l in lines.iter().rev() {
            supplies.push_line_to_stacks(l);
        }

        supplies
    }

    fn push_line_to_stacks(&mut self, line: &str) {
        let crate_strs = str::chunk_string(line, Supplies::CRATE_SPACES);

        for (i, s) in crate_strs.iter().enumerate() {
            // chunk will either be empty or of the form "[X] "
            if !s.trim().is_empty() {
                self.push_to(i, s.chars().nth(1).unwrap());
            }
        }
    }

    fn pop_from(&mut self, num: usize) -> char {
        let stack = self.stacks.get_mut(num).expect("No stack at index");
        stack.pop().expect("Nothing to pop from stack")
    }

    fn push_to(&mut self, num: usize, c: char) {
        let stack = self.stacks.get_mut(num).expect("No stack at index");
        stack.push(c);
    }

    fn push_multiple(&mut self, num: usize, chars: Vec<char>) {
        let stack = self.stacks.get_mut(num).expect("No stack at index");
        stack.extend(chars);
    }

    fn move_crates(&mut self, crate_move: Move, crane_type: &Crane) {
        match crane_type {
            Crane::CM9000 => {
                for _ in 1..=crate_move.amount {
                    let c = self.pop_from(crate_move.from);
                    self.push_to(crate_move.to, c)
                }
            }
            Crane::CM9001 => {
                let mut chars = Vec::<char>::new();
                for _ in 1..=crate_move.amount {
                    let c = self.pop_from(crate_move.from);
                    chars.push(c);
                }
                chars.reverse();
                self.push_multiple(crate_move.to, chars);
            }
        }
    }

    fn get_top_of_stacks(&self) -> String {
        let mut tops = Vec::<char>::new();

        for stk in &self.stacks {
            let top = stk.last().cloned();

            match top {
                Some(c) => tops.push(c),
                None => continue,
            }
        }

        tops.iter().collect()
    }
}

impl Move {
    fn from_line(l: &str) -> Move {
        // Example expected format: "move 1 from 2 to 1"
        let line_split: Vec<&str> = l.split(' ').collect();
        let parse = |n: &str| -> usize { n.parse().expect("Cannot parse to int") };

        // Minus 1 from location numbers to account for vector indexes starting at 0
        Move {
            amount: parse(line_split[1]) as u8,
            from: parse(line_split[3]) - 1,
            to: parse(line_split[5]) - 1,
        }
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    fn test_stacks() -> Supplies {
        let mut st = Supplies::new(2);
        let s1 = st.stacks.get_mut(0).unwrap();
        s1.push('A');
        s1.push('C');
        s1.push('D');

        let s2 = st.stacks.get_mut(1).unwrap();
        s2.push('B');

        st
    }

    #[test]
    fn test_move_crates_cm9000() {
        let mut st = test_stacks();

        // Before move
        assert_eq!(vec!['A', 'C', 'D'], *st.stacks.get(0).unwrap());
        assert_eq!(vec!['B'], *st.stacks.get(1).unwrap());

        let mv = Move {
            amount: 2,
            from: 0,
            to: 1,
        };
        st.move_crates(mv, &Crane::CM9000);

        // After move
        assert_eq!(vec!['A'], *st.stacks.get(0).unwrap());
        assert_eq!(vec!['B', 'D', 'C'], *st.stacks.get(1).unwrap());
    }

    #[test]
    fn test_move_crates_cm9001() {
        let mut st = test_stacks();

        // Before move
        assert_eq!(vec!['A', 'C', 'D'], *st.stacks.get(0).unwrap());
        assert_eq!(vec!['B'], *st.stacks.get(1).unwrap());

        let mv = Move {
            amount: 2,
            from: 0,
            to: 1,
        };
        st.move_crates(mv, &Crane::CM9001);

        // After move
        assert_eq!(vec!['A'], *st.stacks.get(0).unwrap());
        assert_eq!(vec!['B', 'C', 'D'], *st.stacks.get(1).unwrap());
    }

    #[test]
    fn test_parse_move() {
        let mv_str = "move 1 from 2 to 1";
        let mv = Move {
            amount: 1,
            from: 1,
            to: 0,
        };

        assert_eq!(mv, Move::from_line(mv_str));
    }

    #[test]
    fn test_create_stacks() {
        let lines = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".into(),
            "[Z] [M] [P]".into(),
        ];
        let stacks = Supplies::create_supply_stacks(lines);
        let stack_vector = stacks.stacks;

        assert_eq!(3, stack_vector.len());
        assert_eq!(vec!['Z', 'N'], *stack_vector.get(0).unwrap());
        assert_eq!(vec!['M', 'C', 'D'], *stack_vector.get(1).unwrap());
        assert_eq!(vec!['P'], *stack_vector.get(2).unwrap());
    }

    #[test]
    fn test_get_tops() {
        let st = test_stacks();
        let tops = st.get_top_of_stacks();

        assert_eq!("DB".to_string(), tops);
    }

    #[test]
    fn test_process_supplies_plan_from_file() {
        let top_crates =
            process_supplies_plan_from_file("resources/test/05_supplies.txt", Crane::CM9000);
        assert_eq!("CMZ", top_crates);

        let top_crates =
            process_supplies_plan_from_file("resources/test/05_supplies.txt", Crane::CM9001);
        assert_eq!("MCD", top_crates);
    }
}
