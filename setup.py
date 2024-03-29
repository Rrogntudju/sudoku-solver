from setuptools import setup
from setuptools_rust import Binding, RustExtension, Strip

setup(name='sudoku-solver',
      version='2.0',
      rust_extensions=[RustExtension('sudoku_solver.sudoku_solver',
                                     'Cargo.toml', binding=Binding.PyO3,
                                     strip=Strip.All)],
      packages=['sudoku_solver'],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False
      )