/*  coordinates struct
    keep track of coords visited
    every time head moves, check if tail needs to move
        can move up, down,left, right, diagonal (determine which way by comparing coords)
*/

use crate::common::io;

pub fn get_answer(filename: &str) -> () {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32
}