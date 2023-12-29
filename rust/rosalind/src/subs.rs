use std::env;
use std::fs;

// cargo run --bin subs -- /tmp/subs.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_raw = fs::read_to_string(&filename).unwrap();

    let fields: Vec<&str> = input_raw.split_ascii_whitespace().collect();
    assert!(fields.len() == 2, "Expected exactly two newline or whitespace delimited sequences");

    let sequence = fields[0];
    let substr   = fields[1];

    let result = find_substring_indices(&sequence, &substr);
    let mut outstr = String::new();
    for index in &result {
        outstr.push_str(&index.to_string());
        outstr.push(' ');
    }
    println!("{}", outstr);
}


fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn find_substring_indices(sequence: &str, substr: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    if sequence.is_empty() || substr.is_empty() {
        return indices;
    }
 
    let substr_chars: Vec<char> = substr.chars().collect();

    for (index, c) in sequence.char_indices() {
        if c == substr_chars[0] {
            let (_, s) = sequence.split_at(index);
            if s.starts_with(substr) {
                indices.push(index + 1); // +1 as answer specifies 1-indexed, not 0-indexed
            }
        }
    }

    return indices;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_example() {
        let expected = vec![2, 4, 10];
        assert_eq!(expected, find_substring_indices("GATATATGCATATACTT", "ATAT"));
    }

    #[test]
    fn no_match() {
        assert!(find_substring_indices("GATATATGCATATACTT", "ATATATATAT").is_empty());
    }

    #[test]
    fn subs_longer_than_sequence() {
        assert!(find_substring_indices("GAT", "GATACA").is_empty());
    }

    #[test]
    fn empty_sequence() {
        assert!(find_substring_indices("", "GATACA").is_empty());
    }

    #[test]
    fn empty_substring() {
        assert!(find_substring_indices("GATACA", "").is_empty());
    }

    #[test]
    fn exact_match() {
        let expected = vec![1];
        assert_eq!(expected, find_substring_indices("GATACA", "GATACA"));
    }

    #[test]
    fn end_of_str() {
        let expected = vec![1, 2];
        assert_eq!(expected, find_substring_indices("GGGG", "GGG"));
    }
    
    #[test]
    fn very_large_string() {
        let mut seq = String::new();
        for i in 0..100000 {
            seq.push_str("AAATTTCCCGGG")
        }
        assert!(find_substring_indices(&seq, "GGGCCCTTTAAA").is_empty());
    }
}
