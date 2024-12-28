use std::collections::HashSet;

pub fn solve(data: &str) -> (u32, u32) {
    let mat: Vec<Vec<char>> = data
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let mut p1_ans = 0;
    for (y, row) in mat.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '0' {
                p1_ans += find_paths(&mat, &mut HashSet::new(), Some(x), Some(y), '0');
            }
        }
    }
    return (p1_ans, 0);
}

/// This counts all paths that end in a 9 ... not what the question was asking
fn find_paths(
    mat: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    x: Option<usize>,
    y: Option<usize>,
    current: char,
) -> u32 {
    // Check if move is in bounds
    let c = get_nested_value(&mat, x, y);
    if c.is_none() {
        return 0;
    }

    // Check if the height is what we are looking for
    if *c.unwrap() != current {
        return 0;
    }
    // Check if we've been here
    // This needs to be after the check above or else we'll block off
    // paths that we have not explored.
    if visited.contains(&(x.unwrap(), y.unwrap())) {
        return 0;
    }
    visited.insert((x.unwrap(), y.unwrap()));
    // Check for end of trail.
    if *c.unwrap() == '9' {
        return 1;
    }
    // Really wish the ? operator worked differently
    return find_paths(
        mat,
        visited,
        x.and_then(|val| val.checked_add(1)),
        y,
        (current as u8 + 1) as char,
    ) + find_paths(
        mat,
        visited,
        x.and_then(|val| val.checked_sub(1)),
        y,
        (current as u8 + 1) as char,
    ) + find_paths(
        mat,
        visited,
        x,
        y.and_then(|val| val.checked_add(1)),
        (current as u8 + 1) as char,
    ) + find_paths(
        mat,
        visited,
        x,
        y.and_then(|val| val.checked_sub(1)),
        (current as u8 + 1) as char,
    );
}

fn get_nested_value<T>(mat: &Vec<Vec<T>>, x: Option<usize>, y: Option<usize>) -> Option<&T> {
    return mat.get(y?)?.get(x?);
}
