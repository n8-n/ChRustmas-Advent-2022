use std::iter::{Enumerate, Skip};
use std::slice::{Chunks, Iter};

use crate::common::{grid::Grid, io};

pub fn create_trees_grid_from_file(filename: &str) -> Grid<u8> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let mut grid = Grid::new();

    lines
        .iter()
        .map(string_to_numbers)
        .for_each(|nums| grid.add_row(nums));

    grid
}

pub fn find_visible_trees(grid: &Grid<u8>) -> usize {
    // trees on the edge are visible. minus 4 for double-counted corners.
    // we will need to ignore these when iterating through the grid.
    let mut visible = (grid.rows * 2) + (grid.columns * 2) - 4;

    enumerator_over_inner_rows(grid).for_each(|(row_index, row)| {
        iterator_over_inner_columns(row)
            .filter(|(column_index, tree_height)| {
                is_tree_visible(**tree_height, (row_index, row), *column_index, &grid)
            })
            .for_each(|_| visible += 1);
    });

    visible
}

pub fn find_highest_scenic_score(grid: &Grid<u8>) -> usize {
    // we'll ignore the edge tree rows

    enumerator_over_inner_rows(grid)
        .map(|(row_index, row)| {
            iterator_over_inner_columns(row)
                .map(|(column_index, tree_height)| {
                    let column = grid.get_column(column_index).expect("Should have column");
                    get_scenic_score(*tree_height, (row_index, row), (column_index, &column))
                })
                .max()
        })
        .flatten()
        .max()
        .unwrap()
}

fn enumerator_over_inner_rows(grid: &Grid<u8>) -> Skip<Enumerate<Chunks<'_, u8>>> {
    let end = grid.elements.len() - grid.columns;
    grid.elements[..end]
        .chunks(grid.columns)
        .enumerate()
        .skip(1)
}

fn iterator_over_inner_columns(row: &[u8]) -> Skip<Enumerate<Iter<'_, u8>>> {
    let end_row = row.len() - 1;
    row[..end_row].iter().enumerate().skip(1)
}

fn string_to_numbers(line: &String) -> Vec<u8> {
    return line
        .chars()
        .map(|c: char| c.to_digit(10))
        .flatten()
        .map(|n| n as u8)
        .collect();
}

fn is_tree_visible(height: u8, row: (usize, &[u8]), column_index: usize, grid: &Grid<u8>) -> bool {
    let (row_index, row_values) = row;

    let row_left = height_check(height, &row_values[..column_index]);
    if row_left.is_none() {
        return true;
    }

    let row_right = height_check(height, &row_values[(column_index + 1)..]);
    if row_right.is_none() {
        return true;
    }

    let column = grid.get_column(column_index).expect("Should have column");
    let column_up = height_check(height, &column[..row_index]);
    if column_up.is_none() {
        return true;
    }

    let column_down = height_check(height, &column[(row_index + 1)..]);
    column_down.is_none()
}

fn height_check(height: u8, segment: &[u8]) -> Option<&u8> {
    segment.iter().find(|tree: &&u8| height <= **tree)
}

fn get_scenic_score(height: u8, row: (usize, &[u8]), column: (usize, &[u8])) -> usize {
    let (row_index, row_values) = row;
    let (column_index, column_values) = column;

    let row_left = view_len_rev(height, &row_values[..column_index]);
    let row_right = view_len(height, &row_values[(column_index + 1)..]);
    let column_up = view_len_rev(height, &column_values[..row_index]);
    let column_down = view_len(height, &column_values[(row_index + 1)..]);

    row_left * row_right * column_up * column_down
}

fn view_len(height: u8, segment: &[u8]) -> usize {
    view_len_iterate(height, segment.iter())
}

fn view_len_rev(height: u8, segment: &[u8]) -> usize {
    view_len_iterate(height, segment.iter().rev())
}


fn view_len_iterate<'a, I>(height: u8, iter: I) -> usize
where I: Iterator<Item = &'a u8> {
    let mut len = 0;
    for tree in iter {
        len += 1;
        if height <= *tree { break; }
    }
    len
}

//
//
//
#[cfg(test)]
// #[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_numbers() {
        let input = "71824".to_string();
        let result = string_to_numbers(&input);
        assert_eq!(vec![7, 1, 8, 2, 4], result)
    }

    #[test]
    fn test_get_visible_trees_from_file() {
        let trees_grid = create_trees_grid_from_file("resources/test/08_trees.txt");
        let trees = find_visible_trees(&trees_grid);
        assert_eq!(21, trees);
    }

    #[test]
    fn test_get_max_scenic_score_from_file() {
        let trees_grid = create_trees_grid_from_file("resources/test/08_trees.txt");
        let trees = find_highest_scenic_score(&trees_grid);
        assert_eq!(8, trees);
    }

    #[test]
    fn test_view_len_rev() {
        let height = 3;
        let array = [2, 7, 1, 2];
        assert_eq!(3, view_len_rev(height, &array));

        let array = [2, 7, 1, 3];
        assert_eq!(1, view_len_rev(height, &array));
    }

    #[test]
    fn test_get_scenic_score() {
        let row: (usize, &[u8]) = (3, &[3, 3, 5, 4, 9]);
        let column: (usize, &[u8]) = (2, &[3, 5, 3, 5, 3]);
        let height = 5 as u8;
        assert_eq!(8, get_scenic_score(height, row, column))
    }
}
