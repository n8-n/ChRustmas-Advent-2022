use crate::common::io;

pub fn get_sum_of_priorities_for_common_items(rucksacks: &Vec<Rucksack>) -> usize {
    rucksacks.iter().fold(0, |acc, rs: &Rucksack| -> usize {
        acc + rs.common_item_value()
    })
}

pub fn get_sum_of_priorities_for_group(rucksacks: &Vec<Rucksack>) -> usize {
    rucksacks
        .chunks(3)
        .map(|rs: &[Rucksack]| rs[0].group_item_value())
        .fold(0, |acc, g: usize| -> usize { acc + g })
}

pub fn get_rucksacks_from_file(filename: &str) -> Vec<Rucksack> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    lines
        .chunks(3)
        .map(|ch: &[String]| -> Vec<Rucksack> {
            let common = get_common_char(vec![&ch[0], &ch[1], &ch[2]]);

            ch.iter()
                .map(|l: &String| -> Rucksack {
                    let mut rs = Rucksack::from_string(l);
                    rs.add_group_value(common.unwrap());

                    rs
                })
                .collect::<Vec<Rucksack>>()
        })
        .flatten()
        .collect::<Vec<Rucksack>>()
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

        priority.expect("Character is not in priority list.")
    }
}

#[derive(Debug)]
pub struct Rucksack {
    common_item: char,
    group_common: Option<char>,
}

impl Rucksack {
    fn from_string(line: &str) -> Rucksack {
        let (first, second) = line.split_at(line.len() / 2);

        Rucksack {
            common_item: get_common_char(vec![&first, &second]).unwrap(),
            group_common: None,
        }
    }

    fn common_item_value(&self) -> usize {
        priority::get_priority(self.common_item)
    }

    fn group_item_value(&self) -> usize {
        priority::get_priority(self.group_common.expect("No value for common group value"))
    }

    fn add_group_value(&mut self, c: char) {
        self.group_common = Some(c);
    }
}

fn get_common_char(str_vec: Vec<&str>) -> Option<char> {
    'char_loop: for c in str_vec[0].chars() {
        for (index, s) in str_vec[1..].iter().enumerate() {
            if !s.contains(c) {
                continue 'char_loop;
            } else {
                if index == str_vec.len() - 2 {
                    return Some(c);
                }
            }
        }
    }

    return None;
}

//
//
//
#[cfg(test)]
mod tests {
    use super::priority::*;
    use super::*;

    #[test]
    fn test_sum_of_rucksack_commons_items_from_file() {
        let rs = get_rucksacks_from_file("resources/test/03_rucksack.txt");

        let ans = get_sum_of_priorities_for_common_items(&rs);
        assert_eq!(157, ans);

        let ans = get_sum_of_priorities_for_group(&rs);
        assert_eq!(70, ans);
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

        assert_eq!('P', rs.common_item);
        assert_eq!(42, rs.common_item_value());
    }

    #[test]
    fn test_common_char() {
        let v = vec!["WgXc", "YYcs", "WcY", "hHcH"];
        assert_eq!('c', get_common_char(v).unwrap());

        let v = vec!["WgXc"];
        assert_eq!(None, get_common_char(v));

        let v = vec!["WgXc", "YYcs"];
        assert_eq!('c', get_common_char(v).unwrap());
    }
}
