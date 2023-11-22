//! This crate provides code for solving the [English peg solitaire puzzle](http://en.wikipedia.org/wiki/Peg_solitaire)
//! and can produce image and gif solutions. The solver and input puzzle format is based on a
//! [Golang example](https://go.dev/test/solitaire.go). The code extends the existing sample so that it can work
//! with other variations of the puzzle, allowing the user to define a custom board.

use std::{error::Error, fs};

use board::{validate_board, GameMove};
use itertools::iproduct;

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
    /// Number of moves
    pub moves: i32,
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
        let moves = 0;
        let mut board: Vec<char> = Vec::new();
        let solution: Vec<GameMove> = Vec::new();
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
            } else {
                board.push(field);
            }
        }

        if !validate_board(&board) {
            Err("Invalid board")?
        }

        Ok(Self {
            row_length,
            center,
            moves,
            board,
            solution,
        })
    }

    /// Tests if there is a peg at position pos that can jump over another peg in direction dir.
    ///
    /// If the move is valid, it is executed and move returns true. Otherwise, make_move returns false.
    fn make_move(board: &mut Vec<char>, pos: i32, dir: i32) -> bool {
        if board[pos as usize] == '●'
            && board[(pos + dir) as usize] == '●'
            && board[(pos + 2 * dir) as usize] == '○'
        {
            board[pos as usize] = '○';
            board[(pos + dir) as usize] = '○';
            board[(pos + 2 * dir) as usize] = '●';
            return true;
        }
        false
    }

    /// Reverts a previously executed valid move.
    fn unmove(board: &mut Vec<char>, pos: i32, dir: i32) {
        board[pos as usize] = '●';
        board[(pos + dir) as usize] = '●';
        board[(pos + 2 * dir) as usize] = '○';
    }

    /// Solves a board
    ///
    /// It tries to find a sequence of moves such that there is only one peg left at the end;
    /// If center is >= 0, that last peg must be in the center position.
    /// If a solution is found, it will return true.
    pub fn solve(&mut self) -> bool {
        let mut last: i32 = 0;
        let mut n: i32 = 0;

        let board = self.board.clone();
        let dirs = [-1, -(self.row_length as i32), 1, (self.row_length as i32)];
        for ((pos, _), dir) in iproduct!(board.iter().enumerate().filter(|p| *p.1 == '●'), dirs) {
            if Self::make_move(&mut self.board, pos as i32, dir) {
                // a valid move was found and executed,
                // see if this new board has a solution
                if self.solve() {
                    Self::unmove(&mut self.board, pos as i32, dir);
                    self.solution.insert(
                        0,
                        GameMove {
                            board: self.board.clone(),
                            start_pos: pos,
                            direction: dir.into(),
                        },
                    );
                    self.moves += 1;
                    return true;
                }
                Self::unmove(&mut self.board, pos as i32, dir);
            }
            self.moves += 1;
            if dir == dirs[3] {
                last = pos as i32;
                n += 1;
            }
        }
        // tried each possible move
        if n == 1 && (self.center < 0 || last == self.center) {
            // there's only one peg left
            self.solution.insert(
                0,
                GameMove {
                    board: self.board.clone(),
                    start_pos: 0,
                    direction: 0.into(),
                },
            );

            return true;
        }
        // no solution found for this board
        false
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
        let mut solver = Solver::init_from_file("games/english_peg_solo.txt").unwrap();
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
