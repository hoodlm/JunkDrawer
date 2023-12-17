use std::fmt;
use std::env;
use std::fs;

// cargo run --bin dna -- /tmp/dna.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_dna = fs::read_to_string(&filename).unwrap();

    let result = count(&input_dna);
    println!("{result}");
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

struct NucleotideCount {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
}

impl fmt::Display for NucleotideCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.c, self.g, self.t)
    }
}

fn count(input_dna: &str) -> NucleotideCount {
    let (mut a, mut c, mut g, mut t) = (0, 0, 0, 0);
    for nucleotide in input_dna.chars() {
        match nucleotide {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            other => {
                // ignore whitespace, log warning otherwise
                if !other.is_whitespace() {
                    eprintln!("WARN: unexpected character '{other}'");
                }
            }
        };
    }
    NucleotideCount { a: a, c: c, g: g, t: t }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_case() {
        let sample_dataset = String::from(
            "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC"
        );
        assert_eq!("20 12 17 21", count(&sample_dataset).to_string());
    }
}
