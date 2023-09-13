/*  coordinates struct
    keep track of coords visited
    every time head moves, check if tail needs to move
        can move up, down,left, right, diagonal (determine which way by comparing coords)
*/

use std::collections::HashSet;

use crate::common::io;

pub fn get_answer(filename: &str) -> () {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

}

struct Rope {
    head_xy: Position,
    tail_xy: Position,
    tail_visited: HashSet<Position>
}

impl Rope {
    fn new() -> Self {
        Rope {
            head_xy: Position::new(),
            tail_xy: Position::new(),
            tail_visited: HashSet::new()
        }
    }

    fn move_head(&mut self, direction: Direction, steps: u8) {
        for _ in 0..steps {
            // move head
            self.head_xy.move_position(&direction);

            // check if tail needs to move
        }
    }

    fn should_move_tail(&self) -> bool {
        // should move tail if head and tail are not touching
        self.head_xy.x - self.tail_xy.x == 0
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
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
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();
        
        // TODO: fix diagonal check
        (x + y == 1) || (x - y == 0)
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
        let mut pos = Position::new();
        pos.move_position(&Direction::Down);
        assert_eq!(Position{ x: 0, y: -1}, pos);

        pos.move_position(&Direction::Right);
        assert_eq!(Position{ x: 1, y: -1}, pos);
    }
}