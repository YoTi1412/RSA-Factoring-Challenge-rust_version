use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::{Instant, Duration};

fn factorize(n: u64) -> String {
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            let q = n / p;
            return format!("{}={}*{}", n, q, p);
        }
        p += 1;
    }
    format!("{}={}*{}", n, n, 1)
}

fn format_duration(duration: Duration) -> String {
    format!("{:.3}s", duration.as_secs_f64())
}

fn process_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(number) = line?.parse::<u64>() {
            let factorization = factorize(number);
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
    let start = Instant::now();

    if let Err(err) = process_file(file_path) {
        eprintln!("Error processing file: {}", err);
        return;
    }

    let duration = start.elapsed();
    let elapsed_time = format_duration(duration);

    println!("real    {}", elapsed_time);
    println!("user    {}", elapsed_time);
    println!("sys     0.000s");
}
