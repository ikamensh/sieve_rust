use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let start = Instant::now();

    let mut factors = HashMap::new();
    let mut n = 2u32;
    let mut primes = vec![];

    const SEARCH_MAX: u32 = 100;
//    const SEARCH_MAX: u32 = 100_000_000;
    let max_sqrt = (f64::powf(SEARCH_MAX as f64, 0.5) + 1.) as u32;

    // let mut to_push = Vec::new();

    while n < SEARCH_MAX {
        let fs = factors.get(&n).clone();
        match fs {
            None => {
                primes.push(n);
                if n < max_sqrt {
                    factors.insert(n * n, vec![n]);
                }
            }
            Some(v) => {
                for factor in v {
                    let not_prime = *factor + n;
                    let maybe_existing_factors = factors.get_mut(&not_prime);
                    match maybe_existing_factors {
                        None => {factors.insert(not_prime, vec![*factor]);}
                        Some(existing_factors) => {existing_factors.push(*factor);}
                    }
                }
                factors.remove(&n);
            }
        }

        //        if n % 100_000 == 0 {
        //            println!("Found {} primes up to the number {}", primes.len(), n);
        //        }

        n += 1;
    }

    println!("It took {} milliseconds", start.elapsed().as_millis());
    println!(
        "Found {} primes, with biggest being {}",
        primes.len(),
        primes.pop().unwrap()
    );
//
//        println!("{:?}", primes);
}
