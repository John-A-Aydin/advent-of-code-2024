use std::fs;

pub fn solve() -> i32 {
    let contents = fs::read_to_string("data/part-1.txt").expect("Could not find data.");

    let (mut vec1, mut vec2) = parse(contents);

    vec1.sort();
    vec2.sort();

    //println!("{:?}", vec1);

    let mut diff: Vec<i32> = Vec::new();

    for it in vec1.iter().zip(vec2.iter()) {
        let (a, b) = it;

        diff.push((a - b).abs());
    }

    let ans = diff.iter().sum();
    return ans;
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
    println!("Vec 1 len: {}\nVec 2 len: {}", vec1.len(), vec2.len());
    assert!(vec1.len() == vec2.len());
    return (vec1, vec2);
}
