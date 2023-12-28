#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use rand::Rng;

fn main() {
    // get_reduced_sudoku();
    // test_first_sudoku();
    test_every_sudoku();
}

fn get_reduced_sudoku() {
    let file_path = "data/sudoku-rated.csv";
    let binding = fs::read_to_string(file_path).expect("pls work");
    let mut lines = binding.lines();
    lines.next();  // first line is "puzzle"
    let split = lines.next().expect("pls").split(',').collect::<Vec<_>>();
    let string = split[1];
    let mut sudoku = Sudoku::from_string(string);
    let solutions = sudoku.solve(false);
    match solutions {
        SolutionCount::Zero | SolutionCount::Multiple => println!("{:?}", solutions),
        SolutionCount::One(sud) => {
            let reduced = sud.reduce_clues(26);
            println!("{:?}", reduced);
        },
    }
}

fn test_first_sudoku() {
    let file_path = "data/sudoku17.csv";
    let binding = fs::read_to_string(file_path).expect("pls work");
    let mut lines = binding.lines();
    lines.next();  // first line is "puzzle"
    let split = lines.next().expect("pls").split(',').collect::<Vec<_>>();
    let string = split[0];
    let mut sudoku = Sudoku::from_string(string);
    sudoku.print_with_possibilities();
    println!("{}", sudoku);
    let solutions = sudoku.solve(true);
    match solutions {
        SolutionCount::Zero | SolutionCount::Multiple => println!("{:?}", solutions),
        SolutionCount::One(sud) => sud.print_with_possibilities(),
    }
}

fn test_every_sudoku() {
    let file_path = "data/sudoku17.csv";
    let binding = fs::read_to_string(file_path).expect("pls work");
    let mut lines = binding.lines();
    lines.next();  // first line is "puzzle"
    let mut max_difficulty = f32::MIN;
    let puzzle_count = 20_000_000;
    let mut solved_count = 0;
    let mut total_millis = 0.0;
    let mut max_millis = 0.0;
    let debug = false;
    // println!("Solving {} puzzles", puzzle_count);
    for i in 0..puzzle_count {
        let next_line = lines.next();
        if next_line == None {
            break;
        }
        let split = next_line.expect("not None").split(',').collect::<Vec<_>>();
        let string = split[0];
        let mut sudoku = Sudoku::from_string(string);
        // let difficulty: f32 = split[4].parse().unwrap();
        let difficulty = 10.0;
        if difficulty < 9.0 {
            continue;
        }
        solved_count += 1;
        let start = Instant::now();
        let solutions = sudoku.solve(debug);
        match solutions {
            SolutionCount::Zero | SolutionCount::Multiple => { println!("{:?}", solutions); },
            SolutionCount::One(s) => { /* println!("{}", s) */ },
        }
        let millis = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        if difficulty > max_difficulty {
            max_difficulty = difficulty;
            println!("Max Difficulty: {} ({}ms)", max_difficulty, millis);
        }
        total_millis += millis;
        if millis > 10.0 {
            println!("Difficulty: {}\tIndex: {}\tTook {}ms", difficulty, i, millis);
        }
        if millis > max_millis {
            max_millis = millis;
        }
    }
    println!("Solved {} puzzles", solved_count);
    println!("Average time: {}ms", total_millis as f32 / solved_count as f32);
    println!("Max time: {}ms", max_millis as f32);
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
    clues: u32
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
        let possible = [[[true; 9]; 9]; 9];
        Self { board, possible, clues: 0 }
    }
    fn from_string(input: &str) -> Self {
        let board = [[Tile::Void; 9]; 9];
        let possible = [[[true; 9]; 9]; 9];
        if input.len() != 81 {
            panic!("not the right length");
        }
        let mut res = Self { board, possible, clues: 0 };
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
    fn from_sudoku(s: &Self) -> Self {
        let board = s.board.clone();
        let possible = s.possible.clone();
        let clues = s.clues;
        Self {board, possible, clues}
    }
    fn set_tile_at(&mut self, x_pos: usize, y_pos: usize, tile: Tile) {
        self.board[x_pos][y_pos] = tile;

        match tile {
            Tile::Void => {},
            Tile::Num(val) => {
                for val_b in 1..=9 {
                    self.possible[x_pos][y_pos][val_b - 1] = false;
                }
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

    fn get_naked_single(&self, x_pos: usize, y_pos: usize) ->  (Tile, bool) {
        if self.board[x_pos][y_pos] != Tile::Void {
            return (Tile::Void, false);
        }
        let mut res = Tile::Void;
        for i in 1..=9 {
            if self.possible[x_pos][y_pos][i - 1] {
                if res == Tile::Void { res = Tile::Num(i) }
                else { return (Tile::Void, false) }
            }
        }
        (res, res == Tile::Void)
    }
    fn get_naked_pair(&self, x_pos: usize, y_pos: usize) -> (Tile, Tile) {
        let mut first = Tile::Void;
        let mut second = Tile::Void;
        for i in 1..=9 {
            if self.possible[x_pos][y_pos][i - 1] {
                if first == Tile::Void { first = Tile::Num(i) }
                else if second == Tile::Void { second = Tile::Num(i) }
                else { return (Tile::Void, Tile::Void) }
            }
        }
        if second == Tile::Void {
            return (Tile::Void, Tile::Void);
        }
        (first, second)
    }
    fn get_best_guess_spot(&self, debug: bool) -> (usize, usize) {
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

    fn fill_naked_singles(&mut self, debug: bool) -> (bool, bool, bool) {
        let mut changed = false;
        let mut complete = true;
        for x in 0..9 {
            for y in 0..9 {
                if self.board[x][y] == Tile::Void {
                    complete = false;
                }
                let naked = self.get_naked_single(x, y);
                if naked.1 {
                    return (changed, complete, true);
                }
                if naked.0 != Tile::Void {
                    self.set_tile_at(x, y, naked.0);
                    if debug {
                        println!("naked single");
                        // self.print_with_possibilities();
                    }
                    changed = true;
                }
            }
        }
        (changed, complete, false)
    }

    fn fill_last_in_column(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for x in 0..9 {
            for val in 1..=9 {
                let mut possible_count = 0;
                let mut possible_at = None;
                for y in 0..9 {
                    if self.possible[x][y][val - 1] {
                        possible_count += 1;
                        possible_at = Some(y);
                        if possible_count > 1 {
                            break;
                        }
                    }
                }
                if possible_count == 1 {
                    self.set_tile_at(x, possible_at.expect("pls"), Tile::Num(val));
                    if debug {
                        println!("last column");
                        // self.print_with_possibilities();
                    }
                    changed = true;
                }
            }
        }
        changed
    }

    fn fill_last_in_row(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for y in 0..9 {
            for val in 1..=9 {
                let mut possible_count = 0;
                let mut possible_at = None;
                for x in 0..9 {
                    if self.possible[x][y][val - 1] {
                        possible_count += 1;
                        possible_at = Some(x);
                        if possible_count > 1 {
                            break;
                        }
                    }
                }
                if possible_count == 1 {
                    self.set_tile_at(possible_at.expect("pls"), y, Tile::Num(val));
                    if debug {
                        println!("last row");
                        // self.print_with_possibilities();
                    }
                    changed = true;
                }
            }
        }
        changed
    }

    fn fill_last_in_box(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for box_x in (0..9).step_by(3) {
            for box_y in (0..9).step_by(3) {
                for val in 1..=9 {
                    let mut possible_count = 0;
                    let mut possible_at_dx = None;
                    let mut possible_at_dy = None;
                    for dx in 0..3 {
                        for dy in 0..3 {
                            if self.possible[box_x + dx][box_y + dy][val - 1] {
                                possible_count += 1;
                                possible_at_dx = Some(dx);
                                possible_at_dy = Some(dy);
                                if possible_count > 1 {
                                    break;
                                }
                            }
                        }
                    }
                    if possible_count == 1 {
                        let x = box_x + possible_at_dx.expect("pls");
                        let y = box_y + possible_at_dy.expect("pls");
                        self.set_tile_at(x, y, Tile::Num(val));
                        if debug {
                            println!("last box");
                            // self.print_with_possibilities();
                        }
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_pairs(&mut self, debug: bool) {
        todo!();
    }

    fn apply_naked_pairs(&mut self, debug: bool) {
        let mut naked_pair_list: Vec<((Tile, Tile), usize, usize)> = vec![];
        for x in 0..9 {
            for y in 0..9 {
                let naked_pair = self.get_naked_pair(x, y);
                // If the pair is not void
                if let Tile::Num(pair_a) = naked_pair.0 {
                    if let Tile::Num(pair_b) = naked_pair.1 {
                        for naked_pair_coords in &naked_pair_list {
                            let other_pair = naked_pair_coords.0;
                            let other_x = naked_pair_coords.1;
                            let other_y = naked_pair_coords.2;
                            if other_pair == naked_pair {
                                // if in the same column
                                if x == other_x {
                                    for y_2 in 0..9 {
                                        if y_2 == y || y_2 == other_y {
                                            continue;
                                        }
                                        // remove the possibilities from others in the column
                                        let mut changed = false;
                                        if self.possible[x][y_2][pair_a - 1] || self.possible[x][y_2][pair_b - 1] {
                                            changed = true;
                                        }
                                        self.possible[x][y_2][pair_a - 1] = false;
                                        self.possible[x][y_2][pair_b - 1] = false;
                                        if debug && changed {
                                            println!("naked pairing ({}, {})", x, y_2);
                                            self.print_with_possibilities();
                                        }
                                    }
                                }
                                // if in the same row (we can use else as the coords are different)
                                else if y == other_y {
                                    for x_2 in 0..9 {
                                        if x_2 == x || x_2 == other_x {
                                            continue;
                                        }
                                        // remove the possibilities from others in the row
                                        let mut changed = false;
                                        if self.possible[x_2][y][pair_a - 1] || self.possible[x_2][y][pair_b - 1] {
                                            changed = true;
                                        }
                                        self.possible[x_2][y][pair_a - 1] = false;
                                        self.possible[x_2][y][pair_b - 1] = false;
                                        if debug && changed {
                                            println!("naked pairing ({}, {})", x_2, y);
                                            self.print_with_possibilities();
                                        }
                                    }
                                }
                                // if in the same box
                                if x / 3 == other_x / 3 && y / 3 == other_y / 3 {
                                    let box_x = x - x % 3;
                                    let box_y = y - y % 3;
                                    for dx in 0..3 {
                                        for dy in 0..3 {
                                            let remove_x = box_x + dx;
                                            let remove_y = box_y + dy;
                                            if remove_x == x && remove_y == y {
                                                continue;
                                            }
                                            if remove_x == other_x && remove_y == other_y {
                                                continue;
                                            }
                                            // remove the possibilities from others in the box
                                            let mut changed = false;
                                            if self.possible[remove_x][remove_y][pair_a - 1] || self.possible[remove_x][remove_y][pair_b - 1] {
                                                changed = true;
                                            }
                                            self.possible[remove_x][remove_y][pair_a - 1] = false;
                                            self.possible[remove_x][remove_y][pair_b - 1] = false;
                                            if debug && changed {
                                                println!("naked pairing ({}, {})", remove_x, remove_y);
                                                self.print_with_possibilities();
                                            }
                                        }
                                    }
                                }
                            }
                        }  // end for
                        naked_pair_list.push((naked_pair, x, y));
                    }
                }
            }
        }
    }

    fn solve(&mut self, debug: bool) -> SolutionCount {
        for _ in 0..100 {
            let (mut changed, complete, no_solutions) = self.fill_naked_singles(debug);
            if no_solutions {
                return SolutionCount::Zero;
            }
            if complete {
                return SolutionCount::One(*self);
            }

            // Last remaining in columns, rows, boxes
            changed |= self.fill_last_in_column(debug);
            changed |= self.fill_last_in_row(debug);
            changed |= self.fill_last_in_box(debug);


            // naked pairs (this can slow down the performance)
            self.apply_naked_pairs(debug);

            if !changed {
                break;
            }
        }
        let (best_x, best_y) = self.get_best_guess_spot(debug);
        let mut solution = None;
        if debug {
            println!("backtracking");
        }
        for i in 0..9 {
            if self.possible[best_x][best_y][i] {
                let mut new_sudoku = Self::from_sudoku(self);
                new_sudoku.set_tile_at(best_x, best_y, Tile::Num(i + 1));
                match new_sudoku.solve(debug) {
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

    // assumes the Sudoku is solved
    fn reduce_clues(&self, target: usize) -> Option<Self> {
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

    fn print_with_possibilities(&self) {
        println!("-------------------------------------");
        for j in 0..9 {
            for repeat in 0..3 {
                print!("|");
                for i in 0..9 {
                    if self.board[i][j] == Tile::Void {
                        // print possibilities
                        for k in 0..3 {
                            let index = 3 * repeat + k;
                            if self.possible[i][j][index] {
                                print!("{}", index + 1);
                            } else {
                                print!(".");
                            }
                        }
                    } else {
                        for _ in 0..3 {
                            print!("{}", self.board[i][j]);
                        }
                    }
                    if i % 3 == 2 {
                        print!("|");
                    } else {
                        print!(" ");
                    }
                }
                println!();
                if j % 3 == 2 && repeat == 2 {
                    println!("-------------------------------------");
                }
            }
            if j % 3 != 2 {
                println!("|           |           |           |");
            }
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
