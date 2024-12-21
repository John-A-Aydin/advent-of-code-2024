pub fn solve_p1(data: &str) -> u32 {
    // Get rules and updates
    let rules_and_updates = data.split("\n\n\n").collect::<Vec<&str>>();
    // Parsing rules into a list of ordered pairs
    let rules = rules_and_updates[0]
        .split('\n')
        .map(|rule| {
            let parsed = rule
                .split('|')
                .map(|num| {
                    return num.parse::<u32>().unwrap();
                })
                .collect::<Vec<u32>>();
            return (parsed[0], parsed[1]);
        })
        .collect::<Vec<(u32, u32)>>();

    // Parsing updates lists of u32's
    let updates = rules_and_updates[1]
        .split('\n')
        .map(|update| {
            return update
                .split(',')
                .map(|page_num| return page_num.parse::<u32>().unwrap())
                .collect();
        })
        .collect::<Vec<Vec<u32>>>();

    return 15;
}

pub fn solve_p2(data: &str) -> u32 {
    ///
    return 0;
}

fn create_partial_order(rules: Vec<(u32, u32)>) -> u32 {
    return 16;
}
