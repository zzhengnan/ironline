use pyo3::prelude::*;

/// A linear regression library implemented in Rust.
#[pymodule]
mod ironline {
    use pyo3::prelude::*;

    #[pyfunction]
    pub fn compute_mean(numbers: Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for number in &numbers {
            sum += number;
        }
        let len = numbers.len() as f64;
        sum / len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let result = ironline::compute_mean(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}
