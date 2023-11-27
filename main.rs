#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    test_every_sudoku();
}

fn test_every_sudoku() {
    let file_path = "data/sudoku-rated.csv";
    let binding = fs::read_to_string(file_path).expect("pls work");
    let mut lines = binding.lines();
    lines.next();  // first line is "puzzle"
    let mut max_difficulty = f32::MIN;
    let puzzle_count = 200;
    let mut total_millis = 0;
    println!("Solving {} puzzles", puzzle_count);
    for i in 0..puzzle_count {
        let split = lines.next().expect("pls").split(',').collect::<Vec<_>>();
        let string = split[1];
        let mut sudoku = Sudoku::from_string(string);
        let difficulty: f32 = split[4].parse().unwrap();
        let start = Instant::now();
        let solutions = sudoku.solve();
        match solutions {
            SolutionCount::Zero | SolutionCount::Multiple => { println!("{:?}", solutions); },
            SolutionCount::One(s) => { /* println!("{}", s) */ },
        }
        if difficulty > max_difficulty {
            max_difficulty = difficulty;
            println!("Max Difficulty: {}", max_difficulty);
        }
        let millis = start.elapsed().as_millis();
        total_millis += millis;
        if millis > 1000 {
            println!("Difficulty: {}\tTook {}ms", difficulty, millis);
        }
    }
    println!("Average time: {}ms", total_millis as f32 / puzzle_count as f32);
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Void,
    Num(usize),
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum SolutionCount {
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
struct Sudoku {
    board: [[Tile; 9]; 9],
    possible: [[[bool; 9]; 9]; 9],
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
    fn from_string(input: &str) -> Self {
        let board = [[Tile::Void; 9]; 9];
        let possible = [[[true; 9]; 9]; 9];
        if input.len() != 81 {
            panic!("not the right length");
        }
        let mut res = Self { board, possible };
        for i in 0..81 {
            let x_pos = i % 9;
            let y_pos = i / 9;
            let digit = input.as_bytes()[i];
            let tile = Self::get_tile_from_digit(digit);
            res.set_tile_at(x_pos, y_pos, tile);
        }
        return res;
    }
    fn from_sudoku(s: &Self) -> Self {
        let board = s.board.clone();
        let possible = s.possible.clone();
        Self {board, possible}
    }
    fn set_tile_at(&mut self, x_pos: usize, y_pos: usize, tile: Tile) {
        self.board[x_pos][y_pos] = tile;
        match tile {
            Tile::Void => {},
            Tile::Num(val) => {
                self.possible[x_pos][y_pos] = [false; 9];
                for x in 0..9 {
                    self.possible[x][y_pos][val - 1] = false;
                }
                for y in 0..9 {
                    self.possible[x_pos][y][val - 1] = false;
                }
                let box_x = x_pos - x_pos % 3;
                let box_y = y_pos - y_pos % 3;
                for dx in 0..3 {
                    for dy in 0..3 {
                        self.possible[box_x + dx][box_y + dy][val - 1] = false;
                    }
                }
            }
        }
    }

    fn get_naked_single(&self, x_pos: usize, y_pos: usize) -> Tile {
        let mut res = Tile::Void;
        for i in 1..=9 {
            if self.possible[x_pos][y_pos][i - 1] {
                if res == Tile::Void { res = Tile::Num(i) }
                else { return Tile::Void }
            }
        }
        res
    }
    fn get_best_guess_spot(&self) -> (usize, usize) {
        let mut best_x = 0;
        let mut best_y = 0;
        let mut min_possible = usize::MAX;
        for x in 0..9 {
            for y in 0..9 {
                let mut count = 0;
                for i in 0..9 {
                    if self.possible[x][y][i] {
                        count += 1;
                    }
                }
                if count > 0 && count < min_possible {
                    min_possible = count;
                    best_x = x;
                    best_y = y;
                }
            }
        }
        (best_x, best_y)
    }
    fn solve(&mut self) -> SolutionCount {
        // naked singles
        for _ in 0..100 {
            let mut changed = false;
            let mut complete = true;
            for x in 0..9 {
                for y in 0..9 {
                    if self.board[x][y] == Tile::Void {
                        complete = false;
                    }
                    let naked = self.get_naked_single(x, y);
                    if naked != Tile::Void {
                        self.set_tile_at(x, y, naked);
                        changed = true;
                    }
                }
            }
            if complete {
                return SolutionCount::One(*self);
            }
            if !changed {
                break;
            }
        }
        let (best_x, best_y) = self.get_best_guess_spot();
        let mut solution = None;
        for i in 0..9 {
            if self.possible[best_x][best_y][i] {
                let mut new_sudoku = Self::from_sudoku(self);
                new_sudoku.set_tile_at(best_x, best_y, Tile::Num(i + 1));
                match new_sudoku.solve() {
                    SolutionCount::Zero => {},
                    SolutionCount::One(s) => {
                        match solution {
                            None => { solution = Some(s); }
                            Some(_) => { return SolutionCount::Multiple }
                        }
                    }
                    SolutionCount::Multiple => { return SolutionCount::Multiple }
                }
            }
        }
        match solution {
            None => { return SolutionCount::Zero }
            Some(solution) => { return SolutionCount::One(solution)}
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "-------------------")?;
        for j in 0..9 {
            write!(f, "|")?;
            for i in 0..9 {
                write!(f, "{}", self.board[i][j])?;
                if i % 3 == 2 {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
            if j % 3 == 2 {
                writeln!(f, "-------------------")?;
            }
        }
        Ok(())
    }
}
