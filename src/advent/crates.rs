use std::iter;
use std::fmt;
use crate::util::io;

fn read_crate_stack_plan_from_file(filename: &str) {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    for (i, l) in lines.iter().enumerate() {
        if l.is_empty() {
            println!("Crate config finished at line {i}");
        }
    }
}


#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>
}

#[derive(Debug)]
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
    fn new(num: usize) -> Stacks {
        let sts = iter::repeat_with(|| Vec::<char>::new())
            .take(num)
            .collect::<Vec<Vec<char>>>();

        Stacks{ stacks: sts }
    }

    // fn populate_stacks(lines: Vec<String>) -> Stacks {
        // let stk_num = lines.get(0).unwrap().chunks(3)
        // let stks = Stacks::new(num)

        // need to split into chunks of 4 characters
    // }

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
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_crates() {
        let mut st = Stacks::new(2);
        let s1 = st.stacks.get_mut(0).unwrap();
        s1.push('A');
        s1.push('C');
        s1.push('C');

        let s2 = st.stacks.get_mut(1).unwrap();
        s2.push('B');

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
    fn test_crate_stack_plan() {
        read_crate_stack_plan_from_file("resources/test/05_crates.txt");
    }

}