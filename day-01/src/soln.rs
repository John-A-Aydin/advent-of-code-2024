use std::collections::HashMap;
use std::fs;

pub fn solve() -> (i32, i32) {
    let contents = fs::read_to_string("data/part-1.txt").expect("Could not find data.");

    let (mut vec1, mut vec2) = parse(contents);

    vec1.sort();
    vec2.sort();

    let mut diff: Vec<i32> = Vec::new();

    for it in vec1.iter().zip(vec2.iter()) {
        let (a, b) = it;

        diff.push((a - b).abs());
    }

    let pt_1_ans = diff.iter().sum();

    // Create HashMap of counts for each value in vec2
    let mut counts: HashMap<i32, i32> = HashMap::new();
    vec2.iter().for_each(|val| match counts.get_mut(val) {
        Some(prev) => {
            *prev += 1;
        }
        None => {
            counts.insert(*val, 1);
        }
    });

    let pt_2_ans = vec1.iter().fold(0, |acc, val| match counts.get(val) {
        Some(n) => {
            return acc + val * n;
        }
        None => {
            return acc;
        }
    });

    return (pt_1_ans, pt_2_ans);
}

fn parse(data: String) -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    //println!("{}", data);
    let mut cnt = 0;

    let _ = data
        .split([' ', '\n'])
        .filter_map(|val| val.parse::<i32>().ok())
        .for_each(|num| {
            if cnt % 2 == 0 {
                vec1.push(num);
            } else {
                vec2.push(num);
            }
            cnt += 1;
        });
    assert!(vec1.len() == vec2.len());
    return (vec1, vec2);
}
