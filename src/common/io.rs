use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_as_vector(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines = read_file_as_vector("resources/test/02_rps.txt").unwrap();

        assert_eq!(3, lines.len());
        let expected_values: Vec<String> = vec!["A Y".to_string(), "B X".into(), "C Z".into()];
        assert_eq!(expected_values, lines);
    }
}
