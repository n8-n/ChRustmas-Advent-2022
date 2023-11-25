use std::fmt::Display;

use crate::common::{grid::{Grid, Point, self}, io};

pub fn get_smallest_distance(filename: &str) -> u32 {
    let _nodes = read_file_into_grid(filename);


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

// Use Djikstra'a Algorithm
fn calculate_distance(nodes: &mut Grid<Node>) -> u32 {
    let start = find_start_node_position(nodes);
    let start_node = nodes.get_element_mut(start.unwrap());
    start_node.unwrap().update_distance(0);

    0
}

fn find_start_node_position(nodes: &Grid<Node>) -> Option<Point> {
    for (i, n) in nodes.elements.iter().enumerate() {
        if n.elevation == 'S' {
            return nodes.index_to_point(i);
        }
    }
    None
}

#[derive(Clone, Debug)]
struct Node {
    elevation: char,
    visited: bool,
    distance: Option<u32>
}

impl Node {
    fn from_char(c: char) -> Self {
        Node {
            elevation: c,
            visited: false,
            distance: None
        }
    }

    fn from_line(line: &str) -> Vec<Self> {
        line.chars().map(|c| Self::from_char(c)).collect()
    }

    fn update_distance(&mut self, d: u32) {
        self.distance = Some(d);
    }

    fn visit(&mut self) {
        self.visited = true;
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
    fn test_find_start_node() {
        let grid = read_file_into_grid("resources/test/12_hillwalking.txt");
        assert_eq!(find_start_node_position(&grid).unwrap(), Point { x: 0, y: 0 });
        let grid = read_file_into_grid("resources/test/12_hillwalking_modified_test.txt");
        assert_eq!(find_start_node_position(&grid).unwrap(), Point { x: 5, y: 3 });
    }
}
