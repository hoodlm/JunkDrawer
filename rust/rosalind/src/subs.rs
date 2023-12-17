use std::env;
use std::fs;

// cargo run --bin subs -- /tmp/subs.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_raw = fs::read_to_string(&filename).unwrap();

    let fields: Vec<&str> = input_raw.split_ascii_whitespace().collect();
    assert!(fields.len() != 2, "Expected exactly two newline or whitespace delimited sequences");

    let sequence = fields[0];
    let substr   = fields[1];

    let result = find_substring_indices(&sequence, &substr);
    println!("{:?}", result);
}


fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn find_substring_indices(sequence: &str, substr: &str) -> Vec<usize> {
    let substr_chars: Vec<char> = substr.chars().collect();
    let mut indices = Vec::new();

    for (index, c) in sequence.char_indices() {
        if c == substr_chars[0] {
            // Compare the rest of the substring

            indices.push(index);
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
}
