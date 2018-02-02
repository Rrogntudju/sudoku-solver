# sudoku-solver
 A Rust extension for Python.
 ### To build the extension:
 1. Install [Rustup](https://github.com/rust-lang-nursery/rustup.rs#installation) with the nightly toolchain. Note: the nightly toolchain is required for PyO3.
 2. pip install setuptools_rust --user
 3. python setup.py bdist_wheel
 ### To install the extension:
 pip install  */path_to_whl_file/whl_generated_from_build.whl*  --user
### To test the extension:
python test_ss.py
### To solve an evil puzzle from [Web Sudoku](http://www.websudoku.com/?level=4) Note: requires BeautifulSoup
python websudoku.py 