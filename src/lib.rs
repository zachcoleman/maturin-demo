use ndarray::*;
use numpy::*;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// looping sum
#[pyfunction]
fn sum_loop(_py: Python<'_>, arr: &PyArrayDyn<f64>) -> f64 {
    let mut res: f64 = 0.;
    for v in arr.readonly().as_array().iter(){
        res = res + v;
    }
    return res;
}

/// fast sum
#[pyfunction]
fn sum(_py: Python<'_>, arr: &PyArrayDyn<f64>) -> f64 {
    return arr.readonly().as_array().sum();
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_loop, m)?)?;
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    Ok(())
}