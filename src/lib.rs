
#![doc = include_str!("../README.md")]

#![warn(missing_docs)]

use itertools::Itertools;

/// The number of squares on a Sudoku grid.
pub const NUM_SQUARES: usize = 9 * 9;

/// Panic if either coordinate is >= 9.
fn validate_coordinates(x: usize, y: usize) {
    if x > 8 || y > 8 {
        panic!("x and y must both be <= 8 (x = {}, y = {})", x, y);
    }
}

/// Panic if the value is > 9.
fn validate_value(value: u32) {
    if value > 9 {
        panic!("Value must be <= 9 (was {})", value );
    }
}

/// A Sudoku grid
///
/// Holds a 9 x 9 grid of squares containing values ranging from 0 to 9. A
/// value of 0 means that the square is considered empty.
///
/// Each square in the grid is uniquely identified by a pair of x and y
/// coordinates ranging from 0 to 8 each. The point of origin (x = 0, y = 0) is
/// in the top left corner. The x axis signifies horizontal offset from the
/// origin, whereas the y axis signifies vertical offset.
///
/// This implementation guarantees that values cannot be bigger than 9 and
/// panics if supplied with any. It also panics if invalid coordinates are
/// supplied.
// TODO the derived Debug implementation is very ugly, maybe manually implement
// it
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sudoku {
    grid: [u32; 81],
}

impl Sudoku {

    /// Initialize a new Sudoku board from an array.
    ///
    /// This function allows you to do this:
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([4, 3, 0, 0, 0, 9, 8, 0, 0,
    ///                                      1, 9, 0, 8, 0, 0, 0, 0, 5,
    ///                                      0, 0, 0, 7, 2, 4, 0, 0, 0,
    ///                                      0, 6, 1, 0, 9, 0, 0, 2, 0,
    ///                                      0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                      0, 2, 0, 0, 5, 0, 6, 9, 0,
    ///                                      0, 0, 0, 6, 3, 7, 0, 0, 0,
    ///                                      8, 0, 0, 0, 0, 5, 0, 3, 1,
    ///                                      0, 0, 5, 1, 0, 0, 0, 7, 6]);
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(sudoku.is_solvable());
    /// ```
    ///
    /// And here a mathematically more precise description:
    ///
    /// If `i` is an index into the array, the value at `array[i]` will end up at the coordinates ( `x = i % 9` /
    /// `y = i / 9` ).
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let mut sudoku_array = [0; sudoku::NUM_SQUARES];
    /// let i = 54;
    /// let x = i % 9;
    /// let y = i / 9;
    ///
    /// sudoku_array[i] = 7;
    ///
    /// let sudoku = Sudoku::new_from_array(sudoku_array);
    ///
    /// assert_eq!(sudoku_array[i], sudoku.get_value(x, y));
    /// ```
    pub fn new_from_array(array: [u32; NUM_SQUARES]) -> Sudoku {
        for value in array {
            validate_value(value);
        }

        Sudoku {
            grid: array,
        }
    }

    /// Initialize a new empty Sudoku board.
    ///
    /// "Empty board" means that all squares contain a value of 0.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let sudoku_0 = Sudoku::new_empty();
    ///
    /// let sudoku_1 = Sudoku::new_from_array([0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                        0, 0, 0, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(sudoku_0, sudoku_1);
    /// ```
    pub fn new_empty() -> Sudoku {
        Sudoku {
            grid: [0; NUM_SQUARES],
        }
    }

    /// Get the value at the given coordinates in the Sudoku grid.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([0, 0, 0, 0, 7, 5, 8, 1, 0,
    ///                                      0, 0, 2, 4, 0, 0, 7, 0, 0,
    ///                                      0, 0, 0, 0, 3, 0, 0, 4, 0,
    ///                                      2, 0, 0, 0, 0, 7, 0, 5, 3,
    ///                                      0, 1, 0, 2, 0, 9, 0, 8, 0,
    ///                                      5, 9, 0, 3, 0, 0, 0, 0, 4,
    ///                                      0, 2, 0, 0, 6, 0, 0, 0, 0,
    ///                                      0, 0, 9, 0, 0, 4, 3, 0, 0,
    ///                                      0, 4, 8, 7, 1, 0, 0, 0, 0]);
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(sudoku.is_solvable());
    ///
    /// assert_eq!(sudoku.get_value(4, 0), 7);
    ///
    /// assert_eq!(sudoku.get_value(3, 5), 3);
    /// ```
    ///
    /// Panics if the coordinates are out of bounds.
    pub fn get_value(&self, x: usize, y: usize) -> u32 {
        validate_coordinates(x, y);

        self.grid[x + y * 9]
    }

    /// Set the value at the given coordinates in the Sudoku grid.
    ///
    /// Pancis if the coordinates are out of bounds or if `value` is invalid.
    pub fn set_value(&mut self, x: usize, y: usize, value: u32) {
        validate_coordinates(x, y);
        validate_value(value);

        self.grid[x + y * 9] = value;
    }

    /// Check if this [Sudoku] is solved.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let solved_sudoku = Sudoku::new_from_array([5, 6, 1, 9, 8, 3, 2, 7, 4,
    ///                                             9, 7, 4, 5, 1, 2, 3, 6, 8,
    ///                                             8, 2, 3, 4, 6, 7, 5, 9, 1,
    ///                                             3, 1, 9, 6, 7, 5, 4, 8, 2,
    ///                                             4, 8, 2, 3, 9, 1, 7, 5, 6,
    ///                                             6, 5, 7, 8, 2, 4, 1, 3, 9,
    ///                                             7, 4, 8, 2, 5, 6, 9, 1, 3,
    ///                                             1, 3, 6, 7, 4, 9, 8, 2, 5,
    ///                                             2, 9, 5, 1, 3, 8, 6, 4, 7,]);
    ///
    /// assert!(solved_sudoku.is_solved());
    /// ```
    ///
    /// A Sudoku grid is considered solved if it has no empty squares and
    /// contains no duplicate values within any row, column or any of the 9 3x3
    /// cells.
    pub fn is_solved(&self) -> bool {
        !self.has_empty_squares() && self.is_valid()
    }

    /// Find a solution for this [Sudoku] puzzle.
    ///
    /// If there exist one or more possible solutions for this puzzle, this
    /// function will return one of them. Else, `None` will be returned.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://opensky.ca/sudoku
    /// let puzzle = Sudoku::new_from_array([7, 0, 6, 0, 0, 0, 0, 0, 0,
    ///                                      0, 2, 0, 0, 0, 9, 6, 1, 0,
    ///                                      0, 0, 0, 6, 5, 0, 0, 0, 3,
    ///                                      9, 0, 0, 4, 3, 5, 2, 0, 0,
    ///                                      8, 0, 0, 0, 9, 0, 0, 0, 5,
    ///                                      0, 0, 3, 1, 2, 8, 0, 0, 4,
    ///                                      4, 0, 0, 0, 8, 2, 0, 0, 0,
    ///                                      0, 6, 8, 3, 0, 0, 0, 4, 0,
    ///                                      0, 0, 0, 0, 0, 0, 5, 0, 1]);
    ///
    /// let solution = puzzle.find_solution();
    ///
    /// assert!(solution.expect("There exists a solution").is_solved());
    /// ```
    ///
    /// If the given [Sudoku] is invalid (meaning it contains the same value twice
    /// in the same row, column or 3x3 cell), None will be returned.
    ///
    /// This function does not make any guarantees about which solution it returns
    /// if multiple exist and the solution returned may change across different
    /// versions of this crate.
    pub fn find_solution(&self) -> Option<Sudoku> {
        self.find_all_solutions().next()
    }

    /// Find all solutions for this [Sudoku] puzzle.
    ///
    /// Return an [Iterator] of all the possible solutions for the [Sudoku].
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let sudoku = Sudoku::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      0, 0, 0, 3, 0, 0, 6, 0, 0,
    ///                                      0, 0, 0, 2, 0, 0, 5, 0, 0,
    ///                                      0, 0, 0, 1, 0, 0, 4, 0, 0,
    ///                                      0, 0, 0, 9, 0, 0, 3, 0, 0,
    ///                                      0, 0, 0, 8, 0, 0, 2, 0, 0,
    ///                                      0, 0, 0, 7, 0, 0, 1, 0, 0,
    ///                                      0, 0, 0, 6, 0, 0, 9, 0, 0,
    ///                                      0, 0, 0, 5, 0, 0, 8, 0, 0]);
    ///
    /// let mut all_solutions = sudoku.find_all_solutions();
    ///
    /// # // Make sure that there exist at least 2 solutions for `sudoku`
    /// #
    /// # let solution_0 = all_solutions.next().expect("There should exist a solution");
    /// # assert!(solution_0.is_solved());
    /// #
    /// # let solution_1 = all_solutions.next().expect("There should exist a second solution");
    /// # assert!(solution_1.is_solved());
    /// ```
    ///
    /// This function does not make any guarantees about the order of the solutions
    /// generated and the order may change across different versions of this crate.
    ///
    /// Avoid using functions like [Iterator::count()] or [Iterator::collect()] on
    /// the return value of this function unless you are certain that the number of
    /// possible solutions is very limited. Otherwise you'll likely get stuck in an
    /// almost infinite loop.
    pub fn find_all_solutions(&self) -> impl Iterator<Item = Sudoku> + '_ {
        AllSolutionsIterator::new(self)
    }

    /// Return `true` if this [Sudoku] is solvable.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([0, 0, 1, 0, 2, 0, 9, 0, 0,
    ///                                      9, 0, 0, 0, 4, 0, 0, 2, 0,
    ///                                      0, 2, 0, 0, 9, 8, 0, 5, 1,
    ///                                      0, 1, 7, 0, 0, 0, 0, 0, 0,
    ///                                      4, 0, 0, 7, 0, 6, 0, 0, 9,
    ///                                      0, 0, 0, 0, 0, 0, 6, 1, 0,
    ///                                      1, 3, 0, 8, 7, 0, 0, 6, 0,
    ///                                      0, 7, 0, 0, 5, 0, 0, 0, 4,
    ///                                      0, 0, 5, 0, 6, 0, 3, 0, 0]);
    ///
    /// assert!(sudoku.is_solvable());
    /// ```
    ///
    /// This is just syntactic sugar for [Sudoku::find_solution()] and then
    /// checking whether the returned [Option] is `None`.
    ///
    /// ```
    /// # use sudoku::Sudoku;
    /// #
    /// # // Values generated with http://www.opensky.ca/sudoku
    /// # let sudoku = Sudoku::new_from_array([0, 0, 1, 0, 2, 0, 9, 0, 0,
    /// #                                      9, 0, 0, 0, 4, 0, 0, 2, 0,
    /// #                                      0, 2, 0, 0, 9, 8, 0, 5, 1,
    /// #                                      0, 1, 7, 0, 0, 0, 0, 0, 0,
    /// #                                      4, 0, 0, 7, 0, 6, 0, 0, 9,
    /// #                                      0, 0, 0, 0, 0, 0, 6, 1, 0,
    /// #                                      1, 3, 0, 8, 7, 0, 0, 6, 0,
    /// #                                      0, 7, 0, 0, 5, 0, 0, 0, 4,
    /// #                                      0, 0, 5, 0, 6, 0, 3, 0, 0]);
    /// #
    /// # assert!(sudoku.is_solvable());
    /// #
    /// let is_solvable = sudoku.find_solution().is_some(); // equivalent to sudoku.is_solvable()
    /// ```
    pub fn is_solvable(&self) -> bool {
        self.find_solution().is_some()
    }

    /// Check if this [Sudoku] is valid.
    ///
    /// A [Sudoku] is considered valid if it contains no duplicate values
    /// within any row, column or any of the 9 3x3 cells.
    ///
    /// IMPORTANT: Valid does not imply solvable, a [Sudoku] may well be valid
    /// but unsolvable. To check for solvability see [Sudoku::is_solvable()].
    ///
    /// Example of a valid [Sudoku]:
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let valid_sudoku = Sudoku::new_from_array([0, 6, 0, 0, 0, 0, 0, 0, 0,
    ///                                            9, 0, 0, 3, 6, 8, 4, 0, 0,
    ///                                            7, 0, 0, 0, 1, 0, 9, 0, 0,
    ///                                            1, 0, 0, 0, 0, 9, 5, 0, 8,
    ///                                            0, 3, 6, 0, 0, 0, 7, 9, 0,
    ///                                            8, 0, 9, 7, 0, 0, 0, 0, 2,
    ///                                            0, 0, 4, 0, 9, 0, 0, 0, 5,
    ///                                            0, 0, 1, 2, 5, 6, 0, 0, 9,
    ///                                            0, 0, 0, 0, 0, 0, 0, 1, 0]);
    ///
    /// assert!(valid_sudoku.is_valid());
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(valid_sudoku.is_solvable());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.fulfills_horizontal_condition() &&
        self.fulfills_vertical_condition() &&
        self.fulfills_in_3x3_cell_condition()
    }

    /// True if this [Sudoku] has any empty squares.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([0, 3, 0, 0, 0, 0, 0, 7, 0,
    ///                                      0, 0, 7, 9, 0, 0, 0, 4, 2,
    ///                                      4, 0, 0, 7, 0, 0, 8, 5, 0,
    ///                                      0, 0, 0, 8, 7, 5, 0, 6, 0,
    ///                                      0, 0, 0, 1, 0, 2, 0, 0, 0,
    ///                                      0, 8, 0, 4, 6, 3, 0, 0, 0,
    ///                                      0, 1, 2, 0, 0, 7, 0, 0, 4,
    ///                                      9, 6, 0, 0, 0, 1, 7, 0, 0,
    ///                                      0, 4, 0, 0, 0, 0, 0, 3, 0]);
    ///
    /// assert!(sudoku.has_empty_squares());
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(sudoku.is_solvable());
    /// ```
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([8, 3, 9, 2, 5, 4, 1, 7, 6,
    ///                                      6, 5, 7, 9, 1, 8, 3, 4, 2,
    ///                                      4, 2, 1, 7, 3, 6, 8, 5, 9,
    ///                                      1, 9, 4, 8, 7, 5, 2, 6, 3,
    ///                                      3, 7, 6, 1, 9, 2, 4, 8, 5,
    ///                                      2, 8, 5, 4, 6, 3, 9, 1, 7,
    ///                                      5, 1, 2, 3, 8, 7, 6, 9, 4,
    ///                                      9, 6, 3, 5, 4, 1, 7, 2, 8,
    ///                                      7, 4, 8, 6, 2, 9, 5, 3, 1]);
    ///
    /// assert_eq!(sudoku.has_empty_squares(), false);
    ///
    /// # // I don't want to use examples that are invalid states.
    /// # assert!(sudoku.is_solved());
    /// ```
    pub fn has_empty_squares(&self) -> bool {
        for i in 0..NUM_SQUARES {
            if self.grid[i] == 0 {
                return true;
            }
        }

        false
    }

    /// Get the number of empty squares on this [Sudoku] grid.
    ///
    /// This does the same as calling [Sudoku::num_occurrences_of()] with an argument of
    /// `0`.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([4, 3, 0, 0, 0, 9, 8, 0, 0,
    ///                                      1, 9, 0, 8, 0, 0, 0, 0, 5,
    ///                                      0, 0, 0, 7, 2, 4, 0, 0, 0,
    ///                                      0, 6, 1, 0, 9, 0, 0, 2, 0,
    ///                                      0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                      0, 2, 0, 0, 5, 0, 6, 9, 0,
    ///                                      0, 0, 0, 6, 3, 7, 0, 0, 0,
    ///                                      8, 0, 0, 0, 0, 5, 0, 3, 1,
    ///                                      0, 0, 5, 1, 0, 0, 0, 7, 6]);
    ///
    /// assert_eq!(sudoku.num_empty_squares(), 51);
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(sudoku.is_solvable());
    /// ```
    pub fn num_empty_squares(&self) -> usize {
        self.num_occurrences_of(0)
    }

    /// Get the number of squares on this [Sudoku] grid that contain a certain
    /// value.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([4, 3, 0, 0, 0, 9, 8, 0, 0,
    ///                                      1, 9, 0, 8, 0, 0, 0, 0, 5,
    ///                                      0, 0, 0, 7, 2, 4, 0, 0, 0,
    ///                                      0, 6, 1, 0, 9, 0, 0, 2, 0,
    ///                                      0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                                      0, 2, 0, 0, 5, 0, 6, 9, 0,
    ///                                      0, 0, 0, 6, 3, 7, 0, 0, 0,
    ///                                      8, 0, 0, 0, 0, 5, 0, 3, 1,
    ///                                      0, 0, 5, 1, 0, 0, 0, 7, 6]);
    ///
    /// assert_eq!(sudoku.num_occurrences_of(3), 3);
    /// assert_eq!(sudoku.num_occurrences_of(7), 3);
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(sudoku.is_solvable());
    /// ```
    pub fn num_occurrences_of(&self, value: u32) -> usize {
        validate_value(value);

        self.grid.iter().filter(|&item| *item == value).count()
    }

    // There is some repeated code in the following three functions
    // (`fulfills_horizontal_condition()`, `fulfills_vertical_condition()`,
    // `fulfills_in_3x3_cell_condition`). It would be possible to extract that
    // code into a new struct. However, I am too lazy to do this...

    /// True if this [Sudoku] grid has no duplicate values within any
    /// horizontal line.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let sudoku = Sudoku::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ///                                      1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// assert!(sudoku.fulfills_horizontal_condition());
    /// ```
    pub fn fulfills_horizontal_condition(&self) -> bool {
        for y in 0..9 {
            let mut bit_flags = 0;

            for x in 0..9 {
                let value = self.grid[x + y * 9];
                // Check if the value has already been encountered, if yes, the
                // condition is not fulfilled. Else set a flag in `bit_flags`
                // that the value has been encountered.
                //
                // Zero's can just be ignored, empty squares don't matter for
                // this calculation.
                if value == 0 {
                    continue;
                } else if (bit_flags >> (value - 1)) & 1 == 1 {
                    return false;
                } else {
                    bit_flags |= 1 << (value - 1);
                }
            }
        }

        true
    }

    /// True if this [Sudoku] grid has no duplicate values within any vertical
    /// line.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// let sudoku = Sudoku::new_from_array([1, 1, 1, 1, 1, 1, 1, 1, 1,
    ///                                      2, 2, 2, 2, 2, 2, 2, 2, 2,
    ///                                      3, 3, 3, 3, 3, 3, 3, 3, 3,
    ///                                      4, 4, 4, 4, 4, 4, 4, 4, 4,
    ///                                      5, 5, 5, 5, 5, 5, 5, 5, 5,
    ///                                      6, 6, 6, 6, 6, 6, 6, 6, 6,
    ///                                      7, 7, 7, 7, 7, 7, 7, 7, 7,
    ///                                      8, 8, 8, 8, 8, 8, 8, 8, 8,
    ///                                      9, 9, 9, 9, 9, 9, 9, 9, 9]);
    ///
    /// assert!(sudoku.fulfills_vertical_condition());
    /// ```
    pub fn fulfills_vertical_condition(&self) -> bool {
         for x in 0..9 {
            let mut bit_flags = 0;

            for y in 0..9 {
                let value = self.grid[x + y * 9];
                // Check if the value has already been encountered, if yes, the
                // condition is not fulfilled. Else set a flag in `bit_flags`
                // that the value has been encountered.
                //
                // Zero's can just be ignored, empty squares don't matter for
                // this calculation.
                if value == 0 {
                    continue;
                } else if (bit_flags >> (value - 1)) & 1 == 1 {
                    return false;
                } else {
                    bit_flags |= 1 << (value - 1);
                }
            }
        }

        true
    }

    /// True if this [Sudoku] grid has no duplicate values within any of the 9
    /// 3x3 cells.
    ///
    /// ```
    /// use sudoku:: Sudoku;
    ///
    /// let sudoku = Sudoku::new_from_array([1, 2, 3, 1, 2, 3, 1, 2, 3,
    ///                                      4, 5, 6, 4, 5, 6, 4, 5, 6,
    ///                                      7, 8, 9, 7, 8, 9, 7, 8, 9,
    ///                                      1, 2, 3, 1, 2, 3, 1, 2, 3,
    ///                                      4, 5, 6, 4, 5, 6, 4, 5, 6,
    ///                                      7, 8, 9, 7, 8, 9, 7, 8, 9,
    ///                                      1, 2, 3, 1, 2, 3, 1, 2, 3,
    ///                                      4, 5, 6, 4, 5, 6, 4, 5, 6,
    ///                                      7, 8, 9, 7, 8, 9, 7, 8, 9]);
    ///
    /// assert!(sudoku.fulfills_in_3x3_cell_condition());
    /// ```
    pub fn fulfills_in_3x3_cell_condition(&self) -> bool {
        for x_cell in 0..3 {
            for y_cell in 0..3 {
                let min_x = x_cell * 3;
                let max_x = min_x + 2; // inclusive

                let min_y = y_cell * 3;
                let max_y = min_y + 2; // inclusive

                let mut bit_flags = 0;

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        let value = self.grid[x + y * 9];
                        // Check if the value has already been encountered, if yes, the
                        // condition is not fulfilled. Else set a flag in `bit_flags`
                        // that the value has been encountered.
                        //
                        // Zero's can just be ignored, empty squares don't matter for
                        // this calculation.
                        if value == 0 {
                            continue;
                        } else if (bit_flags >> (value - 1)) & 1 == 1 {
                            return false;
                        } else {
                            bit_flags |= 1 << (value - 1);
                        }
                    }
                }
            }
        }

        true
    }

    /// Get a [String] representation of this [Sudoku] grid.
    ///
    /// Useful for debugging purposes.
    ///
    /// ```
    /// use sudoku::Sudoku;
    ///
    /// // Values generated with http://www.opensky.ca/sudoku
    /// let sudoku = Sudoku::new_from_array([5, 0, 0, 0, 9, 0, 3, 8, 0,
    ///                                      0, 0, 0, 6, 0, 0, 0, 0, 4,
    ///                                      9, 0, 3, 8, 0, 1, 0, 7, 0,
    ///                                      2, 8, 0, 0, 5, 0, 0, 0, 0,
    ///                                      0, 4, 0, 0, 0, 0, 0, 6, 0,
    ///                                      0, 0, 0, 0, 7, 0, 0, 4, 8,
    ///                                      0, 5, 0, 7, 0, 4, 8, 0, 9,
    ///                                      7, 0, 0, 0, 0, 9, 0, 0, 0,
    ///                                      0, 1, 9, 0, 8, 0, 0, 0, 6]);
    ///
    /// // sudoku.string_repr() will produce the following string:
    /// let string_repr = "\
    /// 5 0 0 0 9 0 3 8 0
    /// 0 0 0 6 0 0 0 0 4
    /// 9 0 3 8 0 1 0 7 0
    /// 2 8 0 0 5 0 0 0 0
    /// 0 4 0 0 0 0 0 6 0
    /// 0 0 0 0 7 0 0 4 8
    /// 0 5 0 7 0 4 8 0 9
    /// 7 0 0 0 0 9 0 0 0
    /// 0 1 9 0 8 0 0 0 6
    /// ";
    ///
    /// assert_eq!(sudoku.string_repr(), string_repr);
    /// #
    /// # // I want all examples to be solvable
    /// # assert!(sudoku.is_solvable());
    /// ```
    ///
    /// This is not used for implementing the [Debug] or [std::fmt::Display]
    /// traits, because, spanning 9 lines, the output is quite bulky, which is
    /// not practical in every case.
    // I think this function could be optimized some more (a lot of `String`s
    // are created), but I don't feel like figuring out how to optimize it
    // further.
    #[allow(unstable_name_collisions)]
    pub fn string_repr(&self) -> String {
        let mut string_repr = self.grid
            .iter()
            .chunks(9)
            .into_iter()
            .map(|row| row
                 .map(|value| value.to_string())
                 .intersperse(" ".to_owned())
                 .collect::<String>())
            .intersperse("\n".to_owned())
            .collect::<String>();

        string_repr.push_str("\n");

        string_repr
    }
}

/// Remember all values that may still be possible for a specific square.
///
/// See also [NotesGrid].
// TODO the derived Debug trait implementation is very ugly and useless because
// notes_flags is formatted to decimal
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SudokuNote {
    notes_flags: u32,
    num_values_possible: u32,
}

impl SudokuNote {

    /// The state of the `notes_flags` of [SudokuNote] attribute where all values
    /// are still possible.
    const ALL_VALUES_POSSIBLE: u32 = 0b111_111_111;

    /// Initialize a new SudokuNote. It will assume that all values are still
    /// possible in the square it represents.
    fn new_with_all_values_possible() -> SudokuNote {
        SudokuNote {
            notes_flags: SudokuNote::ALL_VALUES_POSSIBLE,
            num_values_possible: 9,
        }
    }

    /// Check if a certain value can still possibly be placed in the square
    /// corresponding to this [SudokuNote].
    ///
    /// Do not use values for `value` > 9. In that case, the behaviour of this
    /// function is not defined and may produce all sorts of weird results.
    fn is_value_possible(&self, value: u32) -> bool {
        (self.notes_flags >> (value - 1)) & 1 != 0
    }

    /// Get how many values can still possibly be placed in the square
    /// corresponding to this [SudokuNote].
    fn num_values_possible(&self) -> u32 {
        self.num_values_possible
    }

    /// Get an [Iterator] of all the values that can still possibly be placed
    /// in the square corresponding to this [SudokuNote].
    ///
    /// The iterator returns the values in ascending order.
    fn possible_values(&self) -> SudokuNoteIter {
        SudokuNoteIter::new(&self)
    }

    /// Reset this note to a state where every value could possibly be placed
    /// in the corresponding sudoku square.
    fn reset_to_all_values_possible(&mut self) {
        self.notes_flags = SudokuNote::ALL_VALUES_POSSIBLE;
        self.num_values_possible = 9;
    }
}

/// The [Iterator] returned by [SudokuNote::possible_values()].
struct SudokuNoteIter<'a> {
    position: u32,
    note: &'a SudokuNote,
}

impl SudokuNoteIter<'_> {
    fn new(note: &SudokuNote) -> SudokuNoteIter {
        SudokuNoteIter {
            position: 0,
            note: note,
        }
    }
}

impl Iterator for SudokuNoteIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        
        // A plaintext explanation of what this implementation does:
        //
        // `position` is the "pointer" of the iterator. It points to some
        // possible value for the SudokuNote. On the next iteration it is moved
        // until a value is found that is possible or until the highest
        // possible value (`9`) is reached.

        self.position += 1;
        while !self.note.is_value_possible(self.position) && self.position <= 9 {
            self.position += 1;
        }

        if self.position > 9 {
            return None;
        }

        Some(self.position)
    }
}

/// A collection of [SudokuNote]s that resembles the grid of a [Sudoku].
///
/// This makes it very simple to associate a [Sudoku] square with a
/// corresponding [SudokuNote] as both can be uniquely identified by a pair of
/// x and y coordinates.
///
/// See [Sudoku] for a more in-depth explanation of the coordinate system.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NotesGrid {
    grid: [SudokuNote; NUM_SQUARES],
}

impl NotesGrid {
    
    /// Initialize a new [NotesGrid].
    ///
    /// Set all [SudokuNote]s to a state where all values are still possible.
    fn new() -> NotesGrid {
        NotesGrid {
            grid: [SudokuNote::new_with_all_values_possible(); NUM_SQUARES],
        }
    }

    /// Borrow the [SudokuNote] for the square at position (`x` / `y`).
    ///
    /// Do not use invalid coordinates. Doing so will yield undesirable
    /// results.
    fn get_note(&self, x: usize, y: usize) -> &SudokuNote {
        &self.grid[x + y * 9]
    }

    /// Get a mutable borrow of the [SudokuNote] for the square at position
    /// (`x` / `y`).
    ///
    /// Do not use invalid coordinates. Doing so will yield undesirable
    /// results.
    fn get_note_mut(&mut self, x: usize, y: usize) -> &mut SudokuNote {
        &mut self.grid[x + y * 9]
    }

    /// Reset the [NotesGrid] to the state generated by [NotesGrid::new()].
    fn reset(&mut self) {
        self.grid.iter_mut().for_each(|note| note.reset_to_all_values_possible());
    }
}

/// Check every square in the given [Sudoku] grid and remove all impossible
/// values from the given [NotesGrid].
///
/// Or a bit more precise:
/// Check every empty square in the [Sudoku] grid and note in its corresponding
/// [SudokuNote] in the given [NotesGrid] that all values in the vertical line,
/// the horizontal line and the surrounding 3x3 cell of the square can not
/// possibly be placed in that square.
///
/// What happens with the notes for squares that already contain a value is not
/// defined and may change in future versions.
fn make_all_notes(notes: &mut NotesGrid, sudoku: &Sudoku) {
    make_vertical_notes(notes, &sudoku);
    make_horizontal_notes(notes, &sudoku);
    make_in_cell_notes(notes, &sudoku);

    for note in &mut notes.grid {
        note.num_values_possible = 0;
        for i in 0..9 {
            note.num_values_possible += (note.notes_flags >> i) & 1
        }
    }
}

/// Make vertical notes for every square in a [Sudoku].
///
/// This functions leaves all [SudokuNote]s in the [NotesGrid] in an invalid
/// state because the field `num_values_possible` is not updated.
fn make_vertical_notes(notes: &mut NotesGrid, sudoku: &Sudoku) {
    for x in 0..9 {
        let mut notes_mask = 0b111_111_111;
        for y in 0..9 {
            let value = sudoku.get_value(x, y);
            if value == 0 {
                continue;
            }
            notes_mask ^= 1 << (value - 1);
        }
        for y in 0..9 {
            notes.get_note_mut(x, y).notes_flags &= notes_mask;
        }
    }
}

/// Make horizontal notes for every square in a [Sudoku].
///
/// This functions leaves all [SudokuNote]s in the [NotesGrid] in an invalid
/// state because the field `num_values_possible` is not updated.
fn make_horizontal_notes(notes: &mut NotesGrid, sudoku: &Sudoku) {
    for y in 0..9 {
        let mut notes_mask = 0b111_111_111;
        for x in 0..9 {
            let value = sudoku.get_value(x, y);
            if value == 0 {
                continue;
            }
            notes_mask ^= 1 << (value - 1);
        }
        for x in 0..9 {
            notes.get_note_mut(x, y).notes_flags &= notes_mask;
        }
    }
}

/// Make notes in the 3x3 cell for every square in a [Sudoku].
///
/// This functions leaves all [SudokuNote]s in the [NotesGrid] in an invalid
/// state because the field `num_values_possible` is not updated.
fn make_in_cell_notes(notes: &mut NotesGrid, sudoku: &Sudoku) {
    for cell_y in 0..3 {
        for cell_x in 0..3 {
            let mut notes_mask = 0b111_111_111;
            for square_y in 0..3 {
                for square_x in 0..3 {
                    let x = cell_x * 3 + square_x;
                    let y = cell_y * 3 + square_y;
                    let value = sudoku.get_value(x, y);
                    if value == 0 {
                        continue;
                    }
                    notes_mask ^= 1 << (value - 1);
                }
            }
            for square_y in 0..3 {
                for square_x in 0..3 {
                    let x = cell_x * 3 + square_x;
                    let y = cell_y * 3 + square_y;
                    notes.get_note_mut(x, y).notes_flags &= notes_mask;
                }
            }
        }
    }
}

/// Replace all the empty squares in the [Sudoku] where only a single value is
/// possible based on the provided [NotesGrid] with that value.
///
/// Return the number of values newly written to the [Sudoku].
fn replace_notes_with_values(sudoku: &mut Sudoku, notes: &NotesGrid) -> u32 {
    let mut num_new_values = 0;

    for x in 0..9 {
        for y in 0..9 {
            let current_note = notes.get_note(x, y);
            // The second part of this expression is required
            // because the notes of squares that already contain a
            // value may still allow some possible values. See the
            // documentation for sudoku::make_all_notes() for more
            // information.
            if current_note.num_values_possible() == 1 && sudoku.get_value(x, y) == 0 {
                let certain_value = current_note
                    .possible_values()
                    .next()
                    .expect("There is always exactly 1 value in this iterator");
                sudoku.set_value(x, y, certain_value);

                num_new_values += 1;
            }
        }
    }

    num_new_values
}

/// Fill in all squares of a [Sudoku] that can be using a [NotesGrid].
fn advance_with_notes(sudoku_grid: &mut Sudoku, notes: &mut NotesGrid) {
    // use a value that cannot be reached otherwise, this makes for easier
    // debugging
    let mut num_changes = u32::MAX;
        
    while num_changes != 0 {
        make_all_notes(notes, &sudoku_grid);
        num_changes = replace_notes_with_values(sudoku_grid, &notes);
    }
}

/// Check if a [Sudoku] is a dead end based on an existing [NotesGrid].
///
/// A [Sudoku] is considered a dead end if there exists at least 1 square
/// on the grid that will result in an invalid [Sudoku] if any value is
/// inserted.
fn is_dead_end(sudoku_grid: &Sudoku, notes: &NotesGrid) -> bool {
    for x in 0..9 {
        for y in 0..9 {
            if notes.get_note(x, y).num_values_possible() == 0 && sudoku_grid.get_value(x, y) == 0 {
                return true;
            }
        }
    }

    false
}

/// The [Iterator] returned by [Sudoku::find_all_solutions()] and the type that
/// does the actual solving of [Sudoku]s.
///
/// `sudoku_grid` is a reference to the [Sudoku] puzzle to be solved by the solver.
///
/// `changes_stack` is a record of what changes needed to be made to the
/// [Sudoku] to find the previous solution. This is required for the solver to
/// know where to continue the search.
///
/// // TODO
/// The solver would probably be faster if not changes, but the states of the
/// grid was stored in the stack. The way it is now, the solver requires very
/// little memory, but does quite some extra calculations because of that.
struct AllSolutionsIterator<'a> {
    sudoku_grid: &'a Sudoku,
    changes_stack: Vec<ValueChange>,
}

impl AllSolutionsIterator<'_> {

    /// Initialize a new [AllSolutionsIterator].
    ///
    /// Takes care of initializing `changes_stack`.
    fn new(sudoku_grid: &Sudoku) -> AllSolutionsIterator {

        // The maximum capacity needed for `changes_stack`.
        //
        // All positions with 3 or less empty squares left should be solvable
        // without the stack (I think, I have no proof of this), which allows
        // us to set the capacity to the number of empty squares - 3.
        //
        // The optimal value is probably quite a bit lower than this, but I
        // don't feel like doing all the maths to figure it out right now and
        // it wouldn't significantly improve the performance of the solver
        // anyways.
        let num_empty_squares = sudoku_grid.num_empty_squares();
        let stack_capacity = if num_empty_squares > 3 {
            num_empty_squares - 3
        } else {
            0
        };

        AllSolutionsIterator {
            sudoku_grid,
            changes_stack: Vec::with_capacity(stack_capacity),
        }
    }

    /// Revert the last change made by the solver.
    ///
    /// Pop the last change off `changes_stack`, revert `sudoku_grid` and
    /// `notes` to the state before the last change and set `last_value` to the
    /// value of the last change.
    ///
    /// Return an Error if `changes_stack` is empty.
    fn revert_last_change(&mut self, sudoku_grid: &mut Sudoku, notes: &mut NotesGrid, last_value: &mut u32) -> Result<(), &'static str> {
        let last_value_change = match self.changes_stack.pop() {
            Some(value_change) => value_change,
            None => return Err("stack empty"),
        };
        *last_value = last_value_change.value;
        *sudoku_grid = *self.sudoku_grid;
        for value_change in &self.changes_stack {
            sudoku_grid.set_value(value_change.x, value_change.y, value_change.value);
        }
        notes.reset();

        Ok(())
    }
}

impl Iterator for AllSolutionsIterator<'_> {
    type Item = Sudoku;

    fn next(&mut self) -> Option<Sudoku> {

        let mut sudoku_grid = *self.sudoku_grid;
        let mut notes = NotesGrid::new();

        // `last_value` ensures that the solver will not just find the same
        // solution over and over again
        //
        // if this is the search for the first solution, set `last_value` to 0,
        // else remove the last value on the stack (otherwise the exact same
        // solution that was already found will be returned) and set last_value
        // to that value
        let mut last_value = match self.changes_stack.pop() {
            Some(value_change) => value_change.value,
            None => 0,
        };

        for value_change in &self.changes_stack {
            sudoku_grid.set_value(value_change.x, value_change.y, value_change.value);
        }

        'outer: loop {

            advance_with_notes(&mut sudoku_grid, &mut notes);
            
            // advance_with_notes() does not guarantee that the grid it
            // produces is valid, so it has to be checked here
            if (!sudoku_grid.is_valid()) || is_dead_end(&sudoku_grid, &notes) {
                match self.revert_last_change(&mut sudoku_grid, &mut notes, &mut last_value) {
                    Ok(_) => continue 'outer,
                    // if the stack is empty
                    Err(_) => return None,
                };
            }

            // if a Sudoku grid is valid and has no empty squares, that means
            // it is solved
            if sudoku_grid.num_empty_squares() == 0 {
                return Some(sudoku_grid);
            }

            for y in 0..9 {
                for x in 0..9 {
                    for possible_value in notes.get_note(x, y).possible_values() {
                        // The second part of this expression is required
                        // because the notes of squares that already contain a
                        // value may still allow some possible values. See the
                        // documentation for sudoku::make_all_notes() for more
                        // information.
                        if possible_value > last_value && sudoku_grid.get_value(x, y) == 0 {
                            last_value = 0;
                            sudoku_grid.set_value(x, y, possible_value);
                            self.changes_stack.push(ValueChange { x, y, value: possible_value });
                            continue 'outer;
                        }
                    }
                }
            }

            match self.revert_last_change(&mut sudoku_grid, &mut notes, &mut last_value) {
                Ok(_) => continue 'outer,
                // if the stack is empty
                Err(_) => return None,
            };
        }
    }
}

/// Stores one change of the solver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ValueChange {
    x: usize,
    y: usize,
    value: u32,
}

#[cfg(test)]
mod tests {

    use crate::Sudoku;
    use crate::SudokuNote;
    use crate::NotesGrid;

    use crate::NUM_SQUARES;

    /// A very simple Sudoku puzzle.
    ///
    /// Generated with https://sudokukingdom.com/very-easy-sudoku.php (accessed 15.08.2022)
    const EXTREMELY_SIMPLE_SUDOKU: [u32; NUM_SQUARES] = [7, 0, 6, 0, 5, 0, 4, 1, 9,
                                                         0, 0, 9, 0, 7, 4, 8, 0, 0,
                                                         4, 8, 0, 6, 0, 9, 0, 0, 5,
                                                         0, 0, 8, 9, 0, 5, 0, 3, 4,
                                                         9, 4, 2, 0, 6, 0, 1, 0, 0,
                                                         3, 0, 0, 4, 0, 1, 0, 9, 2,
                                                         0, 9, 0, 0, 0, 6, 5, 8, 1,
                                                         5, 3, 0, 1, 4, 0, 0, 2, 0,
                                                         0, 6, 1, 5, 9, 0, 3, 0, 0];

    /// The solved state of [EXTREMELY_SIMPLE_SUDOKU].
    const EXTREMELY_SIMPLE_SUDOKU_SOLUTION: [u32; NUM_SQUARES] = [7, 2, 6, 8, 5, 3, 4, 1, 9,
                                                                  1, 5, 9, 2, 7, 4, 8, 6, 3,
                                                                  4, 8, 3, 6, 1, 9, 2, 7, 5,
                                                                  6, 1, 8, 9, 2, 5, 7, 3, 4,
                                                                  9, 4, 2, 3, 6, 7, 1, 5, 8,
                                                                  3, 7, 5, 4, 8, 1, 6, 9, 2,
                                                                  2, 9, 4, 7, 3, 6, 5, 8, 1,
                                                                  5, 3, 7, 1, 4, 8, 9, 2, 6,
                                                                  8, 6, 1, 5, 9, 2, 3, 4, 7];


    // Sudoku methods

    #[test]
    fn new_empty_is_empty() {
        let empty_grid = Sudoku::new_empty();
        
        for x in 0..9 {
            for y in 0..9 {
                assert_eq!(empty_grid.get_value(x, y), 0);
            }
        }
    }

    #[test]
    fn get_value_set_value() {
        let mut grid = Sudoku::new_empty();

        grid.set_value(3, 4, 3);

        let result = grid.get_value(3, 4);
        let expected = 3;

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn get_value_panics_on_invalid_x() {
        let grid = Sudoku::new_empty();
        grid.get_value(11, 3);
    }

    #[test]
    #[should_panic]
    fn get_value_panics_on_invalid_y() {
        let grid = Sudoku::new_empty();
        grid.get_value(1, 9);
    }

    #[test]
    #[should_panic]
    fn set_value_panics_on_invalid_x() {
        let mut grid = Sudoku::new_empty();
        grid.set_value(10, 7, 0);
    }

    #[test]
    #[should_panic]
    fn set_value_panics_on_invalid_y() {
        let mut grid = Sudoku::new_empty();
        grid.set_value(5, 9, 1);
    }

    #[test]
    #[should_panic]
    fn set_value_panics_on_invalid_value() {
        let mut grid = Sudoku::new_empty();
        grid.set_value(7, 0, 10);
    }

    #[test]
    fn new_from_array_sample_test() {
        let grid = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);

        assert_eq!(grid.get_value(0, 0), 7);
        assert_eq!(grid.get_value(2, 3), 8);
        assert_eq!(grid.get_value(1, 0), 0);
        assert_eq!(grid.get_value(0, 1), 0);
        assert_eq!(grid.get_value(5, 7), 0);
        assert_eq!(grid.get_value(8, 3), 4);
    }

    #[test]
    #[should_panic]
    fn new_from_array_panics_on_invalid_value() {
        Sudoku::new_from_array([0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0,10, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn is_solved_yes() {
        // Values generated with http://www.opensky.ca/sudoku
        let solved_sudoku = Sudoku::new_from_array([1, 9, 2, 7, 5, 3, 6, 8, 4,
                                                    7, 6, 5, 8, 9, 4, 1, 3, 2,
                                                    3, 8, 4, 1, 2, 6, 9, 5, 7,
                                                    2, 5, 8, 4, 7, 1, 3, 6, 9,
                                                    4, 1, 7, 6, 3, 9, 5, 2, 8,
                                                    9, 3, 6, 5, 8, 2, 4, 7, 1,
                                                    8, 4, 9, 2, 6, 5, 7, 1, 3,
                                                    6, 7, 1, 3, 4, 8, 2, 9, 5,
                                                    5, 2, 3, 9, 1, 7, 8, 4, 6]);

        assert!(solved_sudoku.is_solved());
    }

    #[test]
    fn is_solved_no_because_contains_0() {
        // Original values generated with http://www.opensky.ca/sudoku
        //
        // Note the `0` inserted at position (x = 1 / y = 2)
        let unsolved_sudoku = Sudoku::new_from_array([1, 9, 2, 7, 5, 3, 6, 8, 4,
                                                      7, 6, 5, 8, 9, 4, 1, 3, 2,
                                                      3, 0, 4, 1, 2, 6, 9, 5, 7,
                                                      2, 5, 8, 4, 7, 1, 3, 6, 9,
                                                      4, 1, 7, 6, 3, 9, 5, 2, 8,
                                                      9, 3, 6, 5, 8, 2, 4, 7, 1,
                                                      8, 4, 9, 2, 6, 5, 7, 1, 3,
                                                      6, 7, 1, 3, 4, 8, 2, 9, 5,
                                                      5, 2, 3, 9, 1, 7, 8, 4, 6]);

        assert!(unsolved_sudoku.has_empty_squares());

        assert!(unsolved_sudoku.fulfills_horizontal_condition());
        assert!(unsolved_sudoku.fulfills_vertical_condition());
        assert!(unsolved_sudoku.fulfills_in_3x3_cell_condition());

        assert!(!unsolved_sudoku.is_solved());
    }

    #[test]
    fn is_solved_no_contains_double_value() {
        // Original values generated with http://www.opensky.ca/sudoku
        //
        // Note the `3` inserted at position (x = 2 / y = 7)
        let unsolved_sudoku = Sudoku::new_from_array([1, 9, 2, 7, 5, 3, 6, 8, 4,
                                                      7, 6, 5, 8, 9, 4, 1, 3, 2,
                                                      3, 8, 4, 1, 2, 6, 9, 5, 7,
                                                      2, 5, 8, 4, 7, 1, 3, 6, 9,
                                                      4, 1, 7, 6, 3, 9, 5, 2, 8,
                                                      9, 3, 6, 5, 8, 2, 4, 7, 1,
                                                      8, 4, 9, 2, 6, 5, 7, 1, 3,
                                                      6, 7, 3, 3, 4, 8, 2, 9, 5,
                                                      5, 2, 3, 9, 1, 7, 8, 4, 6]);

        assert!(!unsolved_sudoku.has_empty_squares());

        assert!(!unsolved_sudoku.fulfills_horizontal_condition());
        assert!(!unsolved_sudoku.fulfills_vertical_condition());
        assert!(!unsolved_sudoku.fulfills_in_3x3_cell_condition());

        assert!(!unsolved_sudoku.is_solved());
    }

    #[test]
    fn is_solved_no_vertical() {
        let unsolved_sudoku = Sudoku:: new_from_array([1, 4, 7, 2, 5, 8, 3, 6, 9,
                                                       2, 5, 8, 3, 6, 9, 4, 7, 1,
                                                       3, 6, 9, 4, 7, 1, 5, 8, 2,
                                                       1, 4, 7, 2, 5, 8, 3, 6, 9,
                                                       2, 5, 8, 3, 6, 9, 4, 7, 1,
                                                       3, 6, 9, 4, 7, 1, 5, 8, 2,
                                                       1, 4, 7, 2, 5, 8, 3, 6, 9,
                                                       2, 5, 8, 3, 6, 9, 4, 7, 1,
                                                       3, 6, 9, 4, 7, 1, 5, 8, 2]);

        assert!(!unsolved_sudoku.has_empty_squares());

        assert!(unsolved_sudoku.fulfills_horizontal_condition());
        assert!(!unsolved_sudoku.fulfills_vertical_condition());
        assert!(unsolved_sudoku.fulfills_in_3x3_cell_condition());

        assert!(!unsolved_sudoku.is_solved());
    }

    #[test]
    fn is_solved_no_horizontal() {
        let unsolved_sudoku = Sudoku::new_from_array([1, 2, 3, 1, 2, 3, 1, 2, 3,
                                                      4, 5, 6, 4, 5, 6, 4, 5, 6,
                                                      7, 8, 9, 7, 8, 9, 7, 8, 9,
                                                      2, 3, 4, 2, 3, 4, 2, 3, 4,
                                                      5, 6, 7, 5, 6, 7, 5, 6, 7,
                                                      8, 9, 1, 8, 9, 1, 8, 9, 1,
                                                      3, 4, 5, 3, 4, 5, 3, 4, 5,
                                                      6, 7, 8, 6, 7, 8, 6, 7, 8,
                                                      9, 1, 2, 9, 1, 2, 9, 1, 2]);

        assert!(!unsolved_sudoku.has_empty_squares());

        assert!(!unsolved_sudoku.fulfills_horizontal_condition());
        assert!(unsolved_sudoku.fulfills_vertical_condition());
        assert!(unsolved_sudoku.fulfills_in_3x3_cell_condition());

        assert!(!unsolved_sudoku.is_solved());
    }

    #[test]
    fn is_solved_no_3x3_cell() {
        let unsolved_sudoku = Sudoku::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9,
                                                      9, 1, 2, 3, 4, 5, 6, 7, 8,
                                                      8, 9, 1, 2, 3, 4, 5, 6, 7,
                                                      7, 8, 9, 1, 2, 3, 4, 5, 6,
                                                      6, 7, 8, 9, 1, 2, 3, 4, 5,
                                                      5, 6, 7, 8, 9, 1, 2, 3, 4,
                                                      4, 5, 6, 7, 8, 9, 1, 2, 3,
                                                      3, 4, 5, 6, 7, 8, 9, 1, 2,
                                                      2, 3, 4, 5, 6, 7, 8, 9, 1]);

        assert!(!unsolved_sudoku.has_empty_squares());

        assert!(unsolved_sudoku.fulfills_horizontal_condition());
        assert!(unsolved_sudoku.fulfills_vertical_condition());
        assert!(!unsolved_sudoku.fulfills_in_3x3_cell_condition());

        assert!(!unsolved_sudoku.is_solved());
    }
    
    #[test]
    fn find_solution_extremely_simple_sudoku() {
        let expected_solution = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU_SOLUTION);

        let puzzle = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let found_solution = puzzle.find_solution();

        assert_eq!(found_solution, Some(expected_solution));
    }

    #[test]
    fn find_solution_extremely_difficult_sudoku() {
        let extremely_difficult_sudoku = Sudoku::new_from_array([9, 4, 0, 0, 5, 0, 8, 0, 0,
                                                                 0, 0, 0, 0, 3, 4, 0, 0, 9,
                                                                 0, 0, 0, 0, 0, 0, 0, 2, 5,
                                                                 1, 5, 9, 0, 7, 0, 0, 4, 0,
                                                                 0, 0, 0, 5, 0, 6, 0, 0, 0,
                                                                 0, 6, 0, 0, 4, 0, 5, 9, 7,
                                                                 6, 2, 0, 0, 0, 0, 0, 0, 0,
                                                                 3, 0, 0, 7, 2, 0, 0, 0, 0,
                                                                 0, 0, 4, 0, 8, 0, 0, 3, 2]);

        let expected_solution = Sudoku::new_from_array([9, 4, 6, 2, 5, 7, 8, 1, 3,
                                                        2, 8, 5, 1, 3, 4, 6, 7, 9,
                                                        7, 3, 1, 9, 6, 8, 4, 2, 5,
                                                        1, 5, 9, 8, 7, 2, 3, 4, 6,
                                                        4, 7, 3, 5, 9, 6, 2, 8, 1,
                                                        8, 6, 2, 3, 4, 1, 5, 9, 7,
                                                        6, 2, 7, 4, 1, 3, 9, 5, 8,
                                                        3, 9, 8, 7, 2, 5, 1, 6, 4,
                                                        5, 1, 4, 6, 8, 9, 7, 3, 2]);

        let found_solution = extremely_difficult_sudoku.find_solution();

        assert_eq!(found_solution.unwrap(), expected_solution);
    }

    #[test]
    fn find_solution_dead_end() {
        let dead_end = Sudoku::new_from_array([1, 2, 0, 4, 5, 6, 7, 8, 9,
                                               0, 0, 3, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let solution = dead_end.find_solution();

        assert_eq!(solution, None);
    }

    #[test]
    fn find_solution_invalid_sudoku() {
        let invalid_sudoku = Sudoku::new_from_array([0, 0, 0, 4, 4, 0, 0, 0, 9,
                                                     0, 8, 5, 0, 0, 3, 0, 0, 0,
                                                     4, 0, 9, 6, 0, 0, 0, 3, 7,
                                                     3, 0, 0, 5, 0, 4, 0, 0, 0,
                                                     5, 0, 4, 0, 1, 0, 6, 0, 3,
                                                     0, 0, 0, 3, 0, 7, 0, 0, 5,
                                                     8, 1, 0, 0, 0, 9, 7, 0, 6,
                                                     0, 0, 0, 2, 0, 0, 1, 9, 0,
                                                     9, 0, 0, 0, 5, 0, 0, 0, 0]);

        let solution = invalid_sudoku.find_solution();

        assert_eq!(solution, None);
    }

    /// This used to be a bug. The solver would try to solve invalid Sudoku
    /// puzzles too. `find_solution_invalid_sudoku()` was not enough to find
    /// that bug.
    #[test]
    fn find_solution_invalid_sudoku_2() {
        // Original values generated with http://www.opensky.ca/sudoku
        //
        // Note the two 9's at coordinates (0 / 0) and (1 / 0)
        let invalid_sudoku = Sudoku::new_from_array([9, 9, 2, 7, 5, 3, 6, 8, 4,
                                                     7, 6, 5, 8, 9, 4, 1, 3, 2,
                                                     3, 8, 4, 1, 2, 6, 9, 5, 7,
                                                     2, 5, 8, 4, 7, 1, 3, 6, 9,
                                                     4, 1, 7, 6, 3, 9, 5, 2, 8,
                                                     9, 3, 6, 5, 8, 2, 4, 7, 1,
                                                     8, 4, 9, 2, 6, 5, 7, 1, 3,
                                                     6, 7, 1, 3, 4, 8, 2, 9, 5,
                                                     5, 2, 3, 9, 1, 7, 8, 4, 6]);

        let solution = invalid_sudoku.find_solution();

        assert_eq!(solution, None);
    }

    #[test]
    fn find_all_solutions_exactly_2_solutions() {
        // taken from https://puzzling.stackexchange.com/questions/67789/examples-of-sudokus-with-two-solutions
        let two_possible_solutions_puzzle = Sudoku::new_from_array([2, 9, 5, 7, 4, 3, 8, 6, 1,
                                                                    4, 3, 1, 8, 6, 5, 9, 0, 0,
                                                                    8, 7, 6, 1, 9, 2, 5, 4, 3,
                                                                    3, 8, 7, 4, 5, 9, 2, 1, 6,
                                                                    6, 1, 2, 3, 8, 7, 4, 9, 5,
                                                                    5, 4, 9, 2, 1, 6, 7, 3, 8,
                                                                    7, 6, 3, 5, 2, 4, 1, 8, 9,
                                                                    9, 2, 8, 6, 7, 1, 3, 5, 4,
                                                                    1, 5, 4, 9, 3, 8, 6, 0, 0]);
        
        let solutions = two_possible_solutions_puzzle.find_all_solutions().collect::<Vec<_>>();
        
        assert_eq!(solutions.len(), 2);
        
        for solution in solutions {
            assert!(solution.is_solved());
        }
    }

    #[test]
    fn num_occurrences_of_and_num_empty_squares() {
        let sudoku = Sudoku::new_from_array([2, 0, 0, 8, 7, 0, 0, 0, 0,
                                             0, 0, 0, 1, 0, 0, 8, 0, 0,
                                             0, 8, 0, 6, 0, 4, 0, 2, 1,
                                             6, 0, 3, 0, 0, 0, 0, 0, 7,
                                             0, 4, 1, 0, 6, 0, 5, 9, 0,
                                             9, 0, 0, 0, 0, 0, 4, 0, 2,
                                             5, 7, 0, 4, 0, 6, 0, 1, 0,
                                             0, 0, 6, 0, 0, 3, 0, 0, 0,
                                             0, 0, 0, 0, 1, 8, 0, 0, 6]);

        assert_eq!(sudoku.num_occurrences_of(0), 50);
        assert_eq!(sudoku.num_occurrences_of(1),  5);
        assert_eq!(sudoku.num_occurrences_of(2),  3);
        assert_eq!(sudoku.num_occurrences_of(3),  2);
        assert_eq!(sudoku.num_occurrences_of(4),  4);
        assert_eq!(sudoku.num_occurrences_of(5),  2);
        assert_eq!(sudoku.num_occurrences_of(6),  6);
        assert_eq!(sudoku.num_occurrences_of(7),  3);
        assert_eq!(sudoku.num_occurrences_of(8),  4);
        assert_eq!(sudoku.num_occurrences_of(9),  2);

        assert_eq!(sudoku.num_empty_squares(), 50);
    }

    #[test]
    #[should_panic]
    fn num_occurrences_of_panics_on_invalid_value() {
        let sudoku = Sudoku::new_empty();

        sudoku.num_occurrences_of(10);
    }

    #[test]
    fn string_repr() {
        // Values generated with http://www.opensky.ca/sudoku
        let sudoku_grid = Sudoku::new_from_array([6, 0, 8, 4, 0, 0, 2, 0, 0,
                                                  5, 0, 0, 7, 0, 0, 0, 0, 0,
                                                  9, 0, 0, 0, 8, 0, 7, 1, 0,
                                                  0, 0, 4, 0, 0, 9, 0, 0, 0,
                                                  7, 9, 0, 8, 0, 5, 0, 2, 1,
                                                  0, 0, 0, 1, 0, 0, 4, 0, 0,
                                                  0, 5, 9, 0, 3, 0, 0, 0, 8,
                                                  0, 0, 0, 0, 0, 4, 0, 0, 2,
                                                  0, 0, 7, 0, 0, 8, 3, 0, 6]);

        let expected = "\
6 0 8 4 0 0 2 0 0
5 0 0 7 0 0 0 0 0
9 0 0 0 8 0 7 1 0
0 0 4 0 0 9 0 0 0
7 9 0 8 0 5 0 2 1
0 0 0 1 0 0 4 0 0
0 5 9 0 3 0 0 0 8
0 0 0 0 0 4 0 0 2
0 0 7 0 0 8 3 0 6
";

        let actual = sudoku_grid.string_repr();

        assert_eq!(actual, expected);
    }

    // SudokuNote methods

    #[test]
    fn new_with_all_values_possible_is_value_possible() {
        let notes = SudokuNote::new_with_all_values_possible();

        for value in 1..=9 {
            assert!(notes.is_value_possible(value));
        }
    }

    #[test]
    fn new_with_all_values_possible_num_values_possible() {
        let notes = SudokuNote::new_with_all_values_possible();

        assert_eq!(notes.num_values_possible(), 9);
    }

    #[test]
    fn reset_to_all_values_possible() {
        let mut note = SudokuNote::new_with_all_values_possible();

        note.notes_flags = 0b111_001_100;

        note.reset_to_all_values_possible();

        assert_eq!(note, SudokuNote::new_with_all_values_possible());
    }

    // NotesGrid methods

    #[test]
    fn new_all_values_possible() {
        let grid = NotesGrid::new();
        for x in 0..9 {
            for y in 0..9 {
                for value in 1..=9 {
                    assert!(grid.get_note(x, y).is_value_possible(value));
                }
            }
        }
    }

    #[test]
    fn reset() {
        let mut notes_grid = NotesGrid::new();

        // some random changes
        notes_grid.get_note_mut(1, 5).notes_flags = 0b001_010_100;
        notes_grid.get_note_mut(1, 7).notes_flags = 0b101_010_100;
        notes_grid.get_note_mut(3, 1).notes_flags = 0b100_010_100;
        notes_grid.get_note_mut(4, 5).notes_flags = 0b000_000_111;
        notes_grid.get_note_mut(6, 2).notes_flags = 0b001_111_111;
        notes_grid.get_note_mut(7, 3).notes_flags = 0b011_110_100;
        notes_grid.get_note_mut(7, 4).notes_flags = 0b001_010_100;
        notes_grid.get_note_mut(7, 8).notes_flags = 0b101_110_110;

        notes_grid.reset();

        assert_eq!(notes_grid, NotesGrid::new());
    }

    // crate-level functions

    /// After calling `make_all_notes()`, all the squares that are empty
    /// must allow for at least 1 possible value (if the provided Sudoku
    /// puzzle is solvable).
    #[test]
    fn make_all_notes_value_possible_if_square_not_filled() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();
        crate::make_all_notes(&mut notes, &sudoku);
        for x in 0..9 {
            for y in 0..9 {
                if sudoku.get_value(x, y) == 0 {
                    assert_ne!(notes.get_note(x, y).num_values_possible, 0);
                }
            }
        }
    }

    #[test]
    fn make_all_notes_sample_test() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();
        crate::make_all_notes(&mut notes, &sudoku);

        assert_eq!(notes.get_note(2, 2).possible_values().collect::<Vec<u32>>(), vec![3]);

        assert_eq!(notes.get_note(4, 5).possible_values().collect::<Vec<u32>>(), vec![8]);

        assert_eq!(notes.get_note(7, 8).possible_values().collect::<Vec<u32>>(), vec![4, 7]);
    }

    #[test]
    fn make_vertical_notes() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();

        crate::make_vertical_notes(&mut notes, &sudoku);

        let expected = vec![3, 4, 5, 7];

        for y in 0..9 {
            let actual: Vec<u32> = notes
                .get_note(2, y)
                .possible_values()
                .collect();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn make_horizontal_notes() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();

        crate::make_horizontal_notes(&mut notes, &sudoku);

        let expected = vec![2, 3, 4, 7];

        for x in 0..9 {
            let actual: Vec<u32> = notes
                .get_note(x, 6)
                .possible_values()
                .collect();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn make_in_cell_notes() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();

        crate::make_in_cell_notes(&mut notes, &sudoku);

        let expected = vec![2, 4, 7, 8];
        let actual: Vec<u32> = notes
            .get_note(2, 6)
            .possible_values()
            .collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn replace_notes_with_values() {
        let mut sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();
        crate::make_all_notes(&mut notes, &sudoku);
        crate::replace_notes_with_values(&mut sudoku, &notes);

        assert_eq!(sudoku.get_value(8, 8), 7);
        assert_eq!(sudoku.get_value(0, 6), 2);
    }

    /// This is an unlikely edge case, but I felt like writing it anyways. It
    /// protects against possible changes in
    /// [sudoku::replace_notes_with_values()] that would overwrite existing
    /// values.
    #[test]
    fn replace_notes_with_values_will_not_replace_existing_values() {
        let mut sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut notes = NotesGrid::new();

        // Set up "notes" so that the only possible value for square x=0/y=0 is
        // 1
        notes.get_note_mut(0, 0).notes_flags = 0b000_000_001;

        crate::replace_notes_with_values(&mut sudoku, &notes);

        // If the square at x=0/y=0 had been replaced with 1, then this
        // assertion will fail
        assert_eq!(sudoku.get_value(0, 0), 7);
    }

    #[test]
    fn is_dead_end() {
        let dead_end = Sudoku::new_from_array([0, 2, 3, 4, 5, 6, 7, 8, 9,
                                               1, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0,
                                               0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let mut notes = NotesGrid::new();

        crate::make_all_notes(&mut notes, &dead_end);
       
        assert!(crate::is_dead_end(&dead_end, &notes));
    }
}
