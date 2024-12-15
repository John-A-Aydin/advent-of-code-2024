use std::fs;

pub fn solve() -> (i32, i32) {
    let contents = fs::read_to_string("data/data.txt").expect("Could not find data.");

    let reports = contents.split('\n').collect::<Vec<&str>>();

    let part_1_ans = reports.iter().fold(0, |acc, report| {
        return acc + check_safety(report) as i32;
    });

    let part_2_ans = reports.iter().fold(0, |acc, report| {
        return acc + check_safety_with_dampener(report) as i32;
    });

    return (part_1_ans, part_2_ans);
}

fn check_safety(report: &str) -> bool {
    let mut prev: Option<i32> = None;
    let mut asc: Option<bool> = None;
    println!("Report: {}", report);
    return report.split(' ').fold(true, |acc: bool, val: &str| {
        if !acc {
            return false;
        }
        let num = val.parse::<i32>().unwrap();
        if prev.is_none() {
            prev = Some(num);
            return true;
        }
        if asc.is_none() {
            asc = Some(prev.unwrap() < num);
        }
        let mut diff = prev.unwrap() - num;
        if asc.unwrap() {
            diff = -diff;
        }
        prev = Some(num);
        return 1 <= diff && diff <= 3;
    });
}

fn check_safety_with_dampener(report: &str) -> bool {
    if check_safety(report) {
        return true;
    }
    let values = report.split(' ').collect::<Vec<&str>>();
    let mut is_safe = false;
    for i in 0..values.len() {
        let mut s = String::new();
        for (j, val) in values.iter().enumerate() {
            if i != j {
                s.push_str(*val);
                s.push_str(" ")
            }
        }
        s.pop();
        is_safe |= check_safety(s.as_str());
    }
    return is_safe;

    /*
    Any edge between two nodes has a four possibilities:
     - Good distance, good order
     - Good distance, bad order
     - Bad distance, good order
     - Bad distance, bad order
    In both variations of the bad distance edge,
    one of the two nodes (i and i+1) connected by the edge must be removed.
    After this, the edge between i-1 and i+1 or i and i+2 needs to be checked.

    If we read the list from left to right and find a bad
    order on the second edge (between nodes 2 and 3),
    either the first, second, or third need to be removed.

     */
}
