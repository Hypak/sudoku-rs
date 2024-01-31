#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod sudoku;
pub mod reader;

use std::env;
use std::fs;
use std::time::Instant;
use std::collections::BTreeSet;
use std::collections::HashMap;
use itertools::Itertools;


use crate::sudoku::*;
use crate::sudoku::solve::*;
use crate::reader::*;

fn main() {
    test_first_sudoku("data/sudoku17.csv", 23);
    // test_every_sudoku();
    // test_no_guessing();
    // test_gen();
}

fn print_every_difficulty() {

}

fn test_gen() {
    let mut sudoku = reader::get_first_sudoku_from_path("data/sudoku17.csv").expect("pls work");
    sudoku.solve(false);
    println!("{}", sudoku);

    let mut generated_queue = vec![];
    let mut generated_set = BTreeSet::new();
    let clues = 28;
    for attempt in 0..1000 {
        let reduced = sudoku.reduce_to_n_random(clues);
        let mut to_solve = Sudoku::from_sudoku(&reduced);
        let solution_count = to_solve.solve(false);
        if let SolutionCount::One(_) = solution_count {
            println!("Found a with {} clues on attempt {}/1000", clues, attempt);
            generated_queue.push(reduced);
            generated_set.insert(reduced);
            break;
        }
    }
    while let Some(sudoku) = generated_queue.pop() {
        let reduced = sudoku.get_all_reduced_by_one_clue();
        if reduced.len() == 0 {
            continue;
        }
        let len = reduced.len();
        let clue_count = reduced[0].clues;
        let mut unseen_count = 0;
        for r in reduced {
            if generated_set.contains(&r) {
                continue;
            }
            // Insert all into the head
            generated_queue.push(r);
            generated_set.insert(r);
            unseen_count += 1;
        }
        if len > 0 {
            for _ in 0..clue_count {
                print!(" ");
            }
            if unseen_count == len {
                println!("w00t we reduced to get {} puzzles with {} clues each", len, clue_count);
            } else {
                println!("w00t we reduced to get {} puzzles ({} unseen) with {} clues each", len, unseen_count, clue_count);
            }
        }
    }
    let mut generated_counts = HashMap::new();
    for sudoku in &generated_set {
        if generated_counts.contains_key(&sudoku.clues) {
            generated_counts.insert(sudoku.clues, generated_counts.get(&sudoku.clues).expect("we tested") + 1);
        } else {
            generated_counts.insert(sudoku.clues, 1);
        }
    }
    let smallest_key = generated_counts.keys().min();
    for mut sudoku in generated_set {
        if sudoku.clues != *smallest_key.expect("pls") {
            continue;
        }
        println!("{}", sudoku);
        println!("{:?} difficulty", sudoku.get_difficulty(3));
        sudoku.print_sudoku_wiki_link();
    }
    println!("These sudoku have {} clues", smallest_key.expect("pls"));
    println!("{:?}", generated_counts);
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

fn test_first_sudoku(path: &str, index: usize) {
    let all_sudoku = reader::get_all_sudoku_from_path(path);
    let mut sudoku = all_sudoku[index];
    sudoku.print_with_possibilities();
    println!("{}", sudoku);
    sudoku.print_sudoku_wiki_link();
    println!("Difficulty: {:?}", sudoku.get_difficulty(2));
    let solutions = sudoku.solve(false);
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
                panic!("Solver found wrong number of solutions at index {}", index);
            }
            index += 1;
            if index >= 100 {
                // Limits solving to 100 Sudoku for time reasons
                return;
            }
        }
    }
    #[test]
    fn test_multiple() {
        let sudoku_puzzles = get_all_sudoku_from_path("data/multiple.csv");
        let mut index = 0;
        for mut sudoku in sudoku_puzzles {
            if let SolutionCount::Multiple = sudoku.solve(false) {
                // good
            } else {
                panic!("Solver found wrong number of solutions");
            }
            index += 1;
            if index >= 100 {
                // Limits solving to 100 Sudoku for time reasons
                return;
            }
        }    }

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


