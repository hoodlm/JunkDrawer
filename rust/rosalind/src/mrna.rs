use std::env;
use std::fs;

// cargo run --bin prot -- /tmp/rna_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let protein = fs::read_to_string(&filename).unwrap();
    let result = compute_mrna_permutations_for_protein(&protein);

    println!("{}", result);
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn compute_mrna_permutations_for_protein(protein: &String) -> u64 {
    protein.chars()
        .map(|aa| aa_to_n_rna_strings(&aa))
        .reduce(|x, y| (x * y) % 1000000)
        .unwrap_or(1)
        * 3 // for 3 possible STOP codons
        % 1000000
}

fn aa_to_n_rna_strings(aa: &char) -> u64 {
    match aa {
        'F' => 2,
        'L' => 6,
        'S' => 6,
        'Y' => 2,
        'C' => 2,
        'W' => 1,
        'P' => 4,
        'H' => 2,
        'Q' => 2,
        'R' => 6,
        'I' => 3,
        'M' => 1,
        'T' => 4,
        'N' => 2,
        'K' => 2,
        'V' => 4,
        'A' => 4,
        'D' => 2,
        'E' => 2,
        'G' => 4,
        c => { eprintln!("WARN: Unexpected char '{}'", c); 1 },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_case() {
        let sample_protein = String::from("MA");
        let result = compute_mrna_permutations_for_protein(&sample_protein);
        assert_eq!(12, result);
    }

    #[test]
    fn empty_string() {
        let sample_protein = String::from("");
        let result = compute_mrna_permutations_for_protein(&sample_protein);
        assert_eq!(3, result);
    }

    #[test]
    fn exhaust_valid_amino_acids() {
        let sample_protein = String::from("FLSYCWPHQRIMTNKVADEG");
        let result = compute_mrna_permutations_for_protein(&sample_protein);
        assert_eq!(215872, result);
    }

    #[test]
    fn overflow_scale_test() {
        let sample_protein = String::from("SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS");
        let result = compute_mrna_permutations_for_protein(&sample_protein);
        assert_eq!(24448, result);
    }
}
