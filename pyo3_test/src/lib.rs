use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// creates a sample State
mod state;
use state::{Driver, Bundle, State};

#[pyfunction]
fn create_state() -> PyResult<()> {
    // create a basic state with 1 driver and bundle each
    let driver = Driver{id: "driver-123".to_string()};
    let bundle = Bundle{id: "bundle-456".to_string()};
    let drivers = vec![driver];
    let bundles = vec![bundle];
    let state = State::new(drivers, bundles);

    // print some info about the state
    state.drivers.get("driver-123").unwrap().hello();
    println!("State contains the following drivers:");
    for driver in state.drivers.values() {
        println!("{}", (*driver).id);
    }
    println!("State contains the following bundles:");
    for bundle in state.bundles.values() {
        println!("{}", (*bundle).id);
    }
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(create_state, m)?)?;
    Ok(())
}
