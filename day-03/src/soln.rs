use regex::Regex;
use std::fs;

pub fn solve() -> i32 {
    let contents = fs::read_to_string("data/data.txt").expect("Could not find data");

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let funcs: Vec<&str> = re
        .captures_iter(&contents)
        .map(|captures| captures.get(0).unwrap().as_str())
        .collect();
    let ans: i32 = funcs
        .into_iter()
        .map(|s| {
            let start = s.char_indices().nth(4).unwrap().0;
            let end = s.char_indices().rev().nth(0).unwrap().0;
            let result = &s[start..end];
            println!("{}", s);
            return result.split(',').fold(1, |acc, val| {
                println!("{}", val);
                return acc * val.parse::<i32>().unwrap();
            });
        })
        .sum();
    return ans;
}
