use crate::lines::read;

pub fn run_part1() {
    // Each line contains numeric digits (1-9).
    // Get the first (tens place) and last (ones place) digit in a line and add to result

    let mut result = 0;
    // Read lines of input
    if let Ok(lines) = read("./inputs/day1/input.txt") {
        // Iterate over the lines
        for line_result in lines {
            // If the line is okay
            if let Ok(line) = line_result {
                // Only get the digits out
                let digits = line.chars().filter_map(|c| c.to_digit(10));
                // Get the first digit and multiply by 10
                let tens = digits.clone().nth(0).unwrap_or(0) * 10;
                // Get the last digit
                let ones = digits.clone().nth_back(0).unwrap_or(0);

                // Add the tens and ones to result
                result = result + tens + ones;
            }
        }
    };

    println!("{}", result);
}

pub fn run_part2() {
    // Each line contains numeric digits (1-9) or spelled out digits (one, two, three, four, five, six, seven, eight, nine).
    // Get the first (tens place) and last (ones place) digit in a line and add to result

    let mut result = 0;
    // Read lines of input
    if let Ok(lines) = read("./inputs/day1/input.txt") {
        // Iterate over the lines
        for line_result in lines {
            // If the line is okay
            if let Ok(line) = line_result {
                let len = line.len();
                let mut tens: u32 = 0;
                let mut ones: u32 = 0;
                // Iterate twice, so we can expand the search rightwards
                for i in 0..len {
                    for j in i..(len + 1) {
                        // digit words have a max length of 5, so if j - i > 5, exit the inner loop
                        if (j - i) > 5 {
                            break;
                        }

                        // Get the string from i to j
                        let a = line[i..j].to_string();
                        // Match that to a digit
                        let digit_opt = match a.as_str() {
                            "1" | "one" => Some(1),
                            "2" | "two" => Some(2),
                            "3" | "three" => Some(3),
                            "4" | "four" => Some(4),
                            "5" | "five" => Some(5),
                            "6" | "six" => Some(6),
                            "7" | "seven" => Some(7),
                            "8" | "eight" => Some(8),
                            "9" | "nine" => Some(9),
                            _ => None,
                        };
                        // If the match was successful
                        if let Some(digit) = digit_opt {
                            // Only set tens once
                            if tens == 0 {
                                tens = digit * 10;
                            }
                            // Set ones every time
                            ones = digit;
                            break;
                        }
                    }
                }
                // Finally, add to result
                result = result + tens + ones;
            }
        }
    };

    println!("{}", result);
}
