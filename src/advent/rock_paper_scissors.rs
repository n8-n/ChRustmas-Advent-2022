use super::io::*;

pub fn calculate_score_for_file(filename: &str, mode: &ParseMode) -> i32 {
    let lines = read_file_as_vector(filename);

    let mut total_score = 0;

    for l in lines {
        total_score += Round::from_line(&l, &mode).score() as i32;
    }

    total_score
}

/// How we should parse/interpret the second column of the text file.
///     Choice: column is what you shape you should choose.
///     Result: column is the result you want to occur.
#[derive(Debug)]
pub enum ParseMode {
    Choice,
    Result
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
    fn value(&self) -> i8 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn is_win(&self, other: &Shape) -> Result {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => Result::Win,
            (Shape::Paper, Shape::Rock) => Result::Win,
            (Shape::Scissors, Shape::Paper) => Result::Win,
            (Shape::Rock, Shape::Paper) => Result::Loss,
            (Shape::Scissors, Shape::Rock) => Result::Loss,
            (Shape::Paper, Shape::Scissors) => Result::Loss,
            _ => Result::Draw
        }
    }

    fn from_char(c: char) -> Shape {
        match c {
            'X' | 'A' => Shape::Rock,
            'Y' | 'B' => Shape::Paper,
            'Z' | 'C' => Shape::Scissors,
            _ => panic!("Cannot parse Shape from character."),
        }
    }
}

impl Result {
    fn value(&self) -> i8 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loss => 0
        }
    }

    fn from_char(c: char) -> Result {
        match c {
            'X' => Result::Loss,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Cannot parse Result from character."),
        }
    }

    fn get_shape_to_match_result(&self, their_shape: &Shape) -> Shape {
        match (self, their_shape) {
            (Result::Win, Shape::Rock) => Shape::Paper,
            (Result::Win, Shape::Scissors) => Shape::Rock,
            (Result::Win, Shape::Paper) => Shape::Scissors,
            (Result::Loss, Shape::Rock) => Shape::Scissors,
            (Result::Loss, Shape::Scissors) => Shape::Paper,
            (Result::Loss, Shape::Paper) => Shape::Rock,
            _ => their_shape.clone()
        }
    }
}

impl Round {
    fn from_line(l: &String, mode: &ParseMode) -> Round {
        match mode {
            ParseMode::Choice => {
                Round {
                    theirs: Shape::from_char(l.chars().nth(0).unwrap()),
                    mine: Shape::from_char(l.chars().nth(2).unwrap()),
                }
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

    fn score(&self) -> i8 {
        let result = self.is_my_win();
        self.mine.value() + result.value()
    }
}


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
        let result = calculate_score_for_file("resources/test/rps.txt", &ParseMode::Choice);
        assert_eq!(15, result);

        let result = calculate_score_for_file("resources/test/rps.txt", &ParseMode::Result);
        assert_eq!(12, result);
    }
}
