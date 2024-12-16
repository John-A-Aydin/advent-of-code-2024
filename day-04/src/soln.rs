pub fn solve_p1(data: &str) -> i32 {
    // Parse into matrix of chars
    let mat: Vec<Vec<char>> = data
        .split('\n')
        .map(|line| {
            return line.chars().collect();
        })
        .collect();

    let ans = mat.iter().enumerate().fold(0, |acc_y: i32, (i, row)| {
        return acc_y
            + row.iter().enumerate().fold(0, |acc_x, (j, _)| {
                return acc_x + check_from_start(i, j, &mat);
            });
    });

    return ans;
}

fn check_from_start(i: usize, j: usize, mat: &Vec<Vec<char>>) -> i32 {
    let xmas = ['X', 'M', 'A', 'S'];

    let start = get_nested_value(mat, i, j);

    match start {
        Some(s) => {
            if *s != 'X' {
                return 0;
            }
        }
        None => return 0,
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
                let x = get_nested_value(mat, row_idx as usize, col_idx as usize);
                if x.is_none() {
                    break;
                }
                if *x.unwrap() != xmas[char_idx as usize] {
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

fn get_nested_value<T>(m: &Vec<Vec<T>>, row: usize, col: usize) -> Option<&T> {
    m.get(row)?.get(col)
}
