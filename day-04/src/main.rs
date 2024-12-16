use std::fs;

mod soln;

fn main() {
    let data = fs::read_to_string("data/data.txt").expect("Could not find data");
    let part_1_ans = soln::solve_p1(&data);
    println!("Part 1 Answer: {}", part_1_ans);
}
