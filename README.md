# What Is This?

This is a Sudoku solver. There is a bunch of other neat little features this
library has, but its mostly just a solver.

I coded this to get more comfortable with Rust. Consequently, this library is
likely very far from optimal, both in terms of coding practises and the solver
algorithm.

If you want to use this for any reason, go ahead.

# How Does The Solver Work?

Note: My knowledge of Sudoku solving is very limited. I know the rules, but
that's it. I have no idea of the mathematical concepts and the algorithms in
the Sudoku world and also didn't feel like reading up on any. I expect my
algorithm to be anything but optimal and my terminology to be quite off.

The algorithm used for the solver is a simple backtracking algorithm with one
small addition: The solver recognizes squares that can only possibly contain a
single value based on the already filled-in squares in the row, column and
surrounding 3x3 cell of the square and fill them in. This should significantly
speed up the solver (I haven't actually tested this though).

I've chosen an iterative implementation for the solver. This makes it possible
to have a function that returns an [Iterator] of solutions
([Sudoku::find_all_solutions()]), which could be useful in cases where
multiple solutions exist and one wants to find a bunch of them. More
importantly though, it was a fun challange.

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
