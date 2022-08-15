
#![allow(dead_code)]

const NUM_SQUARES: usize = 9 * 9;

fn validate_coordinates(x: usize, y: usize) {
    if x > 8 || y > 8 {
        panic!("x and y must both be <= 8 (x = {}, y = {})", x, y);
    }
}

fn validate_value(value: u32) {
    if value > 9 {
        panic!("Value must be <= 9 (was {})", value );
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Sudoku {
    grid: [u32; 81],
}

impl Sudoku {

    /// Initialize a new empty Sudoku board.
    pub fn new_empty() -> Sudoku {
        Sudoku {
            grid: [0; NUM_SQUARES],
        }
    }

    pub fn new_from_array(array: [u32; NUM_SQUARES]) -> Sudoku {
        for value in array {
            validate_value(value);
        }

        Sudoku {
            grid: array,
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> u32 {
        validate_coordinates(x, y);

        self.grid[x + y * 9]
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: u32) {
        validate_coordinates(x, y);
        validate_value(value);

        self.grid[x + y * 9] = value;
    }
}

const ALL_VALUES_POSSIBLE: u32 = 0b111_111_111;
//const NO_VALUES_POSSIBLE: u16 = 0b000_000_000;

#[derive(Clone, Copy)]
struct SudokuNote {
    notes_flags: u32,
    num_values_possible: u32,
}

impl SudokuNote {

    fn new_with_all_values_possible() -> SudokuNote {
        SudokuNote {
            notes_flags: ALL_VALUES_POSSIBLE,
            num_values_possible: 9,
        }
    }

    fn is_value_possible(&self, value: u32) -> bool {
        (self.notes_flags >> (value - 1)) & 1 != 0
    }

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

    fn num_values_possible(&self) -> u32 {
        self.num_values_possible
    }

    fn possible_values(&self) -> SudokuNoteIter {
        SudokuNoteIter::new(&self)
    }
}

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
        self.position += 1;
        while !self.note.is_value_possible(self.position) && self.position <= 9{
            self.position += 1;
        }

        if self.position > 9 {
            return None;
        }

        Some(self.position)
    }
}

struct NotesGrid {
    grid: [SudokuNote; NUM_SQUARES],
}

impl NotesGrid {
    
    fn new() -> NotesGrid {
        NotesGrid {
            grid: [SudokuNote::new_with_all_values_possible(); NUM_SQUARES],
        }
    }

    fn get_note(&self, x: usize, y: usize) -> &SudokuNote {
        &self.grid[x + y * 9]
    }

    fn get_note_mut(&mut self, x: usize, y: usize) -> &mut SudokuNote {
        &mut self.grid[x + y * 9]
    }
}

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

fn make_vertical_notes(note: &mut SudokuNote, x: usize, sudoku: &Sudoku) {
    for y in 0..9 {
        let value_of_square = sudoku.get_value(x, y);
        if value_of_square != 0 {
            note.make_note(value_of_square, false);
        }
    }
}

fn make_horizontal_notes(note: &mut SudokuNote, y: usize, sudoku: &Sudoku) {
    for x in 0..9 {
        let value_of_square = sudoku.get_value(x, y);
        if value_of_square != 0 {
            note.make_note(value_of_square, false);
        }
    }
}

fn make_in_cell_notes(note: &mut SudokuNote, x: usize, y: usize, sudoku: &Sudoku) {
    // determine the upper left square of the 9x9 cell the x and y are located
    // in
    let min_x = x - (x % 3);
    let min_y = y - (y % 3);

    // and also determine the bottom right square
    let max_x = min_x + 2;
    let max_y = min_y + 2;

    // iterate all the squares in the 9x9 cell, starting at the upper left
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

fn replace_notes_with_values(sudoku: &mut Sudoku, notes: &NotesGrid) -> u32 {
    let mut num_notes_replaced = 0;

    for x in 0..9 {
        for y in 0..9 {
            let current_note = notes.get_note(x, y);
            if current_note.num_values_possible() == 1 && sudoku.get_value(x, y) == 0 {
                // Since there is only 1 possible value, we can take the first
                // iterator value
                let certain_value = current_note.possible_values().next().unwrap();
                sudoku.set_value(x, y, certain_value);

                num_notes_replaced += 1;
            }
        }
    }

    num_notes_replaced
}

pub fn try_solve_sudoku(sudoku_grid: &mut Sudoku) {
    let mut notes = NotesGrid::new();
    let mut num_changes = u32::MAX;
    
    while num_changes != 0 {
        make_all_notes(&mut notes, &sudoku_grid);
        num_changes = replace_notes_with_values(sudoku_grid, &notes);
    }
}

#[cfg(test)]
mod tests {

    use crate::Sudoku;
    use crate::SudokuNote;
    use crate::NotesGrid;

    use crate::NUM_SQUARES;

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
    fn new_from_array() {
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

    // SudokuNote methods

    #[test]
    fn new_with_all_values_possible_and_is_value_possible() {
        let notes = SudokuNote::new_with_all_values_possible();

        for value in 1..=9 {
            assert!(notes.is_value_possible(value));
        }
    }

    #[test]
    fn new_with_all_values_possible_and_num_values_possible() {
        let notes = SudokuNote::new_with_all_values_possible();

        assert_eq!(notes.num_values_possible(), 9);
    }

    #[test]
    fn make_note_and_is_value_possible_note() {
        let mut notes = SudokuNote::new_with_all_values_possible();

        notes.make_note(5, false);

        assert!(!notes.is_value_possible(5))
    }

    #[test]
    fn make_note_and_num_values_possible_changes() {
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

        let expected = vec![1, 3, 5, 6, 7, 8];
        let actual: Vec<u32> = notes.possible_values().collect();

        assert_eq!(expected, actual);
    }

    // NotesGrid methods

    #[test]
    fn new_has_all_values_possible() {
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

    #[test]
    fn make_all_notes_value_must_be_possible_if_square_not_filled() {
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
    fn solve_extremly_simple_sudoku() {
        let expected_solution = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU_SOLUTION);

        let mut found_solution = Sudoku::new_from_array(EXTREMELY_SIMPLE_SUDOKU);
        crate::try_solve_sudoku(&mut found_solution);

        assert_eq!(found_solution, expected_solution);
    }
}
