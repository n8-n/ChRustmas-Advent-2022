use std::fmt::Display;

use crate::common::{grid::{Grid, Point}, io};

pub fn get_smallest_distance(filename: &str) -> u32 {
    let _nodes = read_file_into_grid(filename);


    0
}

fn read_file_into_grid(filename: &str) -> Grid<Node> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let mut grid: Grid<Node> = Grid::new();

    lines
        .iter()
        .enumerate()
        .map(|(i, l)| Node::from_line(l, i))
        .for_each(|nodes| grid.add_row(nodes));

    grid
}

// Use Djikstra'a Algorithm
fn calculate_distance(_nodes: &mut Grid<Node>) -> u32 {
    // https://docs.rs/pathfinding/latest/pathfinding/directed/dijkstra/fn.dijkstra.html

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
    position: Point
}

impl Node {
    fn new(elevation: char, position: Point) -> Self {
        Node {
            elevation, position
        }
    }

    fn from_line(line: &str, y: usize) -> Vec<Self> {
        line.chars()
            .enumerate()
            .map(|(i, c)| Self::new(c, Point { x: i, y: y })).collect()
    }

    fn distance_from(&self, other: &Node) -> Option<usize> {
        let here = char_to_number(self.elevation);
        let there = char_to_number(other.elevation);

        if here > there {
            return Some(here - there);
        } else if here < there {
            if there - here == 1 {
                return Some(1);
            } else {
                return None;
            }
        }

        Some(0)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elevation)
    }
}

fn char_to_number(c: char) -> usize {
    match c {
        'a' | 'S' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' | 'E' => 26,
        _ => 0
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
        let point = Point { x: 5, y: 3 };
        assert_eq!(find_start_node_position(&grid).unwrap(), point);

        let node = grid.get_element(point);
        assert_eq!(node.unwrap().elevation, 'S');
    }

    #[test]
    fn test_distance_from() {
        let point = Point { x: 0, y: 0 };
        
        let node1 = Node { elevation: 'a', position: point.clone() };
        let node2 = Node { elevation: 'b', position: point.clone() };
        assert_eq!(node1.distance_from(&node2), Some(1));
        assert_eq!(node2.distance_from(&node1), Some(1));
        assert_eq!(node1.distance_from(&node1), Some(0));

        let node1 = Node { elevation: 'c', position: point.clone() };
        let node2 = Node { elevation: 'h', position: point.clone() };
        assert_eq!(node1.distance_from(&node2), None);
        assert_eq!(node2.distance_from(&node1), Some(5));
    }
}
