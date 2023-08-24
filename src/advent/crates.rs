use std::iter;
use std::fmt;
use crate::common::io;
use crate::common::str;

pub fn process_crate_plan_from_file(filename: &str) -> String {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let (mut stks, mv_start) = parse_populate_stacks(&lines).expect("Could not parse and populate stacks");

    parse_apply_move_commands(&lines[mv_start..].to_vec(), &mut stks);

    stks.get_top_of_stacks()
}

fn parse_populate_stacks(lines: &Vec<String>) -> Option<(Stacks, usize)> {
    for (i, l) in lines.iter().enumerate() {
        if l.is_empty() {
            println!("Crate config finished at line {i}");
            let stks = Stacks::create_stacks(lines[0..i - 1].to_vec());
            
            return Some((stks, i + 1));
        }
    }

    None
}

fn parse_apply_move_commands(lines: &Vec<String>, stks: &mut Stacks) {
    for l in lines {
        let mv = Move::from_line(l);
        stks.move_crates(&mv);
    }
}


#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>
}

#[derive(Debug, Eq, PartialEq)]
struct Move {
    n: u8,
    from: usize,
    to: usize
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(move {} from {} to {})", self.n, self.from, self.to)
    }
}

impl Stacks {
    const CRATE_SPACES: usize = 4; // four spaces for each crate in line of file

    fn new(num: usize) -> Stacks {
        let sts = iter::repeat_with(|| Vec::<char>::new())
            .take(num)
            .collect::<Vec<Vec<char>>>();

        Stacks{ stacks: sts }
    }

    fn create_stacks(lines: Vec<String>) -> Stacks {
        let line_len = lines[0].len() + 1;  // plus one space to make line divisible by 4
        let mut stks = Stacks::new(line_len / Stacks::CRATE_SPACES);

        for l in lines.iter().rev() {
            stks.push_line_to_stacks(l);
        }
        
        stks
    }

    fn push_line_to_stacks(&mut self, line: &str) {
        let crate_strs = str::chunk_str(line, Stacks::CRATE_SPACES);

        for (i, s) in crate_strs.iter().enumerate() {
            // chunk will either be empty or of the form "[X] "
            if !s.trim().is_empty() {
                println!("trying to push {} to {}", s, i);
                self.push_to(i, s.chars().nth(1).unwrap());
            }
        }
    }

    fn pop_from(&mut self, num: usize) -> char {
        let stk = self.stacks.get_mut(num).expect("No stack at index");
        stk.pop().expect("Nothing to pop from stack")
    }

    fn push_to(&mut self, num: usize, c: char) {
        let stk = self.stacks.get_mut(num).expect("No stack at index");
        stk.push(c);
    }

    fn move_crates(&mut self, crate_move: &Move) {
        println!("{}", crate_move);

        for _ in 1..=crate_move.n {
            let c = self.pop_from(crate_move.from);
            self.push_to(crate_move.to, c)
        }
    }

    fn get_top_of_stacks(&self) -> String {
        let mut tops = Vec::<char>::new();

        for stk in &self.stacks {
            let top = stk.last().cloned();

            match top {
                Some(c) => tops.push(c),
                None => continue
            }
        }

        tops.iter().collect()
    }
}

impl Move {
    fn from_line(l: &str) -> Move {
        // Example expected format: "move 1 from 2 to 1"
        let m: Vec<&str> = l.split(' ').collect();
        let parse = |n: &str| -> usize { n.parse().expect("Cannot parse to int") };

        // Minus 1 from location numbers to account for vector indexes starting at 0
        Move { 
            n: parse(m[1]) as u8, 
            from: parse(m[3]) - 1, 
            to: parse(m[5]) - 1 
        }
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    fn test_stacks() -> Stacks {
        let mut st = Stacks::new(2);
        let s1 = st.stacks.get_mut(0).unwrap();
        s1.push('A');
        s1.push('C');
        s1.push('C');

        let s2 = st.stacks.get_mut(1).unwrap();
        s2.push('B');

        st
    }

    #[test]
    fn test_move_crates() {
        let mut st = test_stacks();

        // Before move
        assert_eq!(vec!['A', 'C', 'C'], *st.stacks.get(0).unwrap());
        assert_eq!(vec!['B'], *st.stacks.get(1).unwrap());

        let mv = Move { n: 2, from: 0, to: 1 };
        st.move_crates(&mv);

        // After move
        assert_eq!(vec!['A'], *st.stacks.get(0).unwrap());
        assert_eq!(vec!['B', 'C', 'C'], *st.stacks.get(1).unwrap());
    }

    #[test]
    fn test_parse_move() {
        let mv_str = "move 1 from 2 to 1";
        let mv = Move { n: 1, from: 1, to: 0 };
        
        assert_eq!(mv, Move::from_line(mv_str));
    }

    #[test]
    fn test_create_stacks() {
        let lines = vec!["    [D]    ".to_string(), "[N] [C]    ".into(), "[Z] [M] [P]".into()];
        let stks = Stacks::create_stacks(lines);
        let stk_vec = stks.stacks;

        assert_eq!(3, stk_vec.len());
        assert_eq!(vec!['Z', 'N'], *stk_vec.get(0).unwrap());
        assert_eq!(vec!['M', 'C', 'D'], *stk_vec.get(1).unwrap());
        assert_eq!(vec!['P'], *stk_vec.get(2).unwrap());
    }

    #[test]
    fn test_get_tops() {
        let st = test_stacks();
        let tops = st.get_top_of_stacks();

        assert_eq!("CB".to_string(), tops);
    }

    #[test]
    fn test_process_crate_plan_from_file() {
        let top_crates = process_crate_plan_from_file("resources/test/05_crates.txt");
        assert_eq!("CMZ", top_crates);
    }

}