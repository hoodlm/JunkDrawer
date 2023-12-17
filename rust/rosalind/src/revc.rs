use std::env;
use std::fs;

// cargo run --bin revc -- /tmp/revc_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_dna = fs::read_to_string(&filename).unwrap();

    let result = reverse_complement(&input_dna);
    println!("{result}");
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn reverse_complement(input_dna: &str) -> String {
    let mut output = String::with_capacity(input_dna.len());
    for nucleotide in input_dna.chars().rev() {
        let next_char = match nucleotide {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            other => {
                eprintln!("WARN: Unexpected char '{other}'");
                other
            }
        };
        output.push(next_char)
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_case_a() {
        let sample_dna = String::from(
            "GTCA"
        );
        assert_eq!("TGAC", reverse_complement(&sample_dna));
    }

    #[test]
    fn given_case_b() {
        let sample_dna = String::from(
            "AAAACCCGGT"
        );
        assert_eq!("ACCGGGTTTT", reverse_complement(&sample_dna));
    }
}
