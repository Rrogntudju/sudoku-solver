mod sudoku;

use lazy_static::lazy_static;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use sudoku::{legos, Sudoku};

lazy_static! {
    static ref BRICKS: (Vec<char>, Vec<char>, Vec<String>, Vec<Vec<String>>) = legos();
}

#[pyclass]
struct Solver {
    ss: Sudoku<'static>,
}

#[pymethods]
impl Solver {
    #[new]
    fn new() -> Self {
        let ss = Sudoku::new(BRICKS.0.clone(), BRICKS.1.clone(), &BRICKS.2, &BRICKS.3);
        Solver { ss }
    }

    fn solve(&self, grid: &str) -> PyResult<String> {
        self.ss.solve(grid).map_err(|e| PyErr::new::<PyValueError, _>(format!("{}", e)))
    }

    #[staticmethod]
    fn display(grid: &str) -> PyResult<()> {
        let lines = Sudoku::display(grid).map_err(|e| PyErr::new::<PyValueError, _>(format!("{}", e)))?;
        lines.iter().for_each(|l| println!("{}", l));

        Ok(())
    }
}

#[pymodule]
fn sudoku_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Solver>()?;

    Ok(())
}
