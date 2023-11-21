//! Provides helper methods for working with boards

/// Maximum columns per row allowed
const MAX_COLUMN_COUNT: usize = 40;

/// Board representation
pub struct GameBoard {
    /// Board as a vector of characters
    pub board: Vec<char>,
    /// Row count
    pub row_count: u32,
    /// Column count
    pub column_count: u32,
}

/// Representation of a move's direction
#[derive(Debug, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    Still,
}

impl From<i32> for MoveDirection {
    fn from(value: i32) -> Self {
        if value == 1 {
            return Self::Right;
        }
        if value == -1 {
            return Self::Left;
        }
        if value > 1 {
            return Self::Down;
        }
        if value < -1 {
            return Self::Up;
        }
        return Self::Still;
    }
}

/// Representation of a move
#[derive(Debug)]
pub struct GameMove {
    /// Board as a vector of characters
    pub board: Vec<char>,
    /// Start positino of the move
    pub start_pos: usize,
    /// Direction of the move
    pub direction: MoveDirection,
}

/// Checks that a board is valid
///
/// - The board must only contain valid characters
/// - Column count must not be bigger than [`MAX_COLUMN_COUNT`]
/// - Every column must be of the same size
pub fn validate_board(board: &Vec<char>) -> bool {
    let size = board.len();
    let mut row_count = 0;
    let mut column_count = 0;
    let mut row = 0;
    let mut column = 0;

    for ch in board.iter() {
        if row_count == 0 {
            column_count += 1;
            if *ch != '.' && *ch != '\n' {
                return false;
            }
            if *ch == '\n' {
                row_count = (size + 1) / column_count;
                row = 1;
            }
            if column_count == MAX_COLUMN_COUNT {
                return false;
            }
        } else {
            if row > 1 && row < row_count - 2 && column > 1 && column < column_count - 3 {
                if *ch != '.' && *ch != '●' && *ch != '○' && *ch != '◎' && *ch != '◉' {
                    return false;
                }
            } else if *ch != '.' && *ch != '\n' {
                return false;
            }
            column += 1;
            if *ch == '\n' {
                if column != column_count {
                    return false;
                }
                row += 1;
                column = 0;
            }
        }
    }

    true
}

/// Clears the border in each direction.
///
/// It takes a `&Vec<char>` as input and returns a `GameBoard`. Useful for creating solutions.
pub fn clear_border(board: &Vec<char>) -> GameBoard {
    let size = board.len();
    let mut row_count = 0;
    let mut column_count = 0;
    let mut row = 0;
    let mut column = 0;

    let mut new_board: Vec<char> = Vec::new();

    for ch in board.iter() {
        if row_count == 0 {
            column_count += 1;
            if *ch == '\n' {
                row_count = (size + 1) / column_count;
                row = 1;
            }
            if column_count == MAX_COLUMN_COUNT {
                panic!("Invalid board");
            }
        } else {
            if row > 1 && row < row_count - 2 && column > 1 && column < column_count - 3 {
                new_board.push(*ch);
            }
            column += 1;
            if *ch == '\n' {
                if row > 1 && row < row_count - 2 {
                    new_board.push(*ch);
                }
                row += 1;
                column = 0;
            }
        }
    }

    GameBoard {
        board: new_board,
        row_count: (row_count as u32) - 4,
        column_count: (column_count as u32) - 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOARD1: &str = "...........
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
    fn test_board1_valid() {
        let mut board: Vec<char> = Vec::new();

        for ch in BOARD1.chars() {
            board.push(ch);
        }

        assert!(validate_board(&board));
    }
}
