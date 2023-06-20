use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn factorize_prime(mut n: u64) -> String {
    let mut factors = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut p = 3;
    while p * p <= n {
        if n % p == 0 {
            factors.push(p);
            n /= p;
        } else {
            p += 2;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    let factorization = factors
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("*");

    format!("{}={}", n * factors[0], factorization)
}

fn process_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(number) = line?.parse::<u64>() {
            let factorization = factorize_prime(number);
            println!("{}", factorization);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path as a command-line argument.");
        return;
    }

    let file_path = &args[1];

    if let Err(err) = process_file(file_path) {
        eprintln!("Error processing file: {}", err);
        return;
    }
}

