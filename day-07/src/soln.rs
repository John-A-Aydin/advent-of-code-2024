pub fn solve(data: &str) -> (u64, u64) {
    /// No idea what to call this
    /// enter with idx = 0, current_value = None
    fn rec(operands: &[u64], idx: usize, test_value: u64, current_value: Option<u64>) -> bool {
        if idx == 0 || current_value.is_none() {
            return rec(operands, 1, test_value, Some(operands[0]));
        }
        // Recursion goes past the length of operands by 1
        if idx >= operands.len() {
            return current_value.unwrap() == test_value;
        }
        return rec(
            operands,
            idx + 1,
            test_value,
            Some(current_value.unwrap() + operands[idx]),
        ) || rec(
            operands,
            idx + 1,
            test_value,
            Some(current_value.unwrap() * operands[idx]),
        );
    }

    /// No idea what to call this
    /// enter with idx = 0, current_value = None
    fn rec2(operands: &[u64], idx: usize, test_value: u64, current_value: Option<u64>) -> bool {
        // Recursion goes past the length of operands by 1
        if idx == 0 || current_value.is_none() {
            return rec2(operands, 1, test_value, Some(operands[0]));
        }
        if idx >= operands.len() {
            return current_value.unwrap() == test_value;
        }
        let concat_value =
            current_value.unwrap() * 10u64.pow(operands[idx].ilog10() + 1) + operands[idx];
        return rec2(
            operands,
            idx + 1,
            test_value,
            Some(current_value.unwrap() + operands[idx]),
        ) || rec2(
            operands,
            idx + 1,
            test_value,
            Some(current_value.unwrap() * operands[idx]),
        ) || rec2(operands, idx + 1, test_value, Some(concat_value));
    }

    let mut p1_ans = 0;
    let mut p2_ans = 0;

    data.split('\n').for_each(|line| {
        let eqn: Vec<&str> = line.split(": ").collect();
        let test_value = eqn[0].parse::<u64>().expect("non-numeric input");
        let operands: Vec<u64> = eqn[1]
            .split(' ')
            .map(|operand_str| operand_str.parse::<u64>().expect("non-numeric input"))
            .collect();

        if rec(&operands.as_slice(), 0, test_value, None) {
            p1_ans += test_value;
        }
        if rec2(&operands.as_slice(), 0, test_value, None) {
            p2_ans += test_value;
        }
    });

    return (p1_ans, p2_ans);
}
