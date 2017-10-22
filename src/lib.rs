#![feature(proc_macro, specialization)]
extern crate pyo3;

mod sudoku;

use pyo3::prelude::*;
use sudoku::Sudoku;

#[py::modinit(sudokusolver)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "solve")]
    fn solve(_py: Python, grid: String) -> PyResult<String> {
        let sdk = Sudoku::new(); 
        sdk.solve(&grid).map_err(|e| {PyErr::new::<exc::ValueError, _>(format!("{}", e))})
    }

    #[pyfn(m, "display")]
    fn display(_py: Python, grid: String) -> PyResult<()> {
        let lines = Sudoku::display(&grid).map_err(|e| {PyErr::new::<exc::ValueError, _>(format!("{}", e))});
        lines.unwrap().iter().for_each(|l| {println!("{}", l)});
        Ok(())
    }

    Ok(())
}
