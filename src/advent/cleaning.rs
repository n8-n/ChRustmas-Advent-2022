use super::io;

pub fn get_count_of_overlapping_pairs(filename: &str) -> u16 {
    let lines = io::read_file_as_vector(filename);

    let mut count: u16 = 0;

    for l in lines {
        let pair = parse_line_into_pair(&l);
        if pair.do_pairs_overlap() { count += 1; }
    }
    
    count
}

struct CleaningPair {
    elf1: (u8, u8),
    elf2: (u8, u8),
}

impl CleaningPair {
    fn do_pairs_overlap(&self) -> bool {
        let p1_in_p2 = |p1: (u8, u8), p2: (u8, u8)| -> bool { (p1.0 >= p2.0) && (p1.1 <= p2.1) };

        p1_in_p2(self.elf1, self.elf2) || p1_in_p2(self.elf2, self.elf1)
    }
}

fn parse_line_into_pair(line: &str) -> CleaningPair {
    let (first, second) = line.split_once(',').expect("Cannot split string on ','");

    CleaningPair {
        elf1: string_to_range_ints(first),
        elf2: string_to_range_ints(second),
    }
}

fn string_to_range_ints(s: &str) -> (u8, u8) {
    let (start, end) = s.split_once('-').expect("Cannot split string on '-'");
    let parse = |n: &str| -> u8 { n.trim().parse().expect("Cannot parse as int") };

    (parse(start), parse(end))
}

//
//
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_into_pair() {
        let line = "2-4,6-8";
        let pair = parse_line_into_pair(line);

        assert_eq!((2, 4), pair.elf1);
        assert_eq!((6, 8), pair.elf2);
    }

    #[test]
    fn test_do_pairs_overlap() {
        let pair = CleaningPair { elf1: (1, 4), elf2: (2, 3) };
        assert!(pair.do_pairs_overlap());

        let pair = CleaningPair { elf1: (2, 4), elf2: (2, 4) };
        assert!(pair.do_pairs_overlap());

        let pair = CleaningPair { elf1: (3, 3), elf2: (2, 3) };
        assert!(pair.do_pairs_overlap());

        let pair = CleaningPair { elf1: (2, 3), elf2: (1, 4) };
        assert!(pair.do_pairs_overlap());
    }

    #[test]
    fn test_get_count_overlapping_pairs() {
        let count = get_count_of_overlapping_pairs("resources/test/04_cleaning_pairs.txt");
        assert_eq!(2, count);
    }
}
