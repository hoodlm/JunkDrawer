use std::env;
use std::fs;

// cargo run --bin prot -- /tmp/rna_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_rna = fs::read_to_string(&filename).unwrap();

    let protein = protein_from_rna(&input_rna);
    println!("{protein}");
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn protein_from_rna(input_rna: &str) -> String {
    let rna_length = input_rna.chars().count();
    let mut protein = String::with_capacity(rna_length / 3);

    let mut index = 0;
    while index < rna_length {
        let codon = &input_rna[index..index+3];
        index = index + 3;
        match codon {
            "UAA" => break,
            "UAG" => break,
            "UGA" => break,
            "UUU" => protein.push_str("F"),
            "UUC" => protein.push_str("F"),
            "UUA" => protein.push_str("L"),
            "UUG" => protein.push_str("L"),
            "UCU" => protein.push_str("S"),
            "UCC" => protein.push_str("S"),
            "UCA" => protein.push_str("S"),
            "UCG" => protein.push_str("S"),
            "UAU" => protein.push_str("Y"),
            "UAC" => protein.push_str("Y"),
            "UGU" => protein.push_str("C"),
            "UGC" => protein.push_str("C"),
            "UGG" => protein.push_str("W"),
            "CUU" => protein.push_str("L"),
            "CUC" => protein.push_str("L"),
            "CUA" => protein.push_str("L"),
            "CUG" => protein.push_str("L"),
            "CCU" => protein.push_str("P"),
            "CCC" => protein.push_str("P"),
            "CCA" => protein.push_str("P"),
            "CCG" => protein.push_str("P"),
            "CAU" => protein.push_str("H"),
            "CAC" => protein.push_str("H"),
            "CAA" => protein.push_str("Q"),
            "CAG" => protein.push_str("Q"),
            "CGU" => protein.push_str("R"),
            "CGC" => protein.push_str("R"),
            "CGA" => protein.push_str("R"),
            "CGG" => protein.push_str("R"),
            "AUU" => protein.push_str("I"),
            "AUC" => protein.push_str("I"),
            "AUA" => protein.push_str("I"),
            "AUG" => protein.push_str("M"),
            "ACU" => protein.push_str("T"),
            "ACC" => protein.push_str("T"),
            "ACA" => protein.push_str("T"),
            "ACG" => protein.push_str("T"),
            "AAU" => protein.push_str("N"),
            "AAC" => protein.push_str("N"),
            "AAA" => protein.push_str("K"),
            "AAG" => protein.push_str("K"),
            "AGU" => protein.push_str("S"),
            "AGC" => protein.push_str("S"),
            "AGA" => protein.push_str("R"),
            "AGG" => protein.push_str("R"),
            "GUU" => protein.push_str("V"),
            "GUC" => protein.push_str("V"),
            "GUA" => protein.push_str("V"),
            "GUG" => protein.push_str("V"),
            "GCU" => protein.push_str("A"),
            "GCC" => protein.push_str("A"),
            "GCA" => protein.push_str("A"),
            "GCG" => protein.push_str("A"),
            "GAU" => protein.push_str("D"),
            "GAC" => protein.push_str("D"),
            "GAA" => protein.push_str("E"),
            "GAG" => protein.push_str("E"),
            "GGU" => protein.push_str("G"),
            "GGC" => protein.push_str("G"),
            "GGA" => protein.push_str("G"),
            "GGG" => protein.push_str("G"),
            x => protein.push_str(&format!("<ERROR:{x}>")),
        };
    }
    protein
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_rna_protein() {
        let sample_rna = String::from(
            "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA"
        );
        println!("{}", sample_rna.chars().count());
        assert_eq!("MAMAPRTEINSTRING", protein_from_rna(&sample_rna));
    }

    #[test]
    fn test_all_codons() {
        // Tests all codons except the 'Stop' codons UAA UAG UGA
        let rna = String::from(
            "UUUCUUAUUGUUUUCCUCAUCGUCUUACUAAUAGUAUUGCUGAUGGUGUCUCCUACUGCUUCCCCCACCGCCUCACCAACAGCAUCGCCGACGGCGUAUCAUAAUGAUUACCACAACGACCAAAAAGAACAGAAGGAGUGUCGUAGUGGUUGCCGCAGCGGCCGAAGAGGAUGGCGGAGGGGG" 
        );

        let protein = protein_from_rna(&rna);
        assert_eq!("FLIVFLIVLLIVLLMVSPTASPTASPTASPTAYHNDYHNDQKEQKECRSGCRSGRRGWRRG", &protein);
        assert_eq!(64 - 3, protein.chars().count());
    }

    #[test]
    fn test_stop_codons() {
        // should stop here->...
        let rna_uaa = "UUUCCCUAACUU";
        assert_eq!("FP", protein_from_rna(&rna_uaa));

        // should stop here->...
        let rna_uag = "GAGCCCUAGCUU";
        assert_eq!("EP", protein_from_rna(&rna_uag));

        // should stop here->...
        let rna_uga = "GACCCCUGACUU";
        assert_eq!("DP", protein_from_rna(&rna_uga));
    }
}
