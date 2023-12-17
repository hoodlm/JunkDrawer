use std::env;
use std::fs;

// cargo run --bin iprb -- /tmp/iprb.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let input_raw = fs::read_to_string(&filename).unwrap();

    let fields: Vec<&str> = input_raw.split_ascii_whitespace().collect();
    assert!(fields.len() >= 3, "Expected exactly three space-delimited numbers; e.g. '1 2 3'");

    let homozyg_dom = fields[0].parse().unwrap();
    let heterozyg = fields[1].parse().unwrap();
    let homozyg_rec = fields[2].parse().unwrap();

    let result = p_dominant_phenotype(homozyg_dom, heterozyg, homozyg_rec);
    println!("{result}");
}


fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn p_dominant_phenotype(homozyg_dom: f64, heterozyg: f64, homozyg_rec: f64 ) -> f64 {
    let total_pop = homozyg_dom + heterozyg + homozyg_rec;
    if total_pop < 2.0 {
        panic!("Can't simulate a population unless it has at least 2 individuals");
    }

    let answer =
        (homozyg_dom / total_pop)
      + (heterozyg / total_pop)
          * (0.50 + 0.50 * (homozyg_dom / (total_pop - 1.0) +
                            0.50 * (heterozyg - 1.0) / (total_pop - 1.0)))
      + (homozyg_rec / total_pop)
          * (homozyg_dom / (total_pop - 1.0) + 0.50 * (heterozyg / (total_pop - 1.0)));

    return answer;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_example() {
        assert!(p_dominant_phenotype(2.0, 2.0, 2.0) - 0.78333 <= 0.00001);
    }

    #[test]
    fn all_hom_recessive() {
        assert_eq!(0.00, p_dominant_phenotype(0.0, 0.0, 2.0));
    }

    #[test]
    fn all_hom_dominant() {
        assert_eq!(1.00, p_dominant_phenotype(2.0, 0.0, 0.0));
    }

    #[test]
    fn all_hetero() {
        assert_eq!(0.75, p_dominant_phenotype(0.0, 2.0, 0.0));
    }

    #[test]
    fn dom_and_hetero() {
        assert_eq!(1.00, p_dominant_phenotype(1.0, 1.0, 0.0));
    }

    #[test]
    fn dom_and_rec() {
        assert_eq!(1.00, p_dominant_phenotype(1.0, 0.0, 1.0));
    }

    #[test]
    fn het_and_rec() {
        assert_eq!(0.50, p_dominant_phenotype(0.0, 1.0, 1.0));
    }
}
