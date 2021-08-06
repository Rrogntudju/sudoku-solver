from bs4 import BeautifulSoup
import urllib3
from sudoku_solver import Solver

http = urllib3.PoolManager()

# Extract the evil puzzle link from the frame
frameset_page = http.request("get", "http://www.websudoku.com/?level=4")
soup = BeautifulSoup(frameset_page.data, "html.parser")
puzzle_link = soup("frame")[0]["src"] 

# Extract the puzzle's values and add the empties
puzzle_page = http.request("get", puzzle_link)
soup = BeautifulSoup(puzzle_page.data, "html.parser")
puzzle_values = {tag["id"] : tag["value"] for tag in soup("input", readonly=True)}
ids = ["f" + str(j) + str(i) for i in range(9) for j in range(9)]
puzzle = "".join([puzzle_values[id] if id in puzzle_values else "0" for id in ids])

# Solve
Solver.display(puzzle)
print(" ")
Solver.display(Solver().solve(puzzle))