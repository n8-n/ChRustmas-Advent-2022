use core::num;

use crate::common::{grid::Grid, io};

pub fn get_number_of_visible_trees(filename: &str) -> u32 {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let grid = create_trees_grid(lines);

    0
}

fn create_trees_grid(lines: Vec<String>) -> Grid<u8> {
    let mut grid = Grid::new();

    lines.iter()
        .map(string_to_numbers)
        .for_each(|nums| grid.add_row(nums));

    grid
}

fn string_to_numbers(line: &String) -> Vec<u8> {
    return line.chars()
        .map(|c: char| c.to_digit(10))
        .flatten()
        .map(|n| n as u8)
        .collect();
}

//
//
//
#[cfg(test)]
// #[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let lines = io::read_file_as_vector("resources/test/08_trees.txt").expect("Could not read file");
        let grid = create_trees_grid(lines);

        println!("{}", grid);
    }
}
