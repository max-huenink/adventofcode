pub fn run_part1(input: Vec<String>) {
    let answer = input
        .iter()
        .map(|line| {
            let mut first = (0, '0');
            let mut second = (0, '0');
            for c in line.char_indices().take(line.len() - 2) {
                if c.1.cmp(&first.1).is_gt() {
                    first = c;
                }
            }
            for c in line.char_indices().skip(first.0 + 1) {
                if c.1.cmp(&second.1).is_gt() {
                    second = c;
                }
            }
            println!("{line} {first:?} {second:?}");
            if let Some(first_digit) = first.1.to_digit(10)
                && let Some(second_digit) = second.1.to_digit(10)
            {
                return first_digit * 10 + second_digit;
            }

            0
        })
        .sum::<u32>();

    println!("{answer}");
}

pub fn run_part2(input: Vec<String>) {
    let answer = input
        .iter()
        .map(|line| {
            let mut joltage = 0;
            let mut indices = [0; 12];
            for idx in 0..12 {
                let mut current = 0;
                let skip_amt = if idx == 0 { 0 } else { indices[idx - 1] + 1 };
                let take_amt = line.len() - (skip_amt + (12 - idx));
                for c in line.char_indices().skip(skip_amt).take(take_amt) {
                    if let Some(num) = c.1.to_digit(10).map(|d| d as u64) {
                        if num > current {
                            current = num;
                            indices[idx] = c.0;
                        }
                        if current == 9 {
                            continue;
                        }
                    }
                }
                joltage += 10_u64.pow(11 - idx as u32) * current;
            }

            joltage
        })
        .sum::<u64>();

    println!("{answer}");
}
