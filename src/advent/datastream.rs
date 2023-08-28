use crate::common::io;
use std::collections::{VecDeque, HashSet};


pub fn get_marker_end_index_from_file(filename: &str, marker_size: usize) -> u16 {
    let mut lines = io::read_file_as_vector(filename).expect("Could not read file");

    find_end_index_of_packet_start_marker(lines.get_mut(0).unwrap(), marker_size)
        .expect("Could not find index")
}

fn find_end_index_of_packet_start_marker(datastream: &mut String, marker_size: usize) -> Option<u16> {
    if datastream.len() < marker_size {
        return None;
    }

    let (initial, rest) = datastream.split_at_mut(marker_size);
    let mut previous_chars: VecDeque<char> = VecDeque::from(initial.chars().collect::<Vec<char>>());

    for (i, c) in rest.chars().into_iter().enumerate() {
        if are_chars_unique(&previous_chars) {
            let index = (i + marker_size) as u16;
            return Some(index);
        } else {
            let _ = previous_chars.pop_front();
            previous_chars.push_back(c);
        }
    }
    
    None
}

fn are_chars_unique(previous_chars: &VecDeque<char>) -> bool {
    let hash_of_chars: HashSet<char> = HashSet::from_iter(previous_chars.iter().cloned());

    hash_of_chars.len() == previous_chars.len()
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_end_index() {
        let marker_size = 4;
        let result = find_end_index_of_packet_start_marker(&mut "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), marker_size);
        assert_eq!(5, result.unwrap());

        let result = find_end_index_of_packet_start_marker(&mut "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), marker_size);
        assert_eq!(10, result.unwrap());

        let result = find_end_index_of_packet_start_marker(&mut "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), marker_size);
        assert_eq!(11, result.unwrap());
    }

    #[test]
    fn test_are_chars_unique() {
        let queue: VecDeque<char> = VecDeque::from(['a', 'b', 'a', 'c']);
        assert!(!are_chars_unique(&queue));

        let queue: VecDeque<char> = VecDeque::from(['a', 'b', 'x', 'c']);
        assert!(are_chars_unique(&queue));
    }

}
