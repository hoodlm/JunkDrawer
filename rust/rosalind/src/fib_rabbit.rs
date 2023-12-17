use std::env;
use std::fs;

// cargo run --bin fib_rabbit -- /tmp/rabbits.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let (n_months, k_rabbit_pairs_per_litter) = read_config(&filename);

    let result = simulate(n_months, k_rabbit_pairs_per_litter);
    println!("{result}");
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn read_config(filename: &str) -> (u64, u64) {
    let raw_config = fs::read_to_string(&filename).unwrap();
    let elements: Vec<&str> = raw_config.split_whitespace().collect();
    if elements.len() != 2 {
        panic!("Malformed config in {filename}; expected two whitespace separated elements like '5 3' ");
    }

    (elements[0].parse().unwrap(), elements[1].parse().unwrap())
}

/* Recurrence relation for this problem is:
 * F(N) = F(N-1) + K*F(N-2)
 *
 * Where F(N-2) is the number of mature rabbits,
 * and F(N-1) is the number of juvenile rabbits
 * that count toward the population but are not
 * at reproduction-age yet.
 */
fn simulate(n_months: u64, k_rabbit_pairs_per_litter: u64) -> u64 {
    // The problem is ambiguous about whether the first pair is juvenile or mature,
    // but based on sample dataset/output F(0) = 1, F(1) = 1
    simulate_iter(n_months, 1, 0, k_rabbit_pairs_per_litter)
}

fn simulate_iter(months_left: u64, juvenile_pop: u64, mature_pop: u64, rabbit_pairs_per_litter_per_month: u64) -> u64 {
    if months_left <= 1 {
        return juvenile_pop + mature_pop;
    } else {
        let baby_rabbits = mature_pop * rabbit_pairs_per_litter_per_month;
        let next_adult_pop = juvenile_pop + mature_pop;
        return simulate_iter(months_left - 1, baby_rabbits, next_adult_pop, rabbit_pairs_per_litter_per_month);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_case() {
        assert_eq!(19, simulate(5, 3));
    }
}
