use crate::common::io;

const INITIAL_WINDOW_LEN: usize = 3;

pub fn get_marker_end_index_from_file(filename: &str) -> u16 {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    find_end_index_of_packet_start_marker(&lines[0]).expect("Could not find index")
}

fn find_end_index_of_packet_start_marker(datastream: &str) -> Option<u16> {
    if datastream.len() <= INITIAL_WINDOW_LEN {
        return None;
    }

    let mut chars = datastream.chars();
    let mut window = Window {
        previous_chars: (chars.next()?, chars.next()?, chars.next()?),
    };

    // println!("{:?}", window);

    for (i, c) in chars.into_iter().enumerate() {
        // println!("{i}, {c}");

        if window.are_previous_chars_unique() && window.is_end_of_marker(c) {
            let index = (i + 1 + INITIAL_WINDOW_LEN) as u16;
            return Some(index);
        } else {
            window.update(c);
        }
    }

    None
}

#[derive(Debug)]
struct Window {
    previous_chars: (char, char, char),
}

impl Window {
    fn is_end_of_marker(&self, new_c: char) -> bool {
        !(new_c == self.previous_chars.0)
            && !(new_c == self.previous_chars.1)
            && !(new_c == self.previous_chars.2)
    }

    fn are_previous_chars_unique(&self) -> bool {
        !(self.previous_chars.0 == self.previous_chars.1)
            && !(self.previous_chars.0 == self.previous_chars.2)
            && !(self.previous_chars.1 == self.previous_chars.2)
    }

    fn update(&mut self, new_c: char) {
        self.previous_chars.0 = self.previous_chars.1;
        self.previous_chars.1 = self.previous_chars.2;
        self.previous_chars.2 = new_c;
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_end_index() {
        let result = find_end_index_of_packet_start_marker("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(5, result.unwrap());

        let result = find_end_index_of_packet_start_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(10, result.unwrap());

        let result = find_end_index_of_packet_start_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(11, result.unwrap());
    }
}
