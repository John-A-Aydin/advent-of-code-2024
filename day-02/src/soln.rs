use std::fs;

pub fn solve() -> i32 {
    let contents = fs::read_to_string("data/data.txt").expect("Could not find data.");

    let reports = contents.split('\n').collect::<Vec<&str>>();

    let part_1_ans = reports.iter().fold(0, |acc, report| {
        return acc + check_safety(report) as i32;
    });

    return part_1_ans;
}

fn check_safety(report: &str) -> bool {
    let mut prev: Option<i32> = None;
    let mut asc: Option<bool> = None;
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
