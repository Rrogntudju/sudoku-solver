mod sudoku;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use sudoku::Sudoku;

#[pymodule]
fn libsudokusolver(py: Python, m: &PyModule) -> PyResult<()> {
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

    Ok(())
}
