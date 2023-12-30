fn test() {
let mut column_possible = self.column_possible[x][val - 1];
for y in 0..9 {
    if column_possible & 0b1 == 0b1 {
        possible_count += 1;
        possible_at = Some(y);
        if possible_count > 1 {
            break;
        }
    }
    column_possible >>= 1;
}
}
