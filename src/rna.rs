use std::env;
use std::fs;

// cargo run --bin rna -- /tmp/rna_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_dna = fs::read_to_string(&filename).unwrap();

    let rna = transcribe_to_rna(&input_dna);
    println!("{rna}");
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn transcribe_to_rna(input_dna: &str) -> String {
    let mut rna = String::with_capacity(input_dna.len());
    for nucleotide in input_dna.chars() {
        let next_char = match nucleotide {
            'T' => 'U',
            other => other
        };
        rna.push(next_char)
    }
    rna
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_case() {
        let sample_dna = String::from(
            "GATGGAACTTGACTACGTAAATT"
        );
        assert_eq!("GAUGGAACUUGACUACGUAAAUU", transcribe_to_rna(&sample_dna));
    }
}
