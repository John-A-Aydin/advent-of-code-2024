use std::fs;

mod soln;

fn main() {
    let data = fs::read_to_string("data/data.txt").expect("data not found");
    let (part_1_ans, part_2_ans) = soln::solve(&data);
    println!("Part 1 Answer: {}", part_1_ans);
    println!("Part 2 Answer: {}", part_2_ans);
}
