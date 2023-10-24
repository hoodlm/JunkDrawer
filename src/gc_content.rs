use std::env;
use std::fs;

fn main() {
    println!("Setting this aside; don't know enough rust to write a coherent state machine parser yet");
    if false {
        __main()
    }
}

fn __main() {
    println!("Setting this aside; don't know enough rust to write a coherent state machine parser yet");
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input = fs::read_to_string(&filename).unwrap();

    let fastas: Vec<Fasta> = FastaReader::_new(&input.clone()).collect();
    let result: Vec<f64> = fastas.iter().map(|x| calculate_gc_percentage(x)).collect();
    println!("{:?}", result);
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn calculate_gc_percentage(fasta: &Fasta) -> f64 {
    let mut gc = 0;
    for nucleotide in fasta.sequence.chars() {
        match nucleotide {
            'G' | 'C' => gc += 1,
            _ => { /* NOOP */ }
        };
    }
    let ratio: f64 = (gc as f64) / (fasta.sequence.len() as f64);
    return 100f64 * (ratio as f64); 
}

struct Fasta {
    _id: String,
    sequence: String
}

impl Fasta {
    fn _new(id: String, sequence: String) -> Fasta {
        Fasta { _id: id, sequence: sequence }
    }
}

struct FastaReader<'a> {
    _internal_iter: std::str::Lines<'a>,
}

impl<'a> FastaReader<'a> {
    fn _new(raw_text: &'a String) -> FastaReader<'a> {
        let line_iterator = raw_text.lines();
        FastaReader { _internal_iter: line_iterator }
    }
}

impl Iterator for FastaReader<'_> {
    type Item = Fasta;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gc_percentage() {
        let sample = Fasta { sequence: String::from("AGCTATAG"), _id: String::from("doesnt matter") };
        assert_eq!(37.5, calculate_gc_percentage(&sample));
    }

    #[test]
    #[ignore] // TODO
    fn fasta_parser() {
        let sample_raw_fastas = String::from("
>Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT
        ");
        let reader = FastaReader::_new(&sample_raw_fastas);
        let parsed: Vec<Fasta> = reader.collect();
        assert_eq!(3, parsed.len());
    }
}
