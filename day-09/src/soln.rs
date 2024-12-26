pub fn solve(data: &str) -> (u64, u32) {
    let chars: Vec<char> = data.chars().collect();
    // This is abt to take a lot of memory .-.
    let mut file_blocks: Vec<usize> = Vec::new();
    let mut l: usize = 0;
    let mut r: usize = chars.len() - 1;
    // Figure out if the right index is pointing at empty blocks
    if r % 2 != 0 {
        r -= 1;
    }
    let mut l_counter: u8 = char_to_u8(chars[0]);
    let mut r_counter: u8 = char_to_u8(chars[chars.len() - 1]);
    let mut id: usize = 0;

    loop {
        if l >= r {
            // Cases where this can happen:
            // - We are emptying r into l, l gets full and new l is r
            //      Finish emptying r
            // - We are emptying r into l, r runs out and new r is before l
            //      Do nothing

            if l == r {
                for _ in 0..r_counter {
                    file_blocks.push(id);
                }
            }
            break;
        }
        // Push from left
        if l % 2 == 0 {
            // l_counter is empty so we move to the next free space
            if l_counter == 0 {
                l += 1;
                l_counter = char_to_u8(chars[l]);
                id = r / 2;
            } else {
                // l is not empty so we push an element from it
                file_blocks.push(id);
                l_counter -= 1;
            }
        } else {
            // In an empty block section (take from right)
            if r_counter == 0 {
                // Exhasted rhs
                // This MUST come before the check for the left counter
                r -= 2;
                r_counter = char_to_u8(chars[r]);
                id = r / 2;
            } else if l_counter == 0 {
                // Exhasted free space
                l += 1;
                l_counter = char_to_u8(chars[l]);
                id = l / 2;
            } else {
                // Push from right, taking one of the free spaces.
                file_blocks.push(id);
                l_counter -= 1;
                r_counter -= 1;
            }
        }
    }

    let mut p1_ans: u64 = 0;
    for (i, block_id) in file_blocks.iter().enumerate() {
        p1_ans += (i * block_id) as u64;
    }
    return (p1_ans, 0);
}

/// I don't feel like dealing with options
fn char_to_u8(c: char) -> u8 {
    return c as u8 - b'0';
}
