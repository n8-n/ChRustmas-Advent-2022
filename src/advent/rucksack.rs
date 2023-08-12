use super::io::*;

pub fn get_sum_of_priorities_for_common_items(filename: &str) -> usize {
    let lines = read_file_as_vector(filename);
    return lines.iter().fold(0, |acc, l: &String| -> usize { acc + Rucksack::from_string(l).common_item_value() });
}

mod priority {
    /// Priority of letter related to index of array.
    const PRIORITIES: [char; 53] = [
        '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn get_priority(c: char) -> usize {
        let priority = PRIORITIES.iter().position(|&v| v == c);

        match priority {
            Some(p) => p,
            None => panic!("Character is not in priority list."),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    compartments: (String, String),
    common_item: char,
}

impl Rucksack {
    fn from_string(s: &str) -> Rucksack {
        let (first, second) = s.split_at(s.len() / 2);

        Rucksack {
            compartments: (first.to_string(), second.to_string()),
            common_item: Self::get_common_char(first, second),
        }
    }

    fn get_common_char(s1: &str, s2: &str) -> char {
        let mut common = '_';

        s1.chars().for_each(|c| {
            if s2.contains(c) {
                common = c;
            }
        });

        common
    }

    fn common_item_value(&self) -> usize {
        priority::get_priority(self.common_item)
    }
}

//
//
#[cfg(test)]
mod tests {
    use super::priority::*;
    use super::*;

    #[test]
    fn test_sum_of_rucksack_commons_items_from_file() {
        let ans = get_sum_of_priorities_for_common_items("resources/test/03_rucksack.txt");
        assert_eq!(157, ans);
    }

    #[test]
    fn test_priorities_value() {
        let priority = get_priority('c');
        assert_eq!(3, priority);
    }

    #[test]
    fn test_rucksack_parse() {
        let s = "PmmdzqPrVvPwwTWBwg";
        let rs = Rucksack::from_string(s);

        assert_eq!(
            ("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
            rs.compartments
        );
        assert_eq!('P', rs.common_item);
        assert_eq!(42, rs.common_item_value());
    }
}
