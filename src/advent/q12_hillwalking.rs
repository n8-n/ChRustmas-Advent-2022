use std::fmt::Display;

use crate::common::{grid::Grid, io};

pub fn get_smallest_distance(filename: &str) -> u32 {
    0
}

fn read_file_into_grid(filename: &str) -> Grid<Node> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let mut grid: Grid<Node> = Grid::new();

    lines
        .iter()
        .map(|l| Node::from_line(l))
        .for_each(|nodes| grid.add_row(nodes));

    grid
}

#[derive(Clone, Debug)]
struct Node {
    elevation: char,
    visited: bool,
}

impl Node {
    pub fn from_char(c: char) -> Self {
        Node {
            elevation: c,
            visited: false,
        }
    }

    fn from_line(line: &str) -> Vec<Self> {
        line.chars().map(|c| Self::from_char(c)).collect()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elevation)
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let grid = read_file_into_grid("resources/test/12_hillwalking.txt");
        println!("{grid}");
    }
}
