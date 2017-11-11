import sudoku_solver.libsudokusolver as ss

with open('./puzzles.txt') as f:
    CONTENT = f.read().splitlines()
solved = 0
for line in CONTENT:
    try:
        ss.display(line)
        print('')
        ss.display(ss.solve(line))
        solved = solved + 1
    except ValueError as ex:
        print(line)
        print(ex.message)
    print('')
print('{0}/{1} solved puzzles'.format(solved, len(CONTENT)))
