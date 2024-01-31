use crate::sudoku::*;

impl Sudoku {
    pub fn print_sudoku_wiki_link(&mut self) {
        let base = "https://www.sudokuwiki.org/sudoku.htm?bd=";
        print!("{}", base);
        for y in 0..9 {
            for x in 0..9 {
                match self.board[x][y] {
                    Tile::Void => { print!("0") },
                    Tile::Num(x) => { print!("{}", x) },
                }
            }
        }
        println!();
    }

    pub fn print_with_possibilities(&mut self) {
        println!("-------------------------------------");
        for j in 0..9 {
            for repeat in 0..3 {
                print!("|");
                for i in 0..9 {
                    if self.board[i][j] == Tile::Void {
                        // print possibilities
                        for k in 1..=3 {
                            let index = 3 * repeat + k;
                            if self.is_possible_at(i, j, index) {
                                print!("{}", index);
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
        for x in 0..9 {
            for val in 1..=9 {
                // println!("Row possibilities at (row: {}, num: {}) are {:b}", x, val, self.row_possible[x][val - 1]);
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
