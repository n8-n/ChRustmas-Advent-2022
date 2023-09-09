use crate::common::io;

pub fn calculate_score_for_file(filename: &str, mode: ParseMode) -> u32 {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    let mut total_score: u32 = 0;

    for l in lines {
        total_score += Round::from_line(&l, &mode).score() as u32;
    }

    total_score
}

/// How we should parse/interpret the second column of the text file.
///     Choice: column is what you shape you should choose.
///     Result: column is the result you want to occur.
#[derive(Debug)]
pub enum ParseMode {
    Choice,
    Result,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Result {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct Round {
    theirs: Shape,
    mine: Shape,
}

impl Shape {
    fn value(&self) -> u8 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn is_win(&self, other: &Shape) -> Result {
        use self::Result::*;
        use Shape as S;

        match (*self, other) {
            (S::Rock, S::Scissors) => Result::Win,
            (S::Paper, S::Rock) => Result::Win,
            (S::Scissors, S::Paper) => Result::Win,
            (S::Rock, S::Paper) => Result::Loss,
            (S::Scissors, S::Rock) => Result::Loss,
            (S::Paper, S::Scissors) => Result::Loss,
            _ => Draw,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' | 'A' => Shape::Rock,
            'Y' | 'B' => Shape::Paper,
            'Z' | 'C' => Shape::Scissors,
            _ => panic!("Cannot parse Shape from character."),
        }
    }
}

impl Result {
    fn value(&self) -> u8 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loss => 0,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' => Result::Loss,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Cannot parse Result from character."),
        }
    }

    fn get_shape_to_match_result(&self, their_shape: &Shape) -> Shape {
        use self::Result as R;
        use Shape as S;

        match (self, their_shape) {
            (R::Win, S::Rock) => S::Paper,
            (R::Win, S::Scissors) => S::Rock,
            (R::Win, S::Paper) => S::Scissors,
            (R::Loss, S::Rock) => S::Scissors,
            (R::Loss, S::Scissors) => S::Paper,
            (R::Loss, S::Paper) => S::Rock,
            _ => their_shape.clone(),
        }
    }
}

impl Round {
    fn from_line(l: &str, mode: &ParseMode) -> Self {
        match mode {
            ParseMode::Choice => Round {
                theirs: Shape::from_char(l.chars().nth(0).unwrap()),
                mine: Shape::from_char(l.chars().nth(2).unwrap()),
            },
            ParseMode::Result => {
                let temp_shape = Shape::from_char(l.chars().nth(0).unwrap());
                let result = Result::from_char(l.chars().nth(2).unwrap());
                Round {
                    mine: result.get_shape_to_match_result(&temp_shape),
                    theirs: temp_shape,
                }
            }
        }
    }

    fn is_my_win(&self) -> Result {
        self.mine.is_win(&self.theirs)
    }

    fn score(&self) -> u8 {
        let result = self.is_my_win();
        self.mine.value() + result.value()
    }
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_from_line_parse() {
        let line = String::from("A Z");

        let r = Round::from_line(&line, &ParseMode::Choice);

        assert_eq!(Shape::Rock, r.theirs);
        assert_eq!(Shape::Scissors, r.mine);

        let r = Round::from_line(&line, &ParseMode::Result);

        assert_eq!(Shape::Rock, r.theirs);
        assert_eq!(Shape::Paper, r.mine);
    }

    #[test]
    fn test_is_my_win() {
        let round1 = Round {
            theirs: Shape::Rock,
            mine: Shape::Paper,
        };

        assert_eq!(Result::Win, round1.is_my_win());

        let round2 = Round {
            theirs: Shape::Rock,
            mine: Shape::Scissors,
        };

        assert_eq!(Result::Loss, round2.is_my_win());

        let round3 = Round {
            theirs: Shape::Paper,
            mine: Shape::Paper,
        };

        assert_eq!(Result::Draw, round3.is_my_win());
    }

    #[test]
    fn test_round_score() {
        let round1 = Round {
            theirs: Shape::Rock,
            mine: Shape::Paper,
        };

        assert_eq!(8, round1.score());
    }

    #[test]
    fn test_sum_scores() {
        let filename = "resources/test/02_rps.txt";

        let result = calculate_score_for_file(filename, ParseMode::Choice);
        assert_eq!(15, result);

        let result = calculate_score_for_file(filename, ParseMode::Result);
        assert_eq!(12, result);
    }
}
