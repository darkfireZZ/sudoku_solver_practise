//TODO document crate

#![warn(missing_docs)]

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
    /// # //TODO check if this sudoku is solvable
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
    /// assert_eq!(sudoku0, sudoku1);
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
    /// # //TODO check if this sudoku is solvable
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

    /// Check if this [Sudoku] is valid.
    ///
    /// A [Sudoku] is considered valid if it contains no duplicate values
    /// within any row, column or any of the 9 3x3 cells.
    ///
    /// IMPORTANT: Valid does not imply solvable, a [Sudoku] may well be valid
    /// but unsolvable. To check for solvability see // TODO
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
    /// # // TODO check if this is solvable
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
    /// # // TODO check if this is solvable
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
    /// # // TODO check if this is solvable
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
}

/// Remember all values that may still be possible for a specific square.
///
/// This is used by the solver, for an explanation, see //TODO
///
/// See also [NotesGrid].
#[derive(Clone, Copy)]
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

    /// Set if a certain value can still possibly be placed in the square
    /// corresponding to this [SudokuNote].
    ///
    /// Do not use values for `value` > 9. In that case, the behaviour of this
    /// function is not defined and may produce all sorts of weird results.
    fn make_note(&mut self, value: u32, is_possible: bool) {
        let notes_flags_old = self.notes_flags;

        // clear the bit flag
        self.notes_flags &= !(1 << (value - 1));
        // set the bit flag
        self.notes_flags |= (is_possible as u32) << (value - 1);

        // adapt num_values_possible
        if notes_flags_old > self.notes_flags {
            self.num_values_possible -= 1;
        }
        else if notes_flags_old < self.notes_flags {
            self.num_values_possible += 1;
        }
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
    for x in 0..9 {
        for y in 0..9 {
            if sudoku.get_value(x, y) == 0 {
                let mut current_note = notes.get_note_mut(x, y);
                make_vertical_notes(&mut current_note, x, &sudoku);
                make_horizontal_notes(&mut current_note, y, &sudoku);
                make_in_cell_notes(&mut current_note, x, y, &sudoku);
            }
        }
    }
}

/// Make the notes for a square based on its vertical line.
///
/// See [make_all_notes()] for a more in-depth explanation.
fn make_vertical_notes(note: &mut SudokuNote, x: usize, sudoku: &Sudoku) {
    for y in 0..9 {
        let value_of_square = sudoku.get_value(x, y);
        if value_of_square != 0 {
            note.make_note(value_of_square, false);
        }
    }
}

/// Make the notes for a square based on its horizontal line.
///
/// See [make_all_notes()] for a more in-depth explanation.
fn make_horizontal_notes(note: &mut SudokuNote, y: usize, sudoku: &Sudoku) {
    for x in 0..9 {
        let value_of_square = sudoku.get_value(x, y);
        if value_of_square != 0 {
            note.make_note(value_of_square, false);
        }
    }
}

/// Make the notes for a square based on its surrounding 3x3 cell.
///
/// See [make_all_notes()] for a more in-depth explanation.
fn make_in_cell_notes(note: &mut SudokuNote, x: usize, y: usize, sudoku: &Sudoku) {
    // determine the upper left square of the 3x3 cell the x and y are located
    // in
    let min_x = x - (x % 3);
    let min_y = y - (y % 3);

    // and also determine the bottom right square
    let max_x = min_x + 2;
    let max_y = min_y + 2;

    // iterate all the squares in the 3x3 cell, starting at the upper left
    // square
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let value_of_square = sudoku.get_value(x, y);
            if value_of_square != 0 {
                note.make_note(value_of_square, false);
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

/// Find the solution for the given [Sudoku] grid.
///
/// If there exists exactly 1 possible solution, this function will return the
/// solution. Else, `None` will be returned.
///
/// If the given [Sudoku] is invalid (meaning it contains the same value twice
/// in the same row, column or 3x3 cell), None will be returned.
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
/// let solution = sudoku::find_solution(puzzle);
///
/// if let Some(solved_sudoku) = solution {
///     assert!(solved_sudoku.is_solved());
/// } else {
///     panic!("If the Sudoku puzzle above is solvable, this code is unreachable");
/// }
/// ```
pub fn find_solution(mut sudoku_grid: Sudoku) -> Option<Sudoku> {
    find_all_solutions(sudoku_grid).next()
}

//TODO document
pub fn find_all_solutions(sudoku_grid: Sudoku) -> impl Iterator<Item = Sudoku> {
    AllSolutionsIterator::new(sudoku_grid)
}

//TODO document
struct AllSolutionsIterator {
    solvable_sudoku_grid: Option<Sudoku>,
    changes_stack: Vec<ValueChange>,
}

impl AllSolutionsIterator {
    fn new(sudoku_grid: Sudoku) -> AllSolutionsIterator {

        let solvable_sudoku_grid = if sudoku_grid.is_valid() {
            // TODO finish impl
            //while num_changes != 0 {
                //make_all_notes(&mut notes
            //}
            unimplemented!();
            Some(sudoku_grid)
        } else {
            None
        };

        // TODO this might be 1 too much, have a look at this again after impl
        let num_empty_squares = sudoku_grid.num_empty_squares();

        AllSolutionsIterator {
            solvable_sudoku_grid,
            changes_stack: Vec::with_capacity(num_empty_squares),
        }
    }
}

impl Iterator for AllSolutionsIterator {
    type Item = Sudoku;

    fn next(&mut self) -> Option<Sudoku> {
        
        // TODO verify that sudoku_grid is borrowed and not copied here
        let mut sudoku_grid = match self.solvable_sudoku_grid {
            Some(sudoku_grid) => sudoku_grid,
            None => { return None; },
        };


        let mut notes = NotesGrid::new();
        // use a value that cannot be reached otherwise, this makes for easier
        // debugging
        let mut num_changes = u32::MAX;

        let mut last_value = 0;
        
        while num_changes != 0 {
            make_all_notes(&mut notes, &sudoku_grid);
            num_changes = replace_notes_with_values(&mut sudoku_grid, &notes);
        }
    
        let original_sudoku_grid = sudoku_grid;

        // TODO better label names

        'outer: loop {
            // The 'loop1 label is technically not required, but it is useful to have as a
            // reference in comments.
            'loop1: while num_changes != 0 {
                make_all_notes(&mut notes, &sudoku_grid);
                num_changes = replace_notes_with_values(&mut sudoku_grid, &notes);
            }

            'loop2: for x in 0..9 {
                for y in 0..9 {
                    // TODO not sure if the second part of this and expression is actually required
                    if notes.get_note(x, y).num_values_possible == 0 && sudoku_grid.get_value(x, y) != 0 {
                        // TODO improve expect message
                        let last_value_change = self.changes_stack.pop().expect("This code should be unreachable");
                        last_value = last_value_change.value;
                        sudoku_grid = original_sudoku_grid;
                        for value_change in self.changes_stack {
                            sudoku_grid.set_value(value_change.x, value_change.y, value_change.value);
                        }
                        notes.reset();
                        continue 'outer;
                    }
                }
            }

            if sudoku_grid.num_empty_squares() == 0 {
                return Some(sudoku_grid);
            }

            for x in 0..9 {
                for y in 0..9 {
                    if notes.get_note(x, y).num_values_possible() > last_value {
                        last_value = 0;
                        let value = notes
                            .get_note(x, y)
                            .possible_values()
                            .next()
                            // 'loop2 ensures that there cannot be any square that cannot
                            // possibly hold any value.
                            //
                            // 'loop1 ensures that there cannot be any square that could hold
                            // exactly 1 value. If there is any, the square is filled with that
                            // value.
                            .expect("There should always be at least 2 possible values");
                        self.changes_stack.push(ValueChange { x, y, value });
                        sudoku_grid.set_value(x, y, value);
                        continue 'outer;
                    }
                }
            }
        }

        None
    }
}

//TODO document
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
    fn make_note_is_value_possible_note() {
        let mut notes = SudokuNote::new_with_all_values_possible();

        notes.make_note(5, false);

        assert!(!notes.is_value_possible(5))
    }

    #[test]
    fn make_note_num_values_possible_changes() {
        let mut notes = SudokuNote::new_with_all_values_possible();

        notes.make_note(2, false);

        assert_eq!(notes.num_values_possible(), 8);

        notes.make_note(7, false);
        assert_eq!(notes.num_values_possible(), 7);

        notes.make_note(2, true);
        assert_eq!(notes.num_values_possible(), 8);

        notes.make_note(7, false);
        notes.make_note(1, true);
        assert_eq!(notes.num_values_possible(), 8);
    }

    #[test]
    fn possible_values() {
        let mut notes = SudokuNote::new_with_all_values_possible();
        notes.make_note(4, false);
        notes.make_note(9, false);
        notes.make_note(2, false);

        // the values as well as their order must be the same
        let expected = vec![1, 3, 5, 6, 7, 8];
        let actual: Vec<u32> = notes.possible_values().collect();

        assert_eq!(expected, actual);
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
        let mut note = SudokuNote::new_with_all_values_possible();

        crate::make_vertical_notes(&mut note, 2, &sudoku);

        let expected = vec![3, 4, 5, 7];
        let actual: Vec<u32> = note.possible_values().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn make_horizontal_notes() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut note = SudokuNote::new_with_all_values_possible();

        crate::make_horizontal_notes(&mut note, 6, &sudoku);

        let expected = vec![2, 3, 4, 7];
        let actual: Vec<u32> = note.possible_values().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn make_in_cell_notes() {
        let sudoku = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let mut note = SudokuNote::new_with_all_values_possible();

        crate::make_in_cell_notes(&mut note, 2, 6, &sudoku);

        let expected = vec![2, 4, 7, 8];
        let actual: Vec<u32> = note.possible_values().collect();

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
        for value in 2..=9 {
            notes.get_note_mut(0, 0).make_note(value, false);
        }

        crate::replace_notes_with_values(&mut sudoku, &notes);

        // If the square at x=0/y=0 had been replaced with 1, then this
        // assertion will fail
        assert_eq!(sudoku.get_value(0, 0), 7);
    }
    
    #[test]
    fn find_solution_extremely_simple_sudoku() {
        let expected_solution = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU_SOLUTION);

        let puzzle = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        let found_solution = crate::find_solution(puzzle);

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

        let found_solution = crate::find_solution(extremely_difficult_sudoku);

        assert_eq!(found_solution, Some(expected_solution));
    }

    #[test]
    fn find_solution_unsolvable_sudoku() {
        let unsolvable_sudoku = Sudoku::new_from_array([1, 2, 0, 4, 5, 6, 7, 8, 9,
                                                        0, 0, 3, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                        0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let solution = crate::find_solution(unsolvable_sudoku);

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

        let solution = crate::find_solution(invalid_sudoku);

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

        let solution = crate::find_solution(invalid_sudoku);

        assert_eq!(solution, None);
    }

    #[test]
    fn find_solution_multiple_solutions_sudoku() {
        let puzzle = Sudoku::new_from_array([1, 2, 3, 4, 5, 6, 7, 8, 9,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let solution = crate::find_solution(puzzle);

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
        
        let solutions: Vec<Sudoku> = crate::find_all_solutions(two_possible_solutions_puzzle).collect();
        
        assert_eq!(solutions.len(), 2);
        
        for solution in solutions {
            assert!(solution.is_solved());
        }
    }
}
