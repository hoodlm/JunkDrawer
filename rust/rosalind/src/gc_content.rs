use std::env;
use std::fs;
use std::iter::Peekable;
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input = fs::read_to_string(&filename).unwrap();

    let fastas: Vec<Fasta> = FastaReader::new(&input.clone()).collect();
    let gcs: Vec<(&String, f64)> = fastas.iter()
        .map(|x| (&x.id, calculate_gc_percentage(x)))
        .collect();

    println!("{:?}", gcs);

    let max = gcs.iter()
        .reduce(|x, y| match x.1.partial_cmp(&y.1) { Some(Ordering::Greater) => x, _ => y })
        .unwrap();

    println!("{}", max.0);
    println!("{}", max.1);
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
    id: String,
    sequence: String
}

impl Fasta {
    fn new(id: String, sequence: String) -> Fasta {
        assert!(Self::valid_id_line(&id), "ID '{id}' is not a well-formed ID");
        let id_clean = &id[1..];
        Fasta { id: id_clean.to_string(), sequence: sequence }
    }

    fn valid_id_line(s: &str) -> bool {
        s.chars().next() == Some('>')
    }
}

struct FastaReader<'a> {
    file_iter: Peekable<std::str::Lines<'a>>
}

impl<'a> FastaReader<'a> {
    fn new(raw_text: &'a String) -> FastaReader<'a> {
        FastaReader { file_iter: raw_text.lines().peekable() }
    }
}

impl Iterator for FastaReader<'_> {
    type Item = Fasta;

    fn next(&mut self) -> Option<Self::Item> {
        let id: &str = match self.file_iter.next() {
            None => { return None },
            Some("") => { return None },
            Some(line) => line,
        };
        let mut sequence = String::from("");

        while self.file_iter.peek().is_some() && !Fasta::valid_id_line(self.file_iter.peek().unwrap()) {
            sequence.push_str(self.file_iter.next().unwrap());
        }

        let fasta = Fasta::new(id.to_string(), sequence.to_string());
        return Some(fasta);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gc_percentage() {
        let sample = Fasta { sequence: String::from("AGCTATAG"), id: String::from("doesnt matter") };
        assert_eq!(37.5, calculate_gc_percentage(&sample));
    }
    #[test]
    fn test_valid_id() {
        assert_eq!(true, Fasta::valid_id_line(&String::from(">Foo_bar")));
        assert_eq!(false, Fasta::valid_id_line(&String::from("Foo_bar")));
        assert_eq!(false, Fasta::valid_id_line(&String::from("")));
    }

    #[test]
    fn fasta_parser() {
        let sample_raw_fastas = String::from(
">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT");
        let reader = FastaReader::new(&sample_raw_fastas);
        let parsed: Vec<Fasta> = reader.collect();
        assert_eq!(3, parsed.len());
        assert_eq!("Rosalind_6404", parsed[0].id);
        assert_eq!("Rosalind_5959", parsed[1].id);
        assert_eq!("Rosalind_0808", parsed[2].id);
        assert_eq!("CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG", parsed[0].sequence);
        assert_eq!("CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC", parsed[1].sequence);
        assert_eq!("CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT", parsed[2].sequence);
    }
}
