/*  coordinates struct
    keep track of coords visited
    every time head moves, check if tail needs to move
        can move up, down,left, right, diagonal (determine which way by comparing coords)
*/

use std::collections::HashSet;

use crate::common::io;

pub fn get_answer(filename: &str) -> usize {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");
    let mut rope = Rope::new();
    
    for line in lines {
        let mut chars = line.chars();
        let direction = chars.next().expect("Should be char");
        let steps = chars.skip(1).next().expect("Should be char").to_digit(10).unwrap() as u8;

        rope.move_head(Direction::from(direction).unwrap(), steps);
    }
    println!("{:?}", rope.tail_visited);
    rope.tail_visited.len()
}

struct Rope {
    head_xy: Point,
    tail_xy: Point,
    tail_visited: HashSet<Point>
}

impl Rope {
    fn new() -> Self {
        let mut visited = HashSet::new();
        let tail = Point::new();
        visited.insert(tail.clone());
        Rope {
            head_xy: Point::new(),
            tail_xy: tail,
            tail_visited: visited
        }
    }

    fn move_head(&mut self, direction: Direction, steps: u8) {
        for _ in 0..steps {
            self.head_xy.move_position(&direction);

            if let Some(direction) = self.should_move_tail() {
                let diff_x = self.tail_xy.x - direction.x;
                let diff_y = self.tail_xy.y - direction.y;

                // if diff_x > 0 {
                //     self.tail_xy.x += 1;
                // } else if diff_x < 0 {
                //     self.tail_xy.x -= 1;
                // }

                // if diff_y > 0 {
                //     self.tail_xy.y += 1;
                // } else if diff_y < 0 {
                //     self.tail_xy.y -= 1;
                // }


                self.tail_visited.insert(self.tail_xy.clone());
            }
        }
    }

    fn should_move_tail(&self) -> Option<Point> {
        let head = &self.head_xy;
        let tail = &self.tail_xy;
        if !self.head_xy.is_point_adjacent(&self.tail_xy) {
            return Some(Point { x: head.x - tail.x, y: head.y - tail.y });
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn move_position(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        };
    }

    fn is_point_adjacent(&self, other: &Self) -> bool {
        let a = (self.x - other.x).abs();
        let b = (self.y - other.y).abs();
        
        (a <= 1) && (b <= 1)
    }

    // 0, 0
    // 1, 1
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    fn from(c: char) -> Option<Self> {
        let direction = match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => return None,
        };

        Some(direction)
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
    fn test_direction() {
        assert_eq!(Direction::Left, Direction::from('L').unwrap());
        assert_eq!(None, Direction::from('M'));
    }

    #[test]
    fn test_move_position() {
        let mut pos = Point::new();
        pos.move_position(&Direction::Down);
        assert_eq!(Point{ x: 0, y: -1}, pos);

        pos.move_position(&Direction::Right);
        assert_eq!(Point{ x: 1, y: -1}, pos);
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