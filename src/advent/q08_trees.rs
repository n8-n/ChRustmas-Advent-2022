use crate::common::{grid::Grid, io};

pub fn get_number_of_visible_trees(filename: &str) -> usize {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let grid = create_trees_grid(lines);

    find_visible_trees(grid)
}

fn create_trees_grid(lines: Vec<String>) -> Grid<u8> {
    let mut grid = Grid::new();

    lines
        .iter()
        .map(string_to_numbers)
        .for_each(|nums| grid.add_row(nums));

    grid
}

fn string_to_numbers(line: &String) -> Vec<u8> {
    return line
        .chars()
        .map(|c: char| c.to_digit(10))
        .flatten()
        .map(|n| n as u8)
        .collect();
}

fn find_visible_trees(grid: Grid<u8>) -> usize {
    // trees on the edge are visible. minus 4 for double-counted corners.
    // we will need to ignore these when iterating through the grid.
    let mut visible = (grid.rows * 2) + (grid.columns * 2) - 4;

    let start = grid.columns;
    let end = grid.elements.len() - grid.columns;

    grid.elements[start..end]
        .chunks(grid.columns)
        .enumerate()
        .for_each(|(row_index, row)| {
            let end_row = row.len() - 1;

            row[1..end_row]
                .iter()
                .enumerate()
                .for_each(|(column_index, tree_height)| {
                    if is_tree_visible(*tree_height, (row_index + 1, row), column_index + 1, &grid) {
                        visible += 1;
                    }
                });
        });

    visible
}

fn is_tree_visible(height: u8, row: (usize, &[u8]), column_index: usize, grid: &Grid<u8>) -> bool {
    let row_index = row.0;
    let row_values = row.1;

    // row before
    let row_before = height_check(&row_values[..column_index], height);
    if row_before.is_none() {
        return true;
    }

    // row after
    let row_after = height_check(&row_values[(column_index + 1)..], height);
    if row_after.is_none() {
        return true;
    }

    let column = grid.get_column(column_index).expect("Should have column");

    // column before
    let column_after = height_check(&column[..row_index], height);
    if column_after.is_none() {
        return true;
    }

    // column after
    let column_after = height_check(&column[(row_index + 1)..], height);

    column_after.is_none()
}

fn height_check(segment: &[u8], height: u8) -> Option<&u8> {
    segment.iter().find(|tree: &&u8| height <= **tree)
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
        let trees = get_number_of_visible_trees("resources/test/08_trees.txt");
        assert_eq!(21, trees);
    }
}
