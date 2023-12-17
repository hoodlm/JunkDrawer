use std::env;
use std::fs;

// cargo run --bin hamm -- /tmp/hamm.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_raw = fs::read_to_string(&filename).unwrap();

    let lines: Vec<&str> = input_raw.lines().collect();
    assert!(lines.len() >= 2, "Input file should be two lines to compare");

    let result = hamming_distance(lines[0], lines[1]).unwrap();
    println!("{result}");
}


fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn hamming_distance(str1: &str, str2: &str) -> Result<u32, String> {
    let len1 = str1.len();
    let len2 = str2.len();
    if len1 != len2 {
        return Err(
            format!("Cannot compare hamming distance of strings of different lengths:\n  {}\n  {}", &str1, &str2)
        );
    } else {
        let mut distance = 0;
        let mut l_chars = str1.chars();
        let mut r_chars = str2.chars();
        {
            let mut left = l_chars.next();
            let mut right = r_chars.next();
            while left != None && right != None {
                if left != right {
                    distance += 1;
                }
                left = l_chars.next();
                right = r_chars.next();
            }
        }
        return Ok(distance);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hamming_distance_same_length() {
        assert_eq!(2, hamming_distance("brain", "brown").unwrap());
    }

    #[test]
    fn hamming_distance_given_dna_case() {
        assert_eq!(7, hamming_distance("GAGCCTACTAACGGGAT", "CATCGTAATGACGGCCT").unwrap());
    }

    #[test]
    fn hamming_distance_same_string() {
        assert_eq!(0, hamming_distance("brain", "brain").unwrap());
    }

    #[test]
    fn hamming_distance_one_char_string() {
        assert_eq!(1, hamming_distance("a", "b").unwrap());
    }

    #[test]
    fn hamming_distance_same_char_string() {
        assert_eq!(0, hamming_distance("F", "F").unwrap());
    }

    #[test]
    fn hamming_distance_empty_string() {
        assert_eq!(0, hamming_distance("", "").unwrap());
    }

    #[test]
    fn hamming_distance_differnet_length_err() {
        assert!(hamming_distance("bbb", "bb").is_err());
    }
}
