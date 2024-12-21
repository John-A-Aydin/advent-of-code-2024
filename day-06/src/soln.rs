pub fn solve_p1(data: &str) -> u32 {
    // Converting to char matrix
    let mut mat = str_to_matrix(data);

    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;
    let mut found = false;

    // Finding the guard
    for (y, row) in mat.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                '>' | '<' | '^' | 'v' => {
                    guard_x = x;
                    guard_y = y;
                    found = true;
                    break;
                }
                _ => {
                    continue;
                }
            }
            // Might be dead code, but I dont know how Rust handles breaks.
            // if found {
            //     break;
            // }
        }
        if found {
            break;
        }
    }

    // No guard 0_0
    if !found {
        return 0;
    }

    //println!("{}", mat[guard_])

    // We have the guard position, now we start making his moves
    let mut unique: u32 = 1;
    loop {
        // Don't need options bc this is guarenteed to exist
        let guard_direction = get_nested_value(&mat, guard_y, guard_x).unwrap();

        // Get next position
        let mut new_x = guard_x;
        let mut new_y = guard_y;

        match *guard_direction {
            '^' => new_y -= 1,
            'v' => new_y += 1,
            '<' => new_x -= 1,
            '>' => new_x += 1,
            _ => panic!("Bad state: {}", unique), // Bad program state
        }

        let new_position_option = get_nested_value(&mat, new_y, new_x);

        // Out of map
        if new_position_option.is_none() {
            return unique;
        }

        let new_position = new_position_option.unwrap();

        match *new_position {
            'X' => {
                mat[new_y][new_x] = *guard_direction;
                mat[guard_y][guard_x] = 'X';
                guard_x = new_x;
                guard_y = new_y;
            }
            '#' => match *guard_direction {
                '^' => mat[guard_y][guard_x] = '>',
                '>' => mat[guard_y][guard_x] = 'v',
                'v' => mat[guard_y][guard_x] = '<',
                '<' => mat[guard_y][guard_x] = '^',
                _ => panic!("bad state"),
            },
            '.' => {
                mat[new_y][new_x] = *guard_direction;
                mat[guard_y][guard_x] = 'X';
                guard_x = new_x;
                guard_y = new_y;
                unique += 1;
            }
            _ => {
                panic!("logic error")
            }
        }
    }
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
    return m.get(row)?.get(col);
}
