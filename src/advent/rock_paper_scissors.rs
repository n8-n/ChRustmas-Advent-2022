use super::io::*;

pub fn calculate_score_for_file(filename: &str) -> u32 {
    let lines = read_file_as_vector(filename);

    let mut total_score: u32 = 0;

    for l in lines {
        total_score += Round::from_line(&l).score() as u32;
    }

    total_score
}

#[derive(Debug, PartialEq, Eq)]
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
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn is_win(&self, other: &Shape) -> Result {
        match (self, other) {
            (Shape::Rock, Shape::Paper) => Result::Loss,
            (Shape::Rock, Shape::Scissors) => Result::Win,
            (Shape::Scissors, Shape::Paper) => Result::Win,
            (Shape::Scissors, Shape::Rock) => Result::Loss,
            (Shape::Paper, Shape::Rock) => Result::Win,
            (Shape::Paper, Shape::Scissors) => Result::Loss,
            _ => Result::Draw,
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
    fn value(&self) -> u8 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loss => 0
        }
    }
}

impl Round {
    fn from_line(l: &String) -> Round {
        Round {
            theirs: Shape::from_char(l.chars().nth(0).unwrap()),
            mine: Shape::from_char(l.chars().nth(2).unwrap()),
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_from_line_parse() {
        let line = String::from("X B");

        let r = Round::from_line(&line);

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
        let result = calculate_score_for_file("resources/test/rps.txt");
        assert_eq!(15, result);
    }
}
