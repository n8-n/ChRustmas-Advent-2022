use std::collections::HashSet;
use crate::common::io;

pub fn get_answer(filename: &str) -> usize {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let mut rope = Rope::new();

    for line in lines {
        let (direction, steps) = line.split_once(' ').expect("Should be able to split");
        let direction = direction.chars().next().expect("Should have char");
        let steps: u8 = steps.trim().parse().expect("Should parse");

        rope.move_head(
            Point::from_char(direction).expect("Should be valid char"),
            steps,
        );
    }
    rope.tail_visited.len()
}

struct Rope {
    head_xy: Point,
    tail_xy: Point,
    tail_visited: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        let mut visited = HashSet::new();
        let tail = Point::new();
        visited.insert(tail.clone());
        Rope {
            head_xy: Point::new(),
            tail_xy: tail,
            tail_visited: visited,
        }
    }

    fn move_head(&mut self, direction: Point, steps: u8) {
        for _ in 0..steps {
            let head = &mut self.head_xy;
            let tail = &mut self.tail_xy;
            head.move_position(&direction);

            if !head.is_point_adjacent(tail) {
                let diff_move = get_direction_from_diff(head, tail);
                tail.move_position(&diff_move);
                self.tail_visited.insert(tail.clone());
            }
        }
    }
}

fn get_direction_from_diff(p1: &Point, p2: &Point) -> Point {
    let diff_x = p1.x - p2.x;
    let diff_y = p1.y - p2.y;
    let space_move = |p: i32| -> i32 {              
        if p > 0 {
            1
        } else if p < 0 {
            -1
        } else {
            0
        }
    };

    // want to move only one space
    Point {
        x: space_move(diff_x),
        y: space_move(diff_y),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn from_char(c: char) -> Option<Self> {
        let mut diff_point = Self::new();

        match c {
            'R' => diff_point.x += 1,
            'L' => diff_point.x += -1,
            'U' => diff_point.y += 1,
            'D' => diff_point.y += -1,
            _ => return None,
        };

        Some(diff_point)
    }

    fn move_position(&mut self, move_diff: &Point) {
        self.x += move_diff.x;
        self.y += move_diff.y;
    }

    fn is_point_adjacent(&self, other: &Self) -> bool {
        let a = (self.x - other.x).abs();
        let b = (self.y - other.y).abs();

        (a <= 1) && (b <= 1)
    }
}

//
//
//
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_move_position() {
        let mut pos = Point::new();
        let move_point = Point{ x: 0, y: -1};
        pos.move_position(&move_point);
        pos.move_position(&move_point);
        assert_eq!(Point{ x: 0, y: -2}, pos);

        pos.move_position(&Point{ x: 1, y: 0});
        assert_eq!(Point{ x: 1, y: -2}, pos);
    }

    #[test]
    fn test_is_adjacent() {
        let pos = Point { x: 0, y: 0};

        for i in -1..=1 {
            for j in -1..=1 {
                assert!(pos.is_point_adjacent(&Point { x: i, y: j}));
            }
        }

        assert!(!pos.is_point_adjacent(&Point { x: 1, y: 3}));
        assert!(!pos.is_point_adjacent(&Point { x: 3, y: 3}));
    }

    #[test]
    fn test_move_rope_knots() {
        let result = get_answer("resources/test/09_rope.txt");
        assert_eq!(13, result);
    }
}
