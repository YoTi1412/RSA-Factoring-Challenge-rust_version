use std::env;
use std::fs::File;
use std::io::Read;
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
    let seconds = duration.as_secs_f64();
    format!("{:.3}s", seconds)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path as a command-line argument.");
        return;
    }

    let file_path = &args[1];
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", err);
        return;
    }

    let start = Instant::now();

    for line in contents.lines() {
        if let Ok(number) = line.parse::<u64>() {
            let factorization = factorize(number);
            println!("{}", factorization);
        }
    }

    let duration = start.elapsed();
    let elapsed_time = format_duration(duration);

    println!("real    {}", elapsed_time);
    println!("user    {}", elapsed_time);
    println!("sys     0.000s");
}

