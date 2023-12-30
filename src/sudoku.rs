pub mod solve;
pub mod print;

use rand::Rng;

// 9 lowest bits are true
const ALL_POSSIBLE: u16 =  0b0000000111111111;
const NONE_POSSIBLE: u16 = 0b0000000000000000;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Void,
    Num(usize),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SolutionCount {
    Zero,
    One(Sudoku),
    Multiple
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Void => write!(f, "."),
            Tile::Num(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sudoku {
    pub board: [[Tile; 9]; 9],
    pub possible: [[u16; 9]; 9],
    // order is column index, then number
    pub column_possible: [[u16; 9]; 9],
    pub row_possible: [[u16; 9]; 9],
    pub box_possible: [[u16; 9]; 9],
    pub clues: u32,
}

impl Sudoku {
    fn get_tile_from_digit(digit: u8) -> Tile {
        if digit == b'.' {
            return Tile::Void;
        }
        let digit = digit - 48;  // ASCII '0'
        if digit == 0 {
            return Tile::Void;
        }
        Tile::Num(digit.into())
    }
    fn new_blank() -> Self {
        let board = [[Tile::Void; 9]; 9];
        let possible = [[ALL_POSSIBLE; 9]; 9];
        let column_possible = [[ALL_POSSIBLE; 9]; 9];
        let row_possible = [[ALL_POSSIBLE; 9]; 9];
        let box_possible = [[ALL_POSSIBLE; 9]; 9];
        Self { board, possible, column_possible, row_possible, box_possible, clues: 0 }
    }
    pub fn from_string(input: &str) -> Self {
        if input.len() != 81 {
            panic!("not the right length");
        }
        let mut res = Self::new_blank();
        for i in 0..81 {
            let x_pos = i % 9;
            let y_pos = i / 9;
            let digit = input.as_bytes()[i];
            let tile = Self::get_tile_from_digit(digit);
            res.set_tile_at(x_pos, y_pos, tile);
            if tile != Tile::Void {
                res.clues += 1;
            }
        }
        return res;
    }
    pub fn from_sudoku(s: &Self) -> Self {
        let board = s.board.clone();
        let possible = s.possible.clone();
        let column_possible = s.column_possible.clone();
        let row_possible = s.row_possible.clone();
        let box_possible = s.box_possible.clone();
        let clues = s.clues;
        Self {board, possible, column_possible, row_possible, box_possible, clues}
    }
    fn set_tile_at(&mut self, x_pos: usize, y_pos: usize, tile: Tile) {
        self.board[x_pos][y_pos] = tile;

        match tile {
            Tile::Void => {},
            Tile::Num(val) => {
                self.possible[x_pos][y_pos] = NONE_POSSIBLE;
                let column_mask = !(1 << y_pos);
                let row_mask = !(1 << x_pos);
                let box_index = x_pos / 3 + 3 * (y_pos / 3);
                let index_in_box = x_pos % 3 + 3 * (y_pos % 3);
                let box_mask = !(1 << index_in_box);

                for val in 1..=9 {
                    self.column_possible[x_pos][val - 1] &= column_mask;
                    self.row_possible[y_pos][val - 1] &= row_mask;
                    self.box_possible[box_index][val - 1] &= box_mask;
                }
                for x in 0..9 {
                    self.remove_possible_at(x, y_pos, val);
                }
                for y in 0..9 {
                    self.remove_possible_at(x_pos, y, val);
                }
                let box_x = x_pos - x_pos % 3;
                let box_y = y_pos - y_pos % 3;
                for dx in 0..3 {
                    for dy in 0..3 {
                        self.remove_possible_at(box_x + dx, box_y + dy, val);
                    }
                }
            }
        }
    }

    #[inline]
    pub fn is_possible_at(&mut self, x: usize, y: usize, num: usize) -> bool {
        let mask = 1 << (num - 1);
        return (mask & self.possible[x][y]) != 0;
    }

    #[inline]
    pub fn remove_possible_at(&mut self, x: usize, y: usize, num: usize) {
        let mask = 1 << (num - 1);
        let mask = !mask;
        self.possible[x][y] &= mask;

        let mask = 1 << y;
        let mask = !mask;
        self.column_possible[x][num - 1] &= mask;

        let mask = 1 << x;
        let mask = !mask;
        self.row_possible[y][num - 1] &= mask;

        let box_index = x / 3 + 3 * (y / 3);
        let index_in_box = x % 3 + 3 * (y % 3);
        let mask = 1 << index_in_box;
        let mask = !mask;
        self.box_possible[box_index][num - 1] &= mask;
    }

    // assumes the Sudoku is solved
    pub fn reduce_clues(&self, target: usize) -> Option<Self> {
        let mut rng = rand::thread_rng();
        let proportion = target as f64 / 81.0;
        for _ in 0..100 {
            let new_sudoku = Self::from_sudoku(self);
            for x in 0..9 {
                for y in 0..9 {
                    if rng.gen::<f64>() <= proportion {
                        // do things
                    }
                }
            }
        }
        todo!();
    }
}
