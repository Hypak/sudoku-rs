use crate::sudoku::*;

impl Sudoku {
    fn get_naked_single(&mut self, x_pos: usize, y_pos: usize) ->  (Tile, bool) {
        if self.board[x_pos][y_pos] != Tile::Void {
            return (Tile::Void, false);
        }
        let mut res = Tile::Void;
        for i in 1..=9 {
            if self.is_possible_at(x_pos, y_pos, i) {
                if res == Tile::Void { res = Tile::Num(i) }
                else { return (Tile::Void, false) }
            }
        }
        (res, res == Tile::Void)
    }
    fn get_naked_pair(&mut self, x_pos: usize, y_pos: usize) -> (Tile, Tile) {
        let mut first = Tile::Void;
        let mut second = Tile::Void;
        for i in 1..=9 {
            if self.is_possible_at(x_pos, y_pos, i) {
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
    fn get_best_guess_spot(&mut self, debug: bool) -> (usize, usize) {
        let mut best_x = 0;
        let mut best_y = 0;
        let mut min_possible = usize::MAX;
        for x in 0..9 {
            for y in 0..9 {
                let mut count = 0;
                for i in 1..=9 {
                    if self.is_possible_at(x, y, i) {
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

    pub fn fill_naked_singles(&mut self, debug: bool) -> (bool, bool, bool) {
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
                        // println!("naked single");
                        // self.print_with_possibilities();
                    }
                    changed = true;
                }
            }
        }
        (changed, complete, false)
    }

    pub fn fill_last_in_column(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for x in 0..9 {
            for val in 1..=9 {
                let mut possible_count = 0;
                let mut possible_at = None;

                let mut column_possible = self.column_possible[x][val - 1];
                for y in 0..9 {
                    if column_possible % 2 == 0b1 {
                        possible_count += 1;
                        possible_at = Some(y);
                        if possible_count > 1 {
                            break;
                        }
                    }
                    column_possible >>= 1;
                }

                if possible_count == 1 {
                    self.set_tile_at(x, possible_at.expect("pls"), Tile::Num(val));
                    if debug {
                        // println!("last column");
                        // self.print_with_possibilities();
                    }
                    changed = true;
                }
            }
        }
        changed
    }

    pub fn fill_last_in_row(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for y in 0..9 {
            for val in 1..=9 {
                let mut possible_count = 0;
                let mut possible_at = None;

                let mut row_possible = self.row_possible[y][val - 1];
                for x in 0..9 {
                    if row_possible % 2 == 0b1 {
                        possible_count += 1;
                        possible_at = Some(x);
                        if possible_count > 1 {
                            break;
                        }
                    }
                    row_possible >>= 1;
                }

                if possible_count == 1 {
                    self.set_tile_at(possible_at.expect("pls"), y, Tile::Num(val));
                    if debug {
                        // println!("last row");
                        // self.print_with_possibilities();
                    }
                    changed = true;
                }
            }
        }
        changed
    }

    pub fn fill_last_in_box(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for box_x in (0..9).step_by(3) {
            for box_y in (0..9).step_by(3) {
                for val in 1..=9 {
                    let mut possible_count = 0;
                    let mut possible_at_dx = None;
                    let mut possible_at_dy = None;

                    let box_index = box_x / 3 + 3 * (box_y / 3);
                    let mut box_possible = self.box_possible[box_index][val - 1];
                    for index_in_box in 0..9 {
                        if box_possible % 2 == 0b1 {
                            possible_count += 1;
                            possible_at_dx = Some(index_in_box % 3);
                            possible_at_dy = Some(index_in_box / 3);
                            if possible_count > 1 {
                                break;
                            }
                        }
                        box_possible >>= 1;
                    }

                    if possible_count == 1 {
                        let x = box_x + possible_at_dx.expect("pls");
                        let y = box_y + possible_at_dy.expect("pls");
                        self.set_tile_at(x, y, Tile::Num(val));
                        if debug {
                            // println!("last box");
                            // self.print_with_possibilities();
                        }
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_pairs_columns(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for x in 0..9 {
            for num_first in 1..=8 {
                for num_second in (num_first + 1)..=9 {
                    if self.column_possible[x][num_first - 1] == self.column_possible[x][num_second - 1] {
                        // test if only two possibilities
                        let mut possible = self.column_possible[x][num_first - 1];
                        let mut first_y = None;
                        let mut second_y = None;
                        for y in 0..9 {
                            if possible % 2 == 0b1 {
                                if first_y == None {
                                    first_y = Some(y);
                                } else if second_y == None {
                                    second_y = Some(y);
                                } else {
                                    first_y = None;
                                    second_y = None;
                                    break;
                                }
                            }
                            possible >>= 1;
                        }
                        if let Some(y0) = first_y {
                            if let Some(y1) = second_y {
                                // Remove other possibilities
                                for val in 1..=9 {
                                    if val == num_first || val == num_second {
                                        continue;
                                    }
                                    let old_y0 = self.possible[x][y0];
                                    let old_y1 = self.possible[x][y1];
                                    self.remove_possible_at(x, y0, val);
                                    self.remove_possible_at(x, y1, val);
                                    if old_y0 != self.possible[x][y0] || old_y1 != self.possible[x][y1] {
                                        changed = true;
                                    }
                                }
                            }
                        }

                        // if three are the same, we know that there can't just be two possibilities
                        // therefore we can break to save time
                        break;
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_pairs_rows(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for y in 0..9 {
            for num_first in 1..=8 {
                for num_second in (num_first + 1)..=9 {
                    if self.row_possible[y][num_first - 1] == self.row_possible[y][num_second - 1] {
                        // test if only two possibilities
                        let mut possible = self.row_possible[y][num_first - 1];
                        let mut first_x = None;
                        let mut second_x = None;
                        for x in 0..9 {
                            if possible % 2 == 0b1 {
                                if first_x == None {
                                    first_x = Some(x);
                                } else if second_x == None {
                                    second_x = Some(x);
                                } else {
                                    first_x = None;
                                    second_x = None;
                                    break;
                                }
                            }
                            possible >>= 1;
                        }
                        if let Some(x0) = first_x {
                            if let Some(x1) = second_x {
                                // Remove other possibilities
                                for val in 1..=9 {
                                    if val == num_first || val == num_second {
                                        continue;
                                    }
                                    let old_x0 = self.possible[x0][y];
                                    let old_x1 = self.possible[x1][y];
                                    self.remove_possible_at(x0, y, val);
                                    self.remove_possible_at(x1, y, val);
                                    if old_x0 != self.possible[x0][y] || old_x1 != self.possible[x1][y] {
                                        changed = true;
                                    }
                                }
                            }
                        }

                        // if three are the same, we know that there can't just be two possibilities
                        // therefore we can break to save time
                        break;
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_pairs_boxes(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for box_num in 0..9 {
            for num_first in 1..=8 {
                for num_second in (num_first + 1)..=9 {
                    if self.box_possible[box_num][num_first - 1] == self.box_possible[box_num][num_second - 1] {
                        // test if only two possibilities
                        let mut possible = self.box_possible[box_num][num_first - 1];
                        let mut first_i = None;
                        let mut second_i = None;
                        for box_i in 0..9 {
                            if possible % 2 == 0b1 {
                                if first_i == None {
                                    first_i = Some(box_i);
                                } else if second_i == None {
                                    second_i = Some(box_i);
                                } else {
                                    first_i = None;
                                    second_i = None;
                                    break;
                                }
                            }
                            possible >>= 1;
                        }
                        if let Some(i0) = first_i {
                            if let Some(i1) = second_i {
                                // Remove other possibilities
                                for val in 1..=9 {
                                    if val == num_first || val == num_second {
                                        continue;
                                    }
                                    let box_x = 3 * (box_num % 3);
                                    let box_y = 3 * (box_num / 3);
                                    let x0 = box_x + i0 % 3;
                                    let y0 = box_y + i0 / 3;
                                    let x1 = box_x + i1 % 3;
                                    let y1 = box_y + i1 / 3;

                                    let old_i0 = self.possible[x0][y0];
                                    let old_i1 = self.possible[x1][y1];
                                    self.remove_possible_at(x0, y0, val);
                                    self.remove_possible_at(x1, y1, val);
                                    if old_i0 != self.possible[x0][y0] || old_i1 != self.possible[x1][y1] {
                                        changed = true;
                                    }
                                }
                            }
                        }

                        // if three are the same, we know that there can't just be two possibilities
                        // therefore we can break to save time
                        break;
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_trips_columns(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for x in 0..9 {
            for num_first in 1..=7 {
                let first_poss = self.column_possible[x][num_first - 1];
                if first_poss == NONE_POSSIBLE {
                    continue;
                }
                for num_second in (num_first + 1)..=8 {
                    let second_poss = self.column_possible[x][num_second - 1];
                    if second_poss == NONE_POSSIBLE {
                        continue;
                    }
                    for num_third in (num_second + 1)..=9 {
                        let third_poss = self.column_possible[x][num_third - 1];
                        if third_poss == NONE_POSSIBLE {
                            continue;
                        }
                        let mut intersection = first_poss | second_poss | third_poss;

                        let mut first_y = None;
                        let mut second_y = None;
                        let mut third_y = None;
                        for y in 0..9 {
                            if intersection % 2 == 0b1 {
                                if first_y == None {
                                    first_y = Some(y);
                                } else if second_y == None {
                                    second_y = Some(y);
                                } else if third_y == None {
                                    third_y = Some(y);
                                } else {
                                    first_y = None;
                                    second_y = None;
                                    third_y = None;
                                    break;
                                }
                            }
                            intersection >>= 1;
                        }
                        if let Some(y0) = first_y {
                            if let Some(y1) = second_y {
                                if let Some(y2) = third_y {
                                    // Remove other possibilities
                                    let old_y0 = self.possible[x][y0];
                                    let old_y1 = self.possible[x][y1];
                                    let old_y2 = self.possible[x][y2];

                                    for val in 1..=9 {
                                        if val == num_first || val == num_second || val == num_third {
                                            continue;
                                        }
                                        self.remove_possible_at(x, y0, val);
                                        self.remove_possible_at(x, y1, val);
                                        self.remove_possible_at(x, y2, val);
                                    }
                                    if old_y0 != self.possible[x][y0] || old_y1 != self.possible[x][y1] || old_y2 != self.possible[x][y2] {
                                        changed = true;
                                        if debug {
                                            println!("trips in column: {}", x);
                                            self.print_with_possibilities();
                                            println!();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_trips_rows(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for y in 0..9 {
            for num_first in 1..=7 {
                let first_poss = self.row_possible[y][num_first - 1];
                if first_poss == NONE_POSSIBLE {
                    continue;
                }
                for num_second in (num_first + 1)..=8 {
                    let second_poss = self.row_possible[y][num_second - 1];
                    if second_poss == NONE_POSSIBLE {
                        continue;
                    }
                    for num_third in (num_second + 1)..=9 {
                        let third_poss = self.row_possible[y][num_third - 1];
                        if third_poss == NONE_POSSIBLE {
                            continue;
                        }
                        let mut intersection = first_poss | second_poss | third_poss;

                        let mut first_x = None;
                        let mut second_x = None;
                        let mut third_x = None;
                        for x in 0..9 {
                            if intersection % 2 == 0b1 {
                                if first_x == None {
                                    first_x = Some(x);
                                } else if second_x == None {
                                    second_x = Some(x);
                                } else if third_x == None {
                                    third_x = Some(x);
                                } else {
                                    first_x = None;
                                    second_x = None;
                                    third_x = None;
                                    break;
                                }
                            }
                            intersection >>= 1;
                        }
                        if let Some(x0) = first_x {
                            if let Some(x1) = second_x {
                                if let Some(x2) = third_x {
                                    // Remove other possibilities
                                    let old_x0 = self.possible[x0][y];
                                    let old_x1 = self.possible[x1][y];
                                    let old_x2 = self.possible[x2][y];

                                    for val in 1..=9 {
                                        if val == num_first || val == num_second || val == num_third {
                                            continue;
                                        }
                                        self.remove_possible_at(x0, y, val);
                                        self.remove_possible_at(x1, y, val);
                                        self.remove_possible_at(x2, y, val);
                                    }
                                    if old_x0 != self.possible[x0][y] || old_x1 != self.possible[x1][y] || old_x2 != self.possible[x2][y] {
                                        changed = true;
                                        if debug {
                                            println!("trips in row: {}", y);
                                            self.print_with_possibilities();
                                            println!();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    fn apply_hidden_trips_boxes(&mut self, debug: bool) -> bool {
        let mut changed = false;
        for box_num in 0..9 {
            for num_first in 1..=7 {
                let first_poss = self.box_possible[box_num][num_first - 1];
                if first_poss == NONE_POSSIBLE {
                    continue;
                }
                for num_second in (num_first + 1)..=8 {
                    let second_poss = self.box_possible[box_num][num_second - 1];
                    if second_poss == NONE_POSSIBLE {
                        continue;
                    }
                    for num_third in (num_second + 1)..=9 {
                        let third_poss = self.box_possible[box_num][num_third - 1];
                        if third_poss == NONE_POSSIBLE {
                            continue;
                        }
                        let mut intersection = first_poss | second_poss | third_poss;

                        let mut first_i = None;
                        let mut second_i = None;
                        let mut third_i = None;
                        for box_i in 0..9 {
                            if intersection % 2 == 0b1 {
                                if first_i == None {
                                    first_i = Some(box_i);
                                } else if second_i == None {
                                    second_i = Some(box_i);
                                } else if third_i == None {
                                    third_i = Some(box_i);
                                } else {
                                    first_i = None;
                                    second_i = None;
                                    third_i = None;
                                    break;
                                }
                            }
                            intersection >>= 1;
                        }
                        if let Some(i0) = first_i {
                            if let Some(i1) = second_i {
                                if let Some(i2) = third_i {
                                    let box_x = 3 * (box_num % 3);
                                    let box_y = 3 * (box_num / 3);
                                    let x0 = box_x + i0 % 3;
                                    let y0 = box_y + i0 / 3;
                                    let x1 = box_x + i1 % 3;
                                    let y1 = box_y + i1 / 3;
                                    let x2 = box_x + i2 % 3;
                                    let y2 = box_y + i2 / 3;
                                    // Remove other possibilities
                                    let old_x0 = self.possible[x0][y0];
                                    let old_x1 = self.possible[x1][y2];
                                    let old_x2 = self.possible[x2][y2];

                                    for val in 1..=9 {
                                        if val == num_first || val == num_second || val == num_third {
                                            continue;
                                        }
                                        self.remove_possible_at(x0, y0, val);
                                        self.remove_possible_at(x1, y1, val);
                                        self.remove_possible_at(x2, y2, val);
                                    }
                                    if old_x0 != self.possible[x0][y0] || old_x1 != self.possible[x1][y1] || old_x2 != self.possible[x2][y2] {
                                        changed = true;
                                        if debug {
                                            println!("trips in box: {} with nums: ({}, {}, {}) i: ({}, {}, {})", box_num, num_first, num_second, num_third, i0, i1, i2);
                                            self.print_with_possibilities();
                                            println!();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        changed
    }

    fn apply_naked_pairs(&mut self, debug: bool) -> bool {
        let mut changed = false;
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
                                        if self.is_possible_at(x, y_2, pair_a) || self.is_possible_at(x, y_2, pair_b) {
                                            changed = true;
                                        }
                                        self.remove_possible_at(x, y_2, pair_a);
                                        self.remove_possible_at(x, y_2, pair_b);
                                    }
                                }
                                // if in the same row (we can use else as the coords are different)
                                else if y == other_y {
                                    for x_2 in 0..9 {
                                        if x_2 == x || x_2 == other_x {
                                            continue;
                                        }
                                        // remove the possibilities from others in the row
                                        if self.is_possible_at(x_2, y, pair_a) || self.is_possible_at(x_2, y, pair_b) {
                                            changed = true;
                                        }
                                        self.remove_possible_at(x_2, y, pair_a);
                                        self.remove_possible_at(x_2, y, pair_b);
                                        if debug && changed {
                                            println!("naked pairing ({}, {})", x_2, y);
                                            // self.print_with_possibilities();
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
                                            if self.is_possible_at(remove_x, remove_y, pair_a) || self.is_possible_at(remove_x, remove_y, pair_b) {
                                                changed = true;
                                            }
                                            self.remove_possible_at(remove_x, remove_y, pair_a);
                                            self.remove_possible_at(remove_x, remove_y, pair_b);
                                            if debug && changed {
                                                println!("naked pairing ({}, {})", remove_x, remove_y);
                                                // self.print_with_possibilities();
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
        changed
    }

    pub fn solve_no_guessing(&mut self, only_basic: bool, iter_count: usize, debug: bool) -> SolutionCount {
        for _ in 0..iter_count {
            let (mut changed, complete, no_solutions) = self.fill_naked_singles(debug);
            if no_solutions {
                return SolutionCount::Zero;
            }
            if complete {
                return SolutionCount::One(*self);
            }
            if changed {
                continue;
            }

            // Last remaining in columns, rows, boxes
            changed |= self.fill_last_in_column(debug);
            changed |= self.fill_last_in_row(debug);
            changed |= self.fill_last_in_box(debug);

            if only_basic {
                if !changed {
                    break;
                }
                continue;
            }

            if changed {
                continue;
            }

            // naked pairs (this can slow down the performance)
            changed |= self.apply_naked_pairs(debug);
            if changed {
                continue;
            }

            // hidden pairs/trips (unknown effect on performance)
            changed |= self.apply_hidden_pairs_columns(debug);
            changed |= self.apply_hidden_pairs_rows(debug);
            changed |= self.apply_hidden_pairs_boxes(debug);
            if changed {
                continue;
            }

            changed |= self.apply_hidden_trips_columns(debug);
            changed |= self.apply_hidden_trips_rows(debug);
            changed |= self.apply_hidden_trips_boxes(debug);

            if !changed {
                break;
            }
        }
        return SolutionCount::Zero;
    }

    pub fn solve(&mut self, debug: bool) -> SolutionCount {
        for _ in 0..100 {
            let (mut changed, complete, no_solutions) = self.fill_naked_singles(debug);
            if no_solutions {
                return SolutionCount::Zero;
            }
            if complete {
                return SolutionCount::One(*self);
            }
            if changed {
                continue;
            }

            // Last remaining in columns, rows, boxes
            changed |= self.fill_last_in_column(debug);
            changed |= self.fill_last_in_row(debug);
            changed |= self.fill_last_in_box(debug);

            // skip naked pairs until we run out of easy changes
            if changed {
                continue;
            }

            // naked pairs (this can slow down the performance)
            changed |= self.apply_naked_pairs(debug);

            if changed {
                continue;
            }

            // hidden pairs (unknown effect on performance)

            changed |= self.apply_hidden_pairs_columns(debug);
            changed |= self.apply_hidden_pairs_rows(debug);
            changed |= self.apply_hidden_pairs_boxes(debug);

            if changed {
                continue;
            }

            // hidden trips (unlikely to help much; improves 0-guess solve count from 34115 to 34242 (with col), to 34359 (with col and row), to 34393 (with col, row, box) out of 49151)
            // slows down performance of even the hardest 17-tile puzzles
            changed |= self.apply_hidden_trips_columns(debug);
            changed |= self.apply_hidden_trips_rows(debug);
            changed |= self.apply_hidden_trips_boxes(debug);

            if !changed {
                break;
            }
        }
        let (best_x, best_y) = self.get_best_guess_spot(debug);
        let mut solution = None;
        if debug {
            println!("Making choice point");
            println!("{}", self);
            self.print_sudoku_wiki_link();
            println!("Guessing at {}, {}", best_x, best_y);
        }
        for i in 1..=9 {
            if self.is_possible_at(best_x, best_y, i) {
                let mut new_sudoku = Self::from_sudoku(self);
                new_sudoku.set_tile_at(best_x, best_y, Tile::Num(i));
                let solution_count = new_sudoku.solve(debug);
                if debug {
                    println!("Guess: {}", i);
                    println!("{:?}", solution_count);
                }
                match solution_count {
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
        if debug {
            println!("{}", self);
        }
        match solution {
            None => { return SolutionCount::Zero }
            Some(solution) => { return SolutionCount::One(solution)}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sudoku::*;
    use crate::sudoku::solve::*;
    use crate::reader::*;

    #[test]
    fn test_get_naked_single() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/get_naked_single.csv");
        let output = all_sudoku[0].get_naked_single(0, 0);
        assert_eq!(output, (Tile::Num(1), false));
        let output = all_sudoku[1].get_naked_single(0, 0);
        assert_eq!(output, (Tile::Void, false));

    }

    #[test]
    fn test_get_naked_pair() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/get_naked_pair.csv");
        let output = all_sudoku[0].get_naked_pair(0, 0);
        assert_eq!(output, (Tile::Num(1), Tile::Num(8)));
        let output = all_sudoku[1].get_naked_pair(0, 0);
        assert_eq!(output, (Tile::Void, Tile::Void));
    }

    #[test]
    fn test_get_best_guess_spot() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/get_naked_pair.csv");
        let output = all_sudoku[0].get_best_guess_spot(false);
        assert_eq!(output, (0, 0));
    }

    #[test]
    fn test_fill_naked_singles() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/fill_naked_singles.csv");
        let output = all_sudoku[0].fill_naked_singles(false);
        assert_eq!(all_sudoku[0].board, all_sudoku[1].board);
    }

    #[test]
    fn test_fill_last_in_column() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/fill_last_in_column.csv");
        let output = all_sudoku[0].fill_last_in_column(false);
        assert_eq!(all_sudoku[0].board, all_sudoku[1].board);
    }

    #[test]
    fn test_fill_last_in_row() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/fill_last_in_row.csv");
        let output = all_sudoku[0].fill_last_in_row(false);
        assert_eq!(all_sudoku[0].board, all_sudoku[1].board);
    }

    #[test]
    fn test_fill_last_in_box() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/fill_last_in_box.csv");
        let output = all_sudoku[0].fill_last_in_box(false);
        assert_eq!(all_sudoku[0].board, all_sudoku[1].board);
    }

    #[test]
    fn test_hidden_pairs() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/hidden_pairs.csv");
        let output = all_sudoku[0].apply_hidden_pairs_rows(false);
        let correct = (0b1 << (6 - 1)) + (0b1 << (7 - 1));
        assert_eq!(all_sudoku[0].possible[7][0], correct);
        assert_eq!(all_sudoku[0].possible[8][0], correct);
    }

    #[test]
    fn test_hidden_trips() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/hidden_trips.csv");
        let output = all_sudoku[0].apply_hidden_trips_rows(false);
        let correct_256 = (0b1 << (2 - 1)) + (0b1 << (5 - 1)) + (0b1 << (6 - 1));
        let correct_26 = (0b1 << (2 - 1)) + (0b1 << (6 - 1));
        let correct_25 = (0b1 << (2 - 1)) + (0b1 << (5 - 1));
        assert_eq!(all_sudoku[0].possible[3][0], correct_256);
        assert_eq!(all_sudoku[0].possible[6][0], correct_26);
        assert_eq!(all_sudoku[0].possible[8][0], correct_25);
    }

        #[test]
    fn test_naked_pairs() {
        let mut all_sudoku = get_all_sudoku_from_path("data/test/naked_pairs.csv");
        let output = all_sudoku[0].apply_naked_pairs(false);
        let correct_25 = (0b1 << (2 - 1)) + (0b1 << (5 - 1));
        let correct_257 = (0b1 << (2 - 1)) + (0b1 << (5 - 1)) + (0b1 << (7 - 1));
        assert_eq!(all_sudoku[0].possible[3][0], correct_25);
        assert_eq!(all_sudoku[0].possible[4][0], correct_257);
        assert_eq!(all_sudoku[0].possible[5][0], correct_257);
    }
}
