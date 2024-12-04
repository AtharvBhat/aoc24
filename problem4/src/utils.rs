// Part 1
//Verical checks
pub fn look_up(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if y < 3 {
        return 0;
    }
    if mat[y - 1][x] == 'M' && mat[y - 2][x] == 'A' && mat[y - 3][x] == 'S' {
        return 1;
    }
    0
}
pub fn look_down(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if (y + 3) > (mat.len() - 1) {
        return 0;
    }
    if mat[y + 1][x] == 'M' && mat[y + 2][x] == 'A' && mat[y + 3][x] == 'S' {
        return 1;
    }
    0
}

//horizontal checks
pub fn look_left(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if x < 3 {
        return 0;
    }
    if mat[y][x - 1] == 'M' && mat[y][x - 2] == 'A' && mat[y][x - 3] == 'S' {
        return 1;
    }
    0
}
pub fn look_right(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if (x + 3) > (mat[0].len() - 1) {
        return 0;
    }
    if mat[y][x + 1] == 'M' && mat[y][x + 2] == 'A' && mat[y][x + 3] == 'S' {
        return 1;
    }
    0
}

//diagonal checks
pub fn look_top_left(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if y < 3 || x < 3 {
        return 0;
    }
    if mat[y - 1][x - 1] == 'M' && mat[y - 2][x - 2] == 'A' && mat[y - 3][x - 3] == 'S' {
        return 1;
    }
    0
}
pub fn look_top_right(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if y < 3 || (x + 3) > (mat[0].len() - 1) {
        return 0;
    }

    if mat[y - 1][x + 1] == 'M' && mat[y - 2][x + 2] == 'A' && mat[y - 3][x + 3] == 'S' {
        return 1;
    }
    0
}
pub fn look_bottom_left(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if (y + 3) > (mat.len() - 1) || x < 3 {
        return 0;
    }
    if mat[y + 1][x - 1] == 'M' && mat[y + 2][x - 2] == 'A' && mat[y + 3][x - 3] == 'S' {
        return 1;
    }
    0
}
pub fn look_bottom_right(x: usize, y: usize, mat: &Vec<Vec<char>>) -> usize {
    if (y + 3) > (mat.len() - 1) || (x + 3) > (mat[0].len() - 1) {
        return 0;
    }
    if mat[y + 1][x + 1] == 'M' && mat[y + 2][x + 2] == 'A' && mat[y + 3][x + 3] == 'S' {
        return 1;
    }
    0
}

// Part 2
pub fn check_tl_br_diag(x: usize, y: usize, mat: &Vec<Vec<char>>) -> bool {
    if (y < 1 || x < 1) || ((y + 1) > (mat.len() - 1) || (x + 1) > (mat[0].len() - 1)) {
        return false;
    }
    if (mat[y - 1][x - 1] == 'M') && (mat[y + 1][x + 1] == 'S')
        || (mat[y - 1][x - 1] == 'S') && (mat[y + 1][x + 1] == 'M')
    {
        return true;
    }
    false
}

pub fn check_tr_bl_diag(x: usize, y: usize, mat: &Vec<Vec<char>>) -> bool {
    if (y < 1 || (x + 1) > (mat[0].len() - 1)) || ((y + 1) > (mat.len() - 1) || x < 1) {
        return false;
    }
    if (mat[y - 1][x + 1] == 'M') && (mat[y + 1][x - 1] == 'S')
        || (mat[y - 1][x + 1] == 'S') && (mat[y + 1][x - 1] == 'M')
    {
        return true;
    }
    false
}
