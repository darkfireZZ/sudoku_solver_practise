# A Simple Sudoku Solver

I coded this to get more comfortable with Rust. The code is probably not very clean or
efficient (I didn't know a lot about Rust or Sudoku solving when I coded this).

The solver is implemented as a simple backtracking algorithm with some minor optimizations.

# Examples

```rust
use sudoku::Sudoku;

// Initialize a Sudoku
// Sudoku taken from https://www.conceptispuzzles.com/index.aspx?uri=info/article/424
let a_difficult_sudoku = Sudoku::new_from_array([8, 0, 0, 0, 0, 0, 0, 0, 0,
                                                 0, 0, 3, 6, 0, 0, 0, 0, 0,
                                                 0, 7, 0, 0, 9, 0, 2, 0, 0,
                                                 0, 5, 0, 0, 0, 7, 0, 0, 0,
                                                 0, 0, 0, 0, 4, 5, 7, 0, 0,
                                                 0, 0, 0, 1, 0, 0, 0, 3, 0,
                                                 0, 0, 1, 0, 0, 0, 0, 6, 8,
                                                 0, 0, 8, 5, 0, 0, 0, 1, 0,
                                                 0, 9, 0, 0, 0, 0, 4, 0, 0]);

// Find a solution
let solution = a_difficult_sudoku.find_solution()
                                 .expect("there exists exactly 1 solution");

// Check if it's actually solved
assert!(solution.is_solved());
```

If multiple solutions exist, an iterator over all possible solutions can be obtained:
```rust
use sudoku::Sudoku;

// Initialize a Sudoku
// Sudoku taken from https://math.stackexchange.com/questions/813444/sudoku-puzzle-with-exactly-3-solutions
let is_this_a_sudoku = Sudoku::new_from_array([3, 0, 9, 6, 0, 0, 4, 0, 0,
                                               0, 0, 0, 7, 0, 9, 0, 0, 0,
                                               0, 8, 7, 0, 0, 0, 0, 0, 0,
                                               7, 5, 0, 0, 6, 0, 2, 3, 0,
                                               6, 0, 0, 9, 0, 4, 0, 0, 8,
                                               0, 2, 8, 0, 5, 0, 0, 4, 1,
                                               0, 0, 0, 0, 0, 0, 5, 9, 0,
                                               0, 0, 0, 1, 9, 6, 0, 0, 7,
                                               0, 0, 6, 0, 0, 0, 1, 0, 4]);

// Find all solutions
let solutions = is_this_a_sudoku.find_all_solutions();

assert_eq!(solutions.count(), 161);
```
