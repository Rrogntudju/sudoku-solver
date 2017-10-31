from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='sudoku-solver',
      version='1.0',
      rust_extensions=[RustExtension('sudoku_solver.libsudokusolver',
                                     'Cargo.toml', binding=Binding.PyO3)],
      packages=['sudoku_solver'],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False
)