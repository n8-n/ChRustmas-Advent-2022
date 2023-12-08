use std::fmt::Display;

use pathfinding::directed::bfs::bfs;

use crate::common::{
    grid::{Grid, Point},
    io,
};

pub fn get_fewest_steps_from_start(filename: &str) -> usize {
    let nodes = read_file_into_grid(filename);
    
    let start = find_start_node_position(&nodes).expect("Could not find start node");
    let result = bfs(&start, |n| node_successors(n, &nodes), |n| n.elevation == 'E').expect("Should have a path");
    
    result.len() - 1    // remove extra start or end node, idk
}

pub fn get_fewest_steps_from_low_elevation(filename: &str) -> usize {
    let nodes = read_file_into_grid(filename);

    find_a_nodes_at_edge(&nodes)
        .iter()
        .flat_map(|start| bfs(start, |n| node_successors(n, &nodes), |n| n.elevation == 'E'))
        .map(|path| path.len() - 1)
        .min().expect("Should have min")
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

// fn print_path(path: &Vec<&Node>) {
//     for (i, n) in path.iter().enumerate() {
//         println!("{}: {}", i, n.position);
//     }
// }

fn node_successors<'a>(current: &Node, nodes: &'a Grid<Node>) -> Vec<&'a Node> {
    nodes
        .get_adjacent_points(&current.position)
        .iter()
        .flat_map(|p| nodes.get_element(p))
        .filter(|n| current.distance_from(&n).is_some())
        .collect()
}

fn find_start_node_position(nodes: &Grid<Node>) -> Option<&Node> {
    for n in nodes.elements.iter() {
        if n.elevation == 'S' {
            return Some(n);
        }
    }
    None
}

fn find_a_nodes_at_edge(nodes: &Grid<Node>) -> Vec<&Node> {
    let mut a_nodes = Vec::new();

    for n in nodes.elements.iter() {
        if (n.elevation == 'a' || n.elevation == 'S') && nodes.is_edge_node(&n.position) {
            a_nodes.push(n);
        }
    }

    a_nodes
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    elevation: char,
    position: Point,
}

impl Node {
    fn new(elevation: char, position: Point) -> Self {
        Node {
            elevation,
            position,
        }
    }

    fn from_line(line: &str, y: usize) -> Vec<Self> {
        line.chars()
            .enumerate()
            .map(|(i, c)| Self::new(c, Point { x: i, y: y }))
            .collect()
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
        // Same elevation: 1 step
        Some(1)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elevation)
    }
}

fn char_to_number(c: char) -> usize {
    match c {
        'a' | 'S' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' | 'E' => 25,
        _ => 0,
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
        assert_eq!(
            find_start_node_position(&grid).unwrap().position,
            Point { x: 0, y: 0 }
        );
        let grid = read_file_into_grid("resources/test/12_hillwalking_modified_test.txt");
        let point = Point { x: 5, y: 3 };
        assert_eq!(find_start_node_position(&grid).unwrap().position, point);

        let node = grid.get_element(&point);
        assert_eq!(node.unwrap().elevation, 'S');
    }

    #[test]
    fn test_distance_from() {
        let point = Point { x: 0, y: 0 };

        let node1 = Node {
            elevation: 'a',
            position: point.clone(),
        };
        let node2 = Node {
            elevation: 'b',
            position: point.clone(),
        };
        assert_eq!(node1.distance_from(&node2), Some(1));
        assert_eq!(node2.distance_from(&node1), Some(1));
        assert_eq!(node1.distance_from(&node1), Some(1));

        let node1 = Node {
            elevation: 'c',
            position: point.clone(),
        };
        let node2 = Node {
            elevation: 'h',
            position: point.clone(),
        };
        assert_eq!(node1.distance_from(&node2), None);
        assert_eq!(node2.distance_from(&node1), Some(5));
    }

    #[test]
    fn test_calculate_distance_from_start_to_peak() {
        assert_eq!(31, get_fewest_steps_from_start("resources/test/12_hillwalking.txt"));
    }

    #[test]
    fn test_get_a_nodes() {
        let grid = read_file_into_grid("resources/test/12_hillwalking.txt");
        let result = find_a_nodes_at_edge(&grid);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_calculate_fewest_steps_to_low_elevation() {
        assert_eq!(29, get_fewest_steps_from_low_elevation("resources/test/12_hillwalking.txt"));
    }
}
