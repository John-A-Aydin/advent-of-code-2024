pub fn solve_p1(data: &str) -> u64 {
    let mut sum = 0;

    // F it we going recursive
    fn rec(operands: &[u64], idx: usize, test_value: u64, current_value: u64) -> bool {
        // return rec(operands, op)
        // Recursion goes past the length of operands by 1
        if idx >= operands.len() {
            return current_value == test_value;
        }
        return rec(operands, idx + 1, test_value, current_value + operands[idx])
            || rec(operands, idx + 1, test_value, current_value * operands[idx]);
    }

    data.split('\n').for_each(|line| {
        let eqn: Vec<&str> = line.split(": ").collect();
        let test_value = eqn[0].parse::<u64>().expect("non-numeric input");
        let operands: Vec<u64> = eqn[1]
            .split(' ')
            .map(|operand_str| operand_str.parse::<u64>().expect("non-numeric input"))
            .collect();

        if rec(&operands.as_slice(), 1, test_value, operands[0]) {
            sum += test_value;
        }
    });
    return sum;
}
