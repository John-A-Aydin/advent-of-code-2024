use std::collections::HashSet;

pub fn solve(data: &str) -> (u32, u32) {
    // Converting to char matrix
    let map = str_to_matrix(data);
    let mut p1_mat = map.clone();

    let mut starting_x: usize = 0;
    let mut starting_y: usize = 0;
    let mut found = false;

    // Finding the guard
    for (y, row) in p1_mat.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                '>' | '<' | '^' | 'v' => {
                    starting_x = x;
                    starting_y = y;
                    found = true;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
        if found {
            break;
        }
    }

    // No guard 0_0
    if !found {
        return (0, 0);
    }

    let mut guard_x = starting_x;
    let mut guard_y = starting_y;

    // We have the guard position, now we start making his moves
    let mut num_visited: u32 = 1;
    loop {
        // Don't need options bc this is guarenteed to exist
        let guard_direction = get_nested_value(&p1_mat, guard_y, guard_x).unwrap();

        // Get next position
        let mut new_x = guard_x;
        let mut new_y = guard_y;

        match guard_direction {
            '^' => new_y -= 1,
            'v' => new_y += 1,
            '<' => new_x -= 1,
            '>' => new_x += 1,
            _ => panic!("Bad state: {}", num_visited), // Bad program state
        }

        let new_position_option = get_nested_value(&p1_mat, new_y, new_x);

        // Out of map
        if new_position_option.is_none() {
            // Mark current position as visited and break
            p1_mat[guard_y][guard_x] = 'X';
            break;
        }

        let new_position = new_position_option.unwrap();

        match new_position {
            'X' => {
                p1_mat[new_y][new_x] = *guard_direction;
                p1_mat[guard_y][guard_x] = 'X';
                guard_x = new_x;
                guard_y = new_y;
            }
            '#' => match guard_direction {
                '^' => p1_mat[guard_y][guard_x] = '>',
                '>' => p1_mat[guard_y][guard_x] = 'v',
                'v' => p1_mat[guard_y][guard_x] = '<',
                '<' => p1_mat[guard_y][guard_x] = '^',
                _ => panic!("bad state"),
            },
            '.' => {
                p1_mat[new_y][new_x] = *guard_direction;
                p1_mat[guard_y][guard_x] = 'X';
                guard_x = new_x;
                guard_y = new_y;
                num_visited += 1;
            }
            _ => {
                panic!("logic error")
            }
        }
    }

    // Begin Part 2

    let mut p2_mat = map.clone();

    let mut num_loops: u32 = 0;

    for (y, row) in p1_mat.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            // Find all of the visited spots on the original path and try putting obstacles there.
            match c {
                'X' => {
                    // Check to make sure this isn't the starting position
                    if y == starting_y && x == starting_x {
                        continue;
                    }
                    // Place the obstacle and check for an infinite loop.
                    p2_mat[y][x] = '#';
                    let is_loop = check_for_loop(&p2_mat, starting_y, starting_x);
                    num_loops += is_loop as u32;
                    p2_mat[y][x] = '.';
                }
                _ => {
                    continue;
                }
            }
        }
    }

    return (num_visited, num_loops);
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

/// Checks for an infinite loop given a map state and guard position.
/// Doesn't check
fn check_for_loop(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let mut mat = m.clone();
    let mut visited_states: HashSet<String> = HashSet::new();

    let mut guard_x = x;
    let mut guard_y = y;

    loop {
        // Don't need options bc this is guarenteed to exist
        let guard_direction = get_nested_value(&mat, guard_y, guard_x).unwrap();

        // Encoding the direction and position of the guard
        let encoded_state = format!("{}({},{})", *guard_direction, guard_x, guard_y);

        if visited_states.contains(&encoded_state) {
            // We've reached a previously seen state so the path is a loop.
            return true;
        } else {
            visited_states.insert(encoded_state);
        }

        // Get next position
        let mut new_x = guard_x;
        let mut new_y = guard_y;

        match guard_direction {
            '^' => {
                if new_y == 0 {
                    return false;
                }
                new_y -= 1;
            }
            'v' => new_y += 1,
            '<' => {
                if new_x == 0 {
                    return false;
                }
                new_x -= 1;
            }
            '>' => new_x += 1,
            _ => {
                println!(
                    "Passed invalid guard location to check_for_loop: {}",
                    guard_direction
                ); // Bad program state
                return false;
            }
        }

        let new_position_option = get_nested_value(&mat, new_y, new_x);

        // Out of map
        if new_position_option.is_none() {
            return false;
        }

        let new_position = new_position_option.unwrap();

        match new_position {
            'X' => {
                mat[new_y][new_x] = *guard_direction;
                mat[guard_y][guard_x] = 'X';
                guard_x = new_x;
                guard_y = new_y;
            }
            '#' => match guard_direction {
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
            }
            _ => {
                panic!("logic error")
            }
        }
    }
}
