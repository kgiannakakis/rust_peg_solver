//! This crate provides code for solving the [English peg solitaire puzzle](http://en.wikipedia.org/wiki/Peg_solitaire)
//! and can produce image and gif solutions. The solver and input puzzle format is based on a
//! [Golang example](https://go.dev/test/solitaire.go). The code extends the existing sample so that it can work
//! with other variations of the puzzle, allowing the user to define a custom board.

use board::{validate_board, GameMove};
use itertools::iproduct;
use std::{error::Error, fs};

mod board;
pub mod solution;

/// A Solver struct for peg solitaire boards.
///
/// Every position of the board is represented by a character:
/// - ● represents a peg in the position
/// - ○ represents a hole
/// - ◎ denotes the center position, starting with a hole
/// - ◉ denotes the center position, starting with a peg
/// - . represents an unreachable position
///
/// The board must be surrounded by 2 dots (.) in each direction so that we don't need to check
/// the board boundaries while examining valid moves.
///
/// The classic English peg solitaire board is represented by the following board:
///
/// ``...........``  
/// ``...........``  
/// ``....●●●....``  
/// ``....●●●....``  
/// ``..●●●●●●●..``  
/// ``..●●●◎●●●..``  
/// ``..●●●●●●●..``  
/// ``....●●●....``  
/// ``....●●●....``  
/// ``...........``  
/// ``...........``  
///
/// Every row must end in a single new line character (\n). The last line must not have a new line
/// character at the end.
#[derive(Debug)]
pub struct Solver {
    /// Row length of the board
    row_length: usize,
    /// Current state of the board. The state of every position in the board is represented by a character.
    board: Vec<char>,
    /// The center of the board. Last peg must be in center position.
    center: i32,
    /// Number of pegs in the board
    pub peg_count: u32,
    /// Solution represenation.
    pub solution: Vec<GameMove>,
}

impl Solver {
    /// Initializes a Solver struct from a file.
    ///
    /// It will return an error if the `file_path` is invalid or if the contents
    /// do not represent a valid board. Performs the same checks as [`Solver::init`]
    pub fn init_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        #[allow(clippy::single_char_pattern)]
        let contents = contents.replace("\r", "");
        let n = match contents.find('\n') {
            Some(n) => n,
            None => Err("Invalid content")?,
        };

        return Self::init(&contents, n + 1);
    }

    #[allow(rustdoc::private_intra_doc_links)]
    /// Initializes a Solver struct from a str board and a row length.
    ///
    /// It will return an error if the board isn't valid:
    ///
    /// - A board isn't valid, if multiple centers are defined
    /// - Further validation is performed by [`board::validate_board`]
    ///
    pub fn init(init_board: &str, row_length: usize) -> Result<Self, Box<dyn Error>> {
        let mut center: i32 = -1;
        let mut board: Vec<char> = Vec::new();
        let solution: Vec<GameMove> = Vec::new();
        let mut peg_count = 0;
        for (pos, field) in init_board.chars().enumerate() {
            if field == '◎' {
                if center > -1 {
                    Err("Center already defined")?
                }
                center = pos as i32;
                board.push('○');
            } else if field == '◉' {
                if center > -1 {
                    Err("Center already defined")?
                }
                center = pos as i32;
                board.push('●');
                peg_count += 1;
            } else {
                if field == '●' {
                    peg_count += 1;
                }
                board.push(field);
            }
        }

        if !validate_board(&board) {
            Err("Invalid board")?
        }

        Ok(Self {
            row_length,
            center,
            board,
            solution,
            peg_count,
        })
    }

    /// Makes a move starting from `pos` in direction `dir` at `board`
    fn make_move(board: &mut [char], pos: i32, dir: i32) {
        board[pos as usize] = '○';
        board[(pos + dir) as usize] = '○';
        board[(pos + 2 * dir) as usize] = '●';
    }

    /// Reverts a previously executed valid move.
    fn unmove(board: &mut [char], pos: i32, dir: i32) {
        board[pos as usize] = '●';
        board[(pos + dir) as usize] = '●';
        board[(pos + 2 * dir) as usize] = '○';
    }

    /// Finds next move in the `board`.
    ///
    /// If the move exists it returns the tuple `(pos, dir, index)`. Otherwise it returns `(-1, 0, 0)`.
    /// The searching starts from `skip_items` index, so that the same moves aren't repeatedly tried.
    fn find_next_move(board: &[char], dirs: [i32; 4], skip_items: usize) -> (i32, i32, usize) {
        for (i, ((pos, _), dir)) in
            iproduct!(board.iter().enumerate().filter(|p| *p.1 == '●'), dirs)
                .skip(skip_items)
                .enumerate()
        {
            let pos = pos as i32;
            if board[pos as usize] == '●'
                && board[(pos + dir) as usize] == '●'
                && board[(pos + 2 * dir) as usize] == '○'
            {
                return (pos, dir, skip_items + i);
            }
        }
        (-1, 0, 0)
    }

    /// Solves a board
    ///
    /// It tries to find a sequence of moves such that there is only one peg left at the end;
    /// If center is >= 0, that last peg must be in the center position.
    /// If a solution is found, it will return true.
    pub fn solve(&mut self) -> bool {
        let mut last: i32 = -1;

        let dirs = [-1, -(self.row_length as i32), 1, (self.row_length as i32)];
        let mut board = self.board.clone();
        let mut moves: Vec<(i32, i32, usize)> = Vec::new();
        let mut skip_items: usize = 0;
        let mut max_move_count = 0;

        while moves.len() < ((self.peg_count - 1) as usize)
            || !(self.center < 0 || last == self.center)
        {
            let (pos, dir, last_move_index) = Self::find_next_move(&self.board, dirs, skip_items);
            if pos > 0 {
                Self::make_move(&mut self.board, pos, dir);
                last = pos + 2 * dir;
                moves.push((pos, dir, last_move_index));
                skip_items = 0;

                if moves.len() > max_move_count {
                    max_move_count += 1;
                    println!("Moves so far {}/{}", max_move_count, self.peg_count - 1);
                }
            } else if !moves.is_empty() {
                let last_move = moves.pop().unwrap();
                Self::unmove(&mut self.board, last_move.0, last_move.1);
                skip_items = last_move.2 + 1;
            } else {
                return false;
            }
        }

        for (pos, dir, _) in moves {
            self.solution.push(GameMove {
                board: board.clone(),
                start_pos: pos as usize,
                direction: dir.into(),
            });
            Self::make_move(&mut board, pos, dir);
        }
        self.solution.push(GameMove {
            board: board.clone(),
            start_pos: 0,
            direction: 0.into(),
        });

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 11 + 1; // length of a row (+1 for \n)

    // The board must be surrounded by 2 illegal
    // fields in each direction so that move()
    // doesn't need to check the board boundaries.
    // Periods represent illegal fields,
    // ● are pegs, and ○ are holes.
    const BOARD: &str = "...........
...........
....●●●....
....●●●....
..●●●●●●●..
..●●●◎●●●..
..●●●●●●●..
....●●●....
....●●●....
...........
...........";

    #[test]
    fn test_english_peg_solo_moves() {
        let mut solver = Solver::init(BOARD, N).unwrap();
        solver.solve();
        assert_eq!(solver.solution.len(), 32);
    }

    #[test]
    fn test_english_peg_solo_from_file_moves() {
        let mut solver = Solver::init_from_file("games/english.txt").unwrap();
        solver.solve();
        assert_eq!(solver.solution.len(), 32);
    }

    const BOARD_NO_CENTER: &str = "...........
...........
....●●●....
....●●●....
..●●●●●●●..
..●●●○●●●..
..●●●●●●●..
....●●●....
....●●●....
...........
...........";

    #[test]
    fn test_english_peg_no_center() {
        let mut solver = Solver::init(BOARD_NO_CENTER, N).unwrap();
        solver.solve();
        assert_eq!(solver.solution.len(), 32);
    }

    const BOARD_TWO_CENTERS: &str = "...........
...........
....●●●....
....●●●....
..●●●●●●●..
..●●●◎●●●..
..●●●●●●●..
....●◎●....
....●●●....
...........
...........";

    #[should_panic(expected = "Center already defined")]
    #[test]
    fn test_english_peg_2_centers() {
        let _solver = Solver::init(BOARD_TWO_CENTERS, N).unwrap();
    }
}
