use std::io::Write;
use rand::Rng;
use regex::Regex;

/// Generates a test case file with random numbers for profiler
///
/// # Arguments
///
/// * `file_name` - The file to write into (has to contain a number to specify a count to generate)
/// * `min_value` - The minimum value of the random numbers
/// * `max_value` - The maximum value of the random numbers
///
/// If one of the range values is a floating point number, the generated numbers will also be floating point numbers.
///
/// # Example
///
/// ```shell
/// cargo run --bin generator test_case_1000000.txt 0 100
/// ```
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!("Usage: {} <file_name> <min_value> <max_value>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];

    let re = Regex::new(r"^.*?(\d+)[^\d/\\]*$").unwrap();
    let captures = re.captures(file_name);
    if captures.is_none() {
        eprintln!("The file name has to contain a number to specify a count to generate.");
        std::process::exit(1);
    }
    let captures = captures.unwrap();

    let number_str = captures.get(1).unwrap().as_str();
    let number_count = number_str.parse::<usize>().unwrap();

    let mut is_float = false;
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

    // create directory
    let path = std::path::Path::new(file_name);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = std::fs::File::create(file_name).unwrap();
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
