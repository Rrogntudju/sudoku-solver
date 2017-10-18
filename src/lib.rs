#![feature(proc_macro, specialization, const_fn)]
extern crate pyo3;

mod sudoku;

use pyo3::prelude::*;

use sudoku::Sudoku;

#[py::modinit(sudoku)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "solve")]
    fn solve(py: Python, grid: String) -> PyResult<String> {
        let sdk = Sudoku::new();
        sdk.solve(&grid).map_err(|e| {PyErr::new::<exc::ValueError, _>(format!("{}", e))})
    }

    Ok(())
}
