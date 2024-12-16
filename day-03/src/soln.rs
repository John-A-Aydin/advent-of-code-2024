use regex::Regex;

pub fn solve_p1(data: String) -> i32 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let funcs: Vec<&str> = re
        .captures_iter(&data)
        .map(|captures| captures.get(0).unwrap().as_str())
        .collect();
    let ans: i32 = funcs.into_iter().map(|s| calculate_mul(s)).sum();
    return ans;
}

pub fn solve_p2(data: String) -> i32 {
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();

    let mut should_do = true;

    let ans: i32 = re
        .captures_iter(&data)
        .map(|captures| {
            let s = captures.get(0).unwrap().as_str();
            match s {
                "do()" => should_do = true,
                "don't()" => should_do = false,
                _ => {
                    if should_do {
                        return calculate_mul(s);
                    }
                }
            }
            return 0;
        })
        .sum();
    return ans;
}

fn calculate_mul(f: &str) -> i32 {
    let start = f.char_indices().nth(4).unwrap().0;
    let end: usize = f.char_indices().rev().nth(0).unwrap().0;
    let result = &f[start..end];
    return result.split(',').fold(1, |acc, val| {
        return acc * val.parse::<i32>().unwrap();
    });
}
