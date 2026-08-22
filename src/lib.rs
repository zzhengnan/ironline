use pyo3::prelude::*;

/// A linear regression library implemented in Rust.
#[pymodule]
mod ironline {
    use pyo3::prelude::*;

    #[pyfunction]
    fn compute_mean(numbers: Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for number in &numbers {
            sum += number;
        }
        let len = numbers.len() as f64;
        sum / len
    }
}
