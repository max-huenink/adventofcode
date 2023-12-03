use crate::lines::read;

pub fn run_part1() {
    let mut x = vec!['.'; 10 * 10];
    // Read lines of input
    if let Ok(lines) = read("./inputs/day3/test1.txt") {
        // Iterate over the lines
        let mut i = 0;
        for line_result in lines {
            // If the line is okay
            if let Ok(line) = line_result {
                for (j, char) in line.char_indices() {
                    x[i * j] = char;
                }
            }
            i = i + 1;
        }
    }
    for i in 0..(10 * 10) {
        let c = x[i];
        if let Some(d) = c.to_digit(10) {}
    }
}

pub fn run_part2() {
    // Read lines of input
    if let Ok(lines) = read("./inputs/day3/test2.txt") {
        // Iterate over the lines
        for line_result in lines {
            // If the line is okay
            if let Ok(line) = line_result {}
        }
    }
}
