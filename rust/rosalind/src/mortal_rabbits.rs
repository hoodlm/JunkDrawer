use std::env;
use std::fs;
use std::collections::VecDeque;

// cargo run --bin moral_rabbits -- /tmp/rabbits.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = filename_from_args(&args);
    let (n_months, lifespan_months) = read_config(&filename);

    let result = simulate(n_months, lifespan_months);
    println!("{result}");
}

fn filename_from_args(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!("Expected exactly 1 argument");
    }
    &args[1]
}

fn read_config(filename: &str) -> (u32, usize) {
    let raw_config = fs::read_to_string(&filename).unwrap();
    let elements: Vec<&str> = raw_config.split_whitespace().collect();
    if elements.len() != 2 {
        panic!("Malformed config in {filename}; expected two whitespace separated elements like '5 3' ");
    }

    (elements[0].parse().unwrap(), elements[1].parse().unwrap())
}

fn simulate(n_months: u32, lifespan_months: usize) -> u64 {
    // Initial population vector is size M (lifespan_months). Each index tracks population of a generation.
    let mut population_vector: VecDeque<u64> = VecDeque::with_capacity(lifespan_months);
    // Initialize with zeroes...
    for _ in 0..lifespan_months {
        population_vector.push_front(0);
    }
    // Then our initial juvenile pair:
    population_vector[0] = 1;

    simulate_iter(n_months, &mut population_vector)
}

fn simulate_iter(months_left: u32, population_vector: &mut VecDeque::<u64>) -> u64 {
    eprintln!("{}, pop_vector {:?}", months_left, population_vector);
    let total_pop = population_vector.iter().sum();
    if months_left <= 1 {
        return total_pop;
    } else {
        let juvenile_pop = population_vector[0];
        let adult_pop = total_pop - juvenile_pop;

        // K = 1 in this model; each adult pair produces exactly one baby pair
        let new_babies = adult_pop;

        // Remove oldest generation, add next generation
        population_vector.pop_back();
        population_vector.push_front(new_babies);
        return simulate_iter(months_left - 1, population_vector);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_case() {
        assert_eq!(4, simulate(6, 3));
    }
}
