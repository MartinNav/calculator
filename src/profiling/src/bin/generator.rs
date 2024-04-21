use std::io::Write;
use rand::Rng;

/// Generates a test case file with random numbers for profiler
///
/// # Arguments
///
/// * `number_count` - The number of random numbers to generate
/// * `min_value` - The minimum value of the random numbers
/// * `max_value` - The maximum value of the random numbers
///
/// If one of the range values is a floating point number, the generated numbers will also be floating point numbers.
///
/// # Example
///
/// ```shell
/// cargo run --bin generator 1000000 0 100
/// ```
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!("Usage: {} <number_count> <min_value> <max_value>", args[0]);
        std::process::exit(1);
    }

    let mut is_float = false;
    let number_count = args[1].parse::<usize>().unwrap();

    let val = args[2].parse::<i64>();
    let min_value = match val {
        Ok(v) => v as f64,
        Err(_) => {
            is_float = true;
            args[2].parse::<f64>().unwrap()
        }
    };

    let val = args[3].parse::<i64>();
    let max_value = match val {
        Ok(v) => v as f64,
        Err(_) => {
            is_float = true;
            args[3].parse::<f64>().unwrap()
        }
    };

    let mut rng = rand::thread_rng();

    let mut file = std::fs::File::create(format!("test_case_{number_count}.txt")).unwrap();
    if is_float {
        for i in 0..number_count {
            let n = rng.gen_range(min_value..=max_value);
            write!(file, "{}", n).unwrap();
            if i < number_count - 1 {
                write!(file, " ").unwrap();
            }
        }
    } else {
        for i in 0..number_count {
            let n = rng.gen_range(min_value as i64..=max_value as i64);
            write!(file, "{}", n).unwrap();
            if i < number_count - 1 {
                write!(file, " ").unwrap();
            }
        }
    }
}