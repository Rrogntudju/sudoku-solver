from sudoku_solver import Solver
from pathlib import Path

parent = Path(__file__).resolve().parent
with open(parent.joinpath('puzzles.txt')) as f:
    CONTENT = f.read().splitlines()
solved = 0
ss = Solver()
for line in CONTENT:
    try:
        Solver.display(line)
        print('')
        Solver.display(ss.solve(line))
        solved += 1
    except ValueError as ex:
        print(line)
        print(ex.message)
    print('')
print('{0}/{1} solved puzzles'.format(solved, len(CONTENT)))
