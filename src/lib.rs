mod sudoku;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use sudoku::Sudoku;

#[pyfunction]
fn solve(grid: String) -> PyResult<String> {
    let sdk = Sudoku::new();
    sdk.solve(&grid).map_err(|e| PyErr::new::<ValueError, _>(format!("{}", e)))
}

#[pyfunction]
fn display(grid: String) -> PyResult<()> {
    let lines = Sudoku::display(&grid).map_err(|e| PyErr::new::<ValueError, _>(format!("{}", e)))?;
    lines.iter().for_each(|l| println!("{}", l));
    Ok(())
}

#[pymodule]
fn sudoku_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(solve))?;
    m.add_wrapped(wrap_pyfunction!(display))?;

    Ok(())
}
