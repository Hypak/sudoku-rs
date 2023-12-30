use std::fs;

use crate::sudoku::*;

pub fn get_first_sudoku_from_path(path: &str) -> Option<Sudoku>  {
    let file_path = path;
    let binding = fs::read_to_string(file_path).expect("pls work");
    let mut lines = binding.lines().clone();

    // first line is "sudoku" or "*,sudoku,*"
    let names = lines.next().expect("not None").split(',').collect::<Vec<_>>();
    let sudoku_index = names.iter().position(|&r| r == "sudoku").expect("pls work");
    let difficulty_index = names.iter().position(|&r| r == "difficulty");

    let next_line = lines.next();
    if next_line == None {
        return None;
    }
    let split = next_line.expect("not None").split(',').collect::<Vec<_>>();
    let string = split[sudoku_index];
    let sudoku = Sudoku::from_string(string);
    return Some(sudoku);
}

    pub fn get_all_sudoku_from_path(path: &str) -> Vec<Sudoku>  {
    let file_path = path;
    let binding = fs::read_to_string(file_path).expect("pls work");
    let mut lines = binding.lines().clone();

    // first line is "sudoku" or "*,sudoku,*"
    let names = lines.next().expect("not None").split(',').collect::<Vec<_>>();
    let sudoku_index = names.iter().position(|&r| r == "sudoku").expect("pls work");
    let difficulty_index = names.iter().position(|&r| r == "difficulty");

    let mut res = vec![];
    loop {
        let next_line = lines.next();
        if next_line == None {
            return res;
        }
        let split = next_line.expect("not None").split(',').collect::<Vec<_>>();
        let string = split[sudoku_index];
        let sudoku = Sudoku::from_string(string);
        res.push(sudoku);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_in_sudoku_correctly() {
        let sudoku = get_first_sudoku_from_path("data/sudoku17.csv");
        assert_eq!(sudoku.unwrap().clues, 17);
    }

    #[test]
    fn reads_in_sudoku_correctly_when_there_are_lots_of_columns() {
        let sudoku = get_first_sudoku_from_path("data/sudoku-rated.csv");
        assert_eq!(sudoku.unwrap().clues, 27);
    }

    #[test]
    fn reads_in_correct_count() {
        let sudoku = get_all_sudoku_from_path("data/sudoku17.csv");
        assert_eq!(sudoku.len(), 49151);
    }
}


/*
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
*/
