use std::ops::Index;

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
            if first.1.is_digit(10) && second.1.is_digit(10) {
                return first.1.to_digit(10).unwrap() * 10 + second.1.to_digit(10).unwrap();
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
            let mut str = String::new();
            let mut all = [(0, 0 as u128); 12];
            for idx in 0..12 {
                let skip_amt = if idx == 0 { 0 } else { all[idx - 1].0 + 1 };
                let take_amt = line.len() - (skip_amt + (12 - idx));
                for c in line.char_indices().skip(skip_amt).take(take_amt) {
                    if let Some(num) = c.1.to_digit(10) {
                        let num2 = num as u128;
                        if num2.cmp(&all[idx].1).is_gt() {
                            all[idx] = (c.0, num2);
                        }
                    }
                }
                str.push_str(&all[idx].1.to_string());
            }

            if let Ok(val) = str.parse::<u128>() {
                return val;
            }

            0
        })
        .sum::<u128>();

    println!("{answer}");
}
