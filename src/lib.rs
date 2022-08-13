
#![allow(dead_code)]

const NUM_SQUARES: usize = 9 * 9;

pub struct Sudoku {
    grid: [Option<u32>; 81],
}

fn validate_coordinates(x: usize, y: usize) {
    if x > 8 || y > 8 {
        panic!("x and y must both be <= 8 (x = {}, y = {})", x, y);
    }
}

fn validate_value(value: Option<u32>) {
    if let Some(content) = value {
        if content < 1 || content > 9 {
            panic!("Content of value must be >= 1 and <= 9 (was {})", content);
        }
    }
}

impl Sudoku {

    /// Initialize a new empty Sudoku board.
    pub fn new_empty() -> Sudoku {
        Sudoku {
            grid: [None; NUM_SQUARES],
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> Option<u32> {
        validate_coordinates(x, y);

        self.grid[x + y * 9]
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: Option<u32>) {
        validate_coordinates(x, y);
        validate_value(value);

        self.grid[x + y * 9] = value;
    }
}

const ALL_VALUES_POSSIBLE: u32 = 0b111_111_111;
//const NO_VALUES_POSSIBLE: u16 = 0b000_000_000;

#[derive(Clone, Copy)]
struct SudokuNote {
    notes_flags: u32
}

impl SudokuNote {

    fn new_with_all_values_possible() -> SudokuNote {
        SudokuNote {
            notes_flags: ALL_VALUES_POSSIBLE
        }
    }

    fn is_value_possible(&self, value: u32) -> bool {
        (self.notes_flags >> (value - 1)) & 1 != 0
    }

    fn make_note(&mut self, value: u32, is_possible: bool) {
        // clear the bit flag
        self.notes_flags &= !(1 << (value - 1));
        // set the bit flag
        self.notes_flags |= (is_possible as u32) << (value - 1);
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

    fn make_note(&mut self, x: usize, y: usize, value: u32, is_possible: bool) {
        self.grid[x + y * 9].make_note(value, is_possible);
    }

    fn is_value_possible(&self, x: usize, y: usize, value: u32) -> bool {
        self.grid[x + y * 9].is_value_possible(value)
    }
}

fn solve_sudoku(grid: &mut Sudoku) {
    unimplemented!();
}

#[cfg(test)]
mod tests {

    use crate::Sudoku;
    use crate::SudokuNote;

    #[test]
    fn new_empty_is_empty() {
        let empty_grid = Sudoku::new_empty();
        
        for x in 0..9 {
            for y in 0..9 {
                assert_eq!(empty_grid.get_value(x, y), None);
            }
        }
    }

    #[test]
    fn get_value_set_value() {
        let mut grid = Sudoku::new_empty();

        grid.set_value(3, 4, Some(3));

        let result = grid.get_value(3, 4);
        let expected = Some(3);

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
        grid.set_value(10, 7, None);
    }

    #[test]
    #[should_panic]
    fn set_value_panics_on_invalid_y() {
        let mut grid = Sudoku::new_empty();
        grid.set_value(5, 9, Some(1));
    }

    #[test]
    #[should_panic]
    fn set_value_panics_on_invalid_value() {
        let mut grid = Sudoku::new_empty();
        grid.set_value(7, 0, Some(0));
    }

    #[test]
    fn new_with_all_values_possible() {
        let notes = SudokuNote::new_with_all_values_possible();

        for value in 1..=9 {
            assert!(notes.is_value_possible(value));
        }
    }

    #[test]
    fn make_note_and_is_value_possible_note() {
        let mut notes = SudokuNote::new_with_all_values_possible();

        notes.make_note(5, false);

        println!("{:b}", notes.notes_flags);

        assert!(!notes.is_value_possible(5))
    }

    #[test]
    fn new_notes_grid_and_is_value_possible() {
        let mut grid = NotesGrid::new();
        for x in 0..9 {
            for y in 0..9 {
                for value in 0..9 {
                    assert!(grid.is_value_possible(x, y, value), true);
                }
            }
        }
    }
/*
    #[test]
    fn make_note_and_is_value_possible_grid() {
        let mut grid = NotesGrid::new();
        for x in 0..9 {
            for y in 0..9 {
                
            }
        }
    }
*/
}
