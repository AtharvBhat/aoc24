use std::fs;
mod utils;
fn main() {
    let letter_matrix: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("File not found")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    //part 1
    let mut count: usize = 0;
    for y in 0..letter_matrix.len() {
        for x in 0..letter_matrix[0].len() {
            if letter_matrix[y][x] == 'X' {
                count += utils::look_up(x, y, &letter_matrix);
                count += utils::look_down(x, y, &letter_matrix);
                count += utils::look_left(x, y, &letter_matrix);
                count += utils::look_right(x, y, &letter_matrix);
                count += utils::look_top_left(x, y, &letter_matrix);
                count += utils::look_top_right(x, y, &letter_matrix);
                count += utils::look_bottom_left(x, y, &letter_matrix);
                count += utils::look_bottom_right(x, y, &letter_matrix)
            }
        }
    }

    println!("Part 1: {count}");

    //part 2
    let mut count: usize = 0;
    for y in 0..letter_matrix.len() {
        for x in 0..letter_matrix[0].len() {
            if letter_matrix[y][x] == 'A' {
                if utils::check_tl_br_diag(x, y, &letter_matrix)
                    && utils::check_tr_bl_diag(x, y, &letter_matrix)
                {
                    count += 1;
                }
            }
        }
    }

    println!("Part 2: {count}")
}
