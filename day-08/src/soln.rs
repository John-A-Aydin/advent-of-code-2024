pub fn solve(data: &str) -> (u32, u32) {
    let mat: Vec<Vec<char>> = data
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    // for each antenna, look at previously seen antennas on the same frequency and find their antinodes,
    // save the antenna's location, and continue

    // Assume the input is rectangular
    let mut antinodes_p1: Vec<Vec<bool>> = vec![vec![false; mat[0].len()]; mat.len()];
    let mut antinodes_p2: Vec<Vec<bool>> = vec![vec![false; mat[0].len()]; mat.len()];

    // List of previously seen antenas
    let mut antenas: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 75];

    for (y_1, row) in mat.iter().enumerate() {
        for (x_1, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            // Loop over every previously seen antenna
            for (x_2, y_2) in antenas[*c as usize - 48].iter() {
                // Add antinodes to the antinode matrix
                let dx = x_1.abs_diff(*x_2);
                let dy = y_1.abs_diff(*y_2);
                // Anti-node coords
                let mut x_3;
                let mut y_3;
                let mut x_4;
                let mut y_4;
                let mut dist = 0;
                loop {
                    // Calculate position of antinode
                    if x_1 > *x_2 {
                        x_3 = x_1.checked_add(dx * dist);
                        x_4 = x_2.checked_sub(dx * dist);
                    } else {
                        x_3 = x_1.checked_sub(dx * dist);
                        x_4 = x_2.checked_add(dx * dist);
                    }
                    if y_1 > *y_2 {
                        y_3 = y_1.checked_add(dy * dist);
                        y_4 = y_2.checked_sub(dy * dist);
                    } else {
                        y_3 = y_1.checked_sub(dy * dist);
                        y_4 = y_2.checked_add(dy * dist);
                    }
                    // Check for valid points
                    // these names suck but i dont feel like fixing them
                    if dist == 1 {
                        let p1_3 = get_nested_mut(&mut antinodes_p1, y_3, x_3);
                        match p1_3 {
                            Some(p) => *p = true,
                            None => {}
                        }
                        let p1_4 = get_nested_mut(&mut antinodes_p1, y_4, x_4);
                        match p1_4 {
                            Some(p) => *p = true,
                            None => {}
                        }
                    }
                    let mut p2_3_done = false;
                    let mut p2_4_done = false;
                    let p2_3 = get_nested_mut(&mut antinodes_p2, y_3, x_3);
                    match p2_3 {
                        Some(p) => *p = true,
                        None => p2_3_done = true,
                    }
                    let p2_4 = get_nested_mut(&mut antinodes_p2, y_4, x_4);
                    match p2_4 {
                        Some(p) => *p = true,
                        None => p2_4_done = true,
                    }
                    if p2_3_done && p2_4_done {
                        break;
                    }
                    dist += 1;
                }
            }
            antenas[*c as usize - 48].push((x_1, y_1));
        }
    }

    // Sum up unique antinodes
    let p1_ans = antinodes_p1.iter().fold(0, |acc_y, row| {
        return acc_y
            + row.iter().fold(0, |acc_x, val| {
                return acc_x + *val as u32;
            });
    });
    let p2_ans = antinodes_p2.iter().fold(0, |acc_y, row| {
        return acc_y
            + row.iter().fold(0, |acc_x, val| {
                return acc_x + *val as u32;
            });
    });

    return (p1_ans, p2_ans);
}

fn get_nested_mut<T>(mat: &mut Vec<Vec<T>>, y: Option<usize>, x: Option<usize>) -> Option<&mut T> {
    return mat.get_mut(y?)?.get_mut(x?);
}
