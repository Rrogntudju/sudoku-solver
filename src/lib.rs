use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use sudoku::solver::Sudoku;

#[pyclass]
struct Solver {
    ss: Sudoku<'static>,
}

#[pymethods]
impl Solver {
    #[new]
    fn new() -> Self {
        let ss = Sudoku::new();
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
