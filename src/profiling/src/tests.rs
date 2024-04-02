#[cfg(test)]
mod tests {
    use crate::*;
    use rand::Rng;
    use rand_chacha::ChaCha8Rng;
    use rand_chacha::rand_core::SeedableRng;
    use test_case::test_matrix;

    fn generate_random_numbers(count: usize, value_range: (f64, f64)) -> Vec<f64> {
        let mut rng = ChaCha8Rng::seed_from_u64(293184945190798259);
        let mut numbers = Vec::with_capacity(count);
        for _ in 0..count {
            numbers.push(rng.gen_range(value_range.0..=value_range.1));
        }
        numbers
    }

    fn join_numbers(numbers: Vec<f64>) -> String {
        let mut result = String::new();
        for number in numbers {
            result.push_str(&number.to_string());
            result.push(' ');
        }
        result.pop();
        result
    }

    #[test_matrix(
        [10, 1000, 1_000_000],
        [(0.0, 10.0), (0.0, 1000.0), (0.0, 1_000_000.0),
         (500.0, 1000.0), (1000.0, 50_000.0), (1_000_000.0, 50_000_000.0)]
    )]
    fn test(count: usize, value_range: (f64, f64)) {
        let a = generate_random_numbers(count, value_range);
        let b = join_numbers(a);
        let c = prepare_to_evaluate(b);
        let d = evaluate_parse_tree(c);
        assert!(d.is_ok());
    }
}