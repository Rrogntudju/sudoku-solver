// From Peter Norvig’s Sudoku solver     http://www.norvig.com/sudoku.html
use std::collections::{HashMap};

#[derive(Debug)]
struct Context {
    cols: Vec<char>,
    rows: Vec<char>,
    squares: Vec<String>,
    unitlist: Vec<Vec<String>>,
    units: HashMap<String, Vec<Vec<String>>>,
    peers: HashMap<String, Vec<String>>
}

fn cross (rows: &[char], cols: &[char]) -> Vec<String> {
    let mut v = Vec::new();
    for ch in rows {
        for d in cols {
            let mut sq = String::new();
            sq.push(*ch);
            sq.push(*d);
            v.push(sq)
        }
    }
    v
}

pub struct Sudoku {
    ctx: Context
}

impl Sudoku {
    pub fn new () -> Sudoku {
        Sudoku { ctx: Sudoku::context_init() }
    }

    fn context_init() -> Context {
        let cols: Vec<char> = "123456789".chars().collect();
        let rows: Vec<char> = "ABCDEFGHI".chars().collect();
        let squares = cross(&rows, &cols);
        // A vector of units (a unit is a column or a row or a box of 9 squares)
        let mut unitlist = Vec::<Vec<String>>::with_capacity(27);
        // columns
        for d in &cols {
            unitlist.push(cross(&rows, &[*d]));
        }
        // rows
        for ch in &rows {
            unitlist.push(cross(&[*ch], &cols));
        }
        // boxes
        for r in &[&rows[0..3], &rows[3..6], &rows[6..9]] {
            for c in &[&cols[0..3], &cols[3..6], &cols[6..9]] {
                unitlist.push(cross(*r, *c));
            }
        }
        //  units is a dictionary where each square maps to the list of units that contain the square  
        let mut units = HashMap::<String, Vec<Vec<String>>>::with_capacity(81);
        for s in &squares {
            let unit_s : Vec<Vec<String>> = unitlist.iter().cloned().filter(|u| u.contains(s)).collect();
            units.insert(s.clone(), unit_s);   
        }
        //  peers is a dictionary where each square s maps to the set of squares formed by the union of the squares in the units of s, but not s itself 
        let mut peers = HashMap::<String, Vec<String>>::with_capacity(81);
        for s in &squares {
            let mut peers_s : Vec<String> = units[s].concat().iter().cloned().filter(|p| p != s).collect();
            peers_s.sort();
            peers_s.dedup();
            peers.insert(s.clone(), peers_s);   
        }

        Context {cols: cols, rows: rows, squares: squares, unitlist: unitlist, units: units, peers: peers}
    }


    fn grid_values (&self, grid: &str) -> HashMap<String, Vec<char>> {
        //  Convert grid into a dict of (square, char Vec) with '0' or '.' for empties.
        let grid_chars: Vec<Vec<char>> = grid.chars().filter(|ch| self.ctx.cols.contains(ch) || ['0', '.'].contains(ch)).map(|ch| vec![ch]).collect();
        assert_eq!(grid_chars.len(), 81);
        let mut grid_values = HashMap::<String, Vec<char>>::new();
        grid_values.extend(self.ctx.squares.iter().cloned().zip(grid_chars.into_iter()));
        grid_values
    }

    fn parse_grid (&self, grid: &str) -> Option<HashMap<String, Vec<char>>> {
        //  Convert grid to Some dict of possible values, [square, digits], or return None if a contradiction is detected.
        let mut values = HashMap::<String, Vec<char>>::with_capacity(81);
        for s in &self.ctx.squares { 
            values.insert(s.clone(), self.ctx.cols.clone());
        }
        for (s, gvalues) in &self.grid_values(grid) {
            for d in gvalues {
                if self.ctx.cols.contains(d) && !self.assign(&mut values, s, d) {
                    return None;
                }
            }
        }
        Some(values)
    }

    fn assign (&self, values: &mut HashMap<String, Vec<char>>, s: &str, d: &char) -> bool {
        // Assign a value d by eliminating all the other values (except d) from values[s] and propagate. Return false if a contradiction is detected.  
        let other_values: Vec<char> = values[s].iter().cloned().filter(|d2| d2 != d).collect();
        other_values.iter().all(|d2| self.eliminate(values, s, d2))
    }

    fn eliminate (&self, values: &mut HashMap<String, Vec<char>>, s: &str, d: &char) -> bool {
        if !values[s].contains(d) {
            return true    // already eliminated
        }
        let i = values[s].iter().position(|d2| d2 == d).unwrap();
        values.get_mut(s).unwrap().remove(i);
        // (rule 1) If a square s is reduced to one value d2, then eliminate d2 from the peers.
        let d2 = values[s].clone();
        if d2.is_empty() {
            return false; // Contradiction: removed last value
        } 
        if d2.len() == 1 && !self.ctx.peers[s].iter().all(|s2| self.eliminate(values, s2, &d2[0])) {
            return false;
        }
        // (rule 2) If a unit u is reduced to only one place for a value d, then put it there.
        for u in &self.ctx.units[s] {
            let dplaces: Vec<String> = u.iter().cloned().filter(|s2| values[s2].contains(d)).collect();
            if dplaces.is_empty() {
                return false;   // Contradiction: no place for this value
            }
            // if d can only be in one place in unit assign it there
            if dplaces.len() == 1 && !self.assign(values, &dplaces[0], d) {
                return false;
            }
        }
        true
    }

    pub fn display (&self, values: &HashMap<String, Vec<char>>) -> Vec<String> {
        let width = 1 + (values.iter().map(|v| v.1.len()).max().unwrap());
        let line = ["-"; 3].iter().map(|c| c.repeat(3*width)).collect::<Vec<String>>().join("+");
        let mut lines = Vec::new();
        for r in &self.ctx.rows {
            lines.push(self.ctx.cols.iter()
                        .map(|c| {  let s = [*r, *c].iter().collect::<String>();
                                    format!("{0: ^1$}", values[&s].iter().collect::<String>(), width) + (if ['3', '6'].contains(c) {"|"} else {""})})                                             
                        .collect::<String>());
            if ['C', 'F'].contains(r) {
                lines.push(line.clone());
            }
        }
        lines
    }

    fn search (&self, values: HashMap<String, Vec<char>>) -> Option<HashMap<String, Vec<char>>> {
        // Using depth-first search and propagation, try all possible values
        if values.iter().all(|(_, v)| v.len() == 1) {
            return Some(values);  // Solved!
        }
        // Choose the unfilled square s with the fewest possibilities
        let (_, s) = values.iter().filter(|&(_, v)| v.len() > 1).map(|(s, v)| (v.len(), s)).min().unwrap();
        for d in &values[s] {
            let mut cloned_values = values.clone();
            if self.assign(&mut cloned_values, s, d) {
                if let Some(svalues) = self.search(cloned_values) {
                    return Some(svalues);
                }  
            }
        }
        None
    }

    pub fn solve (&self, grid: &str) -> Option<HashMap<String, Vec<char>>> {
        self.parse_grid(grid).and_then(|v| self.search(v))
    }

    fn solved (values: &HashMap<String, Vec<char>>, ctx: &Context) -> bool {
        //  A puzzle is solved if each unit is a permutation of the digits 1 to 9.  
        let unitsolved = |unit: &Vec<String>| {
            let mut digits_values = unit.iter().map(|s| values[s].iter().collect::<String>()).collect::<Vec<String>>();
            digits_values.sort();
            digits_values == ctx.cols.iter().map(char::to_string).collect::<Vec<String>>()
        };
        ctx.unitlist.iter().all(|u| unitsolved(u))
    }  
}


