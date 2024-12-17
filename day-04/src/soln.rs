pub fn solve_p1(data: &str) -> i32 {
    // Parse into matrix of chars
    let mat = str_to_matrix(data);

    let ans = mat.iter().enumerate().fold(0, |acc_y: i32, (i, row)| {
        return acc_y
            + row.iter().enumerate().fold(0, |acc_x, (j, _)| {
                return acc_x + check_from_start(i, j, &mat);
            });
    });

    return ans;
}

pub fn solve_p2(data: &str) -> i32 {
    let mat = str_to_matrix(data);
    let ans = mat.iter().enumerate().fold(0, |acc_y: i32, (i, row)| {
        return acc_y
            + row.iter().enumerate().fold(0, |acc_x, (j, _)| {
                return acc_x + check_from_middle(i, j, &mat);
            });
    });
    return ans;
}

fn str_to_matrix(data: &str) -> Vec<Vec<char>> {
    return data
        .split('\n')
        .map(|line| {
            return line.chars().collect();
        })
        .collect();
}

fn get_nested_value<T>(m: &Vec<Vec<T>>, row: usize, col: usize) -> Option<&T> {
    m.get(row)?.get(col)
}

fn check_from_start(i: usize, j: usize, mat: &Vec<Vec<char>>) -> i32 {
    let xmas = ['X', 'M', 'A', 'S'];

    if !check_nested_value(mat, i, j, 'X') {
        return 0;
    }

    let mut ans = 0;

    for y_move in -1..2 {
        for x_move in -1..2 {
            for char_idx in 1..4 {
                let row_idx: i32 = y_move * char_idx + i as i32;
                let col_idx: i32 = x_move * char_idx + j as i32;
                if row_idx < 0 || col_idx < 0 {
                    break;
                }
                if !check_nested_value(
                    mat,
                    row_idx as usize,
                    col_idx as usize,
                    xmas[char_idx as usize],
                ) {
                    break;
                }
                if char_idx == 3 {
                    ans += 1;
                }
            }
        }
    }

    return ans;
}

fn check_from_middle(i: usize, j: usize, mat: &Vec<Vec<char>>) -> i32 {
    // make sure i, j != 0
    if i < 1 || j < 1 {
        return 0;
    }
    if !check_nested_value(mat, i, j, 'A') {
        return 0;
    }

    // top left - bottom right check
    let tl_br = (check_nested_value(mat, i - 1, j - 1, 'M')
        && check_nested_value(mat, i + 1, j + 1, 'S'))
        || (check_nested_value(mat, i - 1, j - 1, 'S')
            && check_nested_value(mat, i + 1, j + 1, 'M'));

    // top right - bottom left check
    let tr_bl = (check_nested_value(mat, i + 1, j - 1, 'M')
        && check_nested_value(mat, i - 1, j + 1, 'S'))
        || (check_nested_value(mat, i + 1, j - 1, 'S')
            && check_nested_value(mat, i - 1, j + 1, 'M'));

    return (tl_br && tr_bl) as i32;
}

fn check_nested_value<T: std::cmp::PartialEq>(
    m: &Vec<Vec<T>>,
    row: usize,
    col: usize,
    target: T,
) -> bool {
    let val = get_nested_value(m, row, col);
    match val {
        Some(v) => {
            if *v == target {
                return true;
            }
            return false;
        }
        None => return false,
    }
}
