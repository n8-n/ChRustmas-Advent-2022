pub fn chunk_string(string: &str, chunk_size: usize) -> Vec<String> {
    let chars: Vec<char> = string.chars().collect();

    return chars.chunks(chunk_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_str() {
        let s = "abcdefghi";

        let res = chunk_string(s, 3);
        let expected_values: Vec<String> = vec!["abc".to_string(), "def".into(), "ghi".into()];
        assert_eq!(expected_values, res);

        let res = chunk_string(s, 4);
        let expected_values: Vec<String> = vec!["abcd".to_string(), "efgh".into(), "i".into()];
        assert_eq!(expected_values, res);
    }

}