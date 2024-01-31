pub mod solve;
pub mod print;

use std::cmp::Ordering;
use rand::{Rng, seq::SliceRandom, thread_rng};

// 9 lowest bits are true
const ALL_POSSIBLE: u16 =  0b0000000111111111;
const NONE_POSSIBLE: u16 = 0b0000000000000000;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tile {
    Void,
    Num(usize),
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = match self {
            Tile::Void => 0,
            Tile::Num(x) => x + 1,
        };
        let o = match other {
            Tile::Void => 0,
            Tile::Num(x) => x + 1,
        };
        return s.cmp(&o);
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SolutionCount {
    Zero,
    One(Sudoku),
    Multiple,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Difficulty {
    Trivial,
    Easy,
    LevelOne(usize),
    LevelTwo(usize),
    LevelThree(usize),
    LiterallyZeroSolutions,
    LiterallyMultipleSolutions,
    TooDeep,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Void => write!(f, "."),
            Tile::Num(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Sudoku {
    pub board: [[Tile; 9]; 9],
    pub possible: [[u16; 9]; 9],
    // order is column index, then number
    pub column_possible: [[u16; 9]; 9],
    pub row_possible: [[u16; 9]; 9],
    pub box_possible: [[u16; 9]; 9],
    pub clues: usize,
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

    pub fn reduce_to_n_random(&mut self, n: usize) -> Sudoku {
        if n > 81 {
            panic!("n is too large");
        }
        for x in 0..9 {
            for y in 0..9 {
                if self.board[x][y] == Tile::Void {
                    panic!("incomplete Sudoku");
                }
            }
        }
        let mut res = Sudoku::new_blank();
        // Shuffle the indicies, then draw the first n
        let mut indicies = (0..81).collect::<Vec<_>>();
        indicies.shuffle(&mut thread_rng());
        let indicies: Vec<_> = indicies.into_iter().take(n).collect();
        for index in indicies {
            let x = index % 9;
            let y = index / 9;
            res.set_tile_at(x, y, self.board[x][y]);
        }
        res.clues = n;
        return res;
    }

    pub fn get_all_reduced_by_one_clue(&self) -> Vec<Sudoku> {
        let mut res = vec![];
        for x in 0..9 {
            for y in 0..9 {
                if self.board[x][y] == Tile::Void {
                    continue;
                }
                // We cannot copy then remove a clue, as the possibilities would not be aligned
                let mut new_sudoku = Sudoku::new_blank();
                for x1 in 0..9 {
                    for y1 in 0..9 {
                        if x == x1 && y == y1 {
                            continue;
                        }
                        new_sudoku.set_tile_at(x1, y1, self.board[x1][y1]);
                    }
                }
                let mut to_solve = Sudoku::from_sudoku(&new_sudoku);
                let solution_count = to_solve.solve(false);
                new_sudoku.clues = self.clues - 1;
                if let SolutionCount::One(_) = solution_count {
                    res.push(new_sudoku);
                }
            }
        }
        return res;
    }

    pub fn get_difficulty(&self, depth: usize) -> Difficulty {
        if depth > 0 {
            let cheat = self.get_difficulty(depth - 1);
            if cheat != Difficulty::TooDeep {
                return cheat;
            }
        }
        let mut new = Sudoku::from_sudoku(self);
        let trivial = new.solve_no_guessing(true, 100, false);
        if let SolutionCount::One(_) = trivial {
            return Difficulty::Trivial;
        }
        let easy = new.solve_no_guessing(false, 100, false);
        if let SolutionCount::One(_) = trivial {
            return Difficulty::Easy;
        }
        let solution_count = new.solve(false);
        match solution_count {
            SolutionCount::Zero => return Difficulty::LiterallyZeroSolutions,
            SolutionCount::Multiple => return Difficulty::LiterallyMultipleSolutions,
            SolutionCount::One(solution) => {
                if depth < 1 {
                    return Difficulty::TooDeep;
                }
                let mut level = 4;
                let mut level_1_count = 0;
                let mut level_2_count = 0;
                let mut level_3_count = 0;
                // Compute level
                for x in 0..9 {
                    for y in 0..9 {
                        if self.board[x][y] != Tile::Void {
                            continue;
                        }
                        let mut new = Sudoku::from_sudoku(self);
                        new.set_tile_at(x, y, solution.board[x][y]);
                        let new_difficulty = new.get_difficulty(depth - 1);
                        match new_difficulty {
                            Difficulty::LiterallyZeroSolutions | Difficulty::LiterallyMultipleSolutions => return new_difficulty,
                            Difficulty::Trivial | Difficulty::Easy => {
                                level = 1;
                                level_1_count += 1;
                            },
                            Difficulty::LevelOne(x) => {
                                if level > 2 {
                                    level = 2;
                                    level_2_count += x;
                                }
                            },
                            Difficulty::LevelTwo(x) => {
                                if level > 3 {
                                    level = 3;
                                    level_3_count += x;
                                }
                            },
                            Difficulty::LevelThree(_) => { // lmao this is unlikely
                                println!("CONGRATS! You have won Sudoku!!!");
                            },
                            Difficulty::TooDeep => {},
                        }
                    }
                }
                match level {
                    1 => { return Difficulty::LevelOne(level_1_count) },
                    2 => { return Difficulty::LevelTwo(level_1_count) },
                    3 => { return Difficulty::LevelThree(level_3_count) },
                    _ => { return Difficulty::TooDeep },
                }
            },
        }

    }
}

impl PartialOrd for Sudoku {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sudoku {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for x in 0..9 {
            for y in 0..9 {
                if self.board[x][y] > other.board[x][y] {
                    return Ordering::Greater;
                }
                if self.board[x][y] < other.board[x][y] {
                    return Ordering::Less;
                }
            }
        }
        return Ordering::Equal;
    }
}
