#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod sudoku;
pub mod reader;

use std::env;
use std::fs;
use std::time::Instant;
use itertools::Itertools;

use crate::sudoku::*;
use crate::sudoku::solve::*;
use crate::reader::*;

fn main() {
    // test_first_sudoku();
    // test_every_sudoku();
    test_no_guessing();
}

fn test_no_guessing() {
    let all_sudoku = reader::get_all_sudoku_from_path("data/sudoku17.csv");
    let total_count = all_sudoku.len();
    let mut solved_count = 0;
    let mut i = 0;
    for mut sudoku in all_sudoku {
        let solved = sudoku.solve_no_guessing(false, 100, false);
        match solved {
            SolutionCount::Zero => {},
            SolutionCount::One(_) => { solved_count += 1 },
            SolutionCount::Multiple => { panic!("this shouldn't happen lol") },
        }
        i += 1;
        if i % 100_000 == 0 {
            println!("Checked {}", i);
        }
    }
    let percentage = (solved_count as f32) / (total_count as f32) * 100.0;
    println!("Solved {} / {} sudoku ({}%)", solved_count, total_count, percentage);
}

fn test_first_sudoku() {
    let mut sudoku = reader::get_first_sudoku_from_path("data/sudoku17.csv").expect("pls work");
    sudoku.print_with_possibilities();
    println!("{}", sudoku);
    let solutions = sudoku.solve(true);
    match solutions {
        SolutionCount::Zero | SolutionCount::Multiple => println!("{:?}", solutions),
        SolutionCount::One(mut sud) => sud.print_with_possibilities(),
    }
}

fn test_every_sudoku() {
    let all_sudoku = reader::get_all_sudoku_from_path("data/sudoku17.csv");
    let mut solved_count = 0;
    let mut total_millis = 0.0;
    let mut max_millis = 0.0;
    let debug = false;
    // println!("Solving {} puzzles", puzzle_count);
    let mut index = 0;
    let mut zero_count = 0;
    let mut multi_count = 0;
    for mut sudoku in all_sudoku {
        solved_count += 1;
        let start = Instant::now();
        let solutions = sudoku.solve(debug);
        match solutions {
            SolutionCount::Zero => zero_count += 1,
            SolutionCount::Multiple => multi_count += 1,
            SolutionCount::One(s) => { /* println!("{}", s) */ },
        }
        let millis = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        total_millis += millis;
        if millis > 10.0 {
            println!("Index: {}\tTook {}ms", index, millis);
        }
        if millis > max_millis {
            max_millis = millis;
        }
        index += 1;
    }
    println!("Solved {} / {} puzzles", solved_count - zero_count - multi_count, solved_count);
    if zero_count > 0 {
        println!("{} puzzles had no solutions", zero_count);
    }
    if multi_count > 0 {
        println!("{} puzzles had multiple solutions", multi_count);
    }
    println!("Average time: {}ms", total_millis as f32 / solved_count as f32);
    println!("Max time: {}ms", max_millis as f32);
}

#[cfg(test)]
mod tests {
    use crate::reader::*;
    use crate::sudoku::*;
    use crate::sudoku::solve::*;
    #[test]
    fn test_solver() {
        let sudoku_puzzles = get_all_sudoku_from_path("data/sudoku17.csv");
        let mut index = 0;
        for mut sudoku in sudoku_puzzles {
            if let SolutionCount::One(solution) = sudoku.solve(false) {
                if !is_sudoku_solved(solution) {
                    panic!("Given solution is invalid");
                }
            } else {
                panic!("Solver found wrong number of solutions");
            }
            index += 1;
            if index >= 100 {
                // Limits solving to 100 Sudoku for time reasons
                return;
            }
        }
    }

    fn is_sudoku_solved(sudoku: Sudoku) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if sudoku.board[x][y] == Tile::Void {
                    return false;
                }
            }
        }

        for x in 0..9 {
            let mut counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
            for y in 0..9 {
                if let Tile::Num(num) = sudoku.board[x][y] {
                    counts[num - 1] += 1;
                    if counts[num - 1] > 1 {
                        return false;
                    }
                }
            }
        }

        for y in 0..9 {
            let mut counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
            for x in 0..9 {
                if let Tile::Num(num) = sudoku.board[x][y] {
                    counts[num - 1] += 1;
                    if counts[num - 1] > 1 {
                        return false;
                    }
                }
            }
        }

        for box_x in (0..9).step_by(3) {
            for box_y in (0..9).step_by(3) {
                let mut counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
                for dx in 0..3 {
                    for dy in 0..3 {
                        if let Tile::Num(num) = sudoku.board[box_x + dx][box_y + dy] {
                            counts[num - 1] += 1;
                            if counts[num - 1] > 1 {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        return true;
    }
}


