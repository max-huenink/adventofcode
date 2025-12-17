pub fn run_part1(input: &[String]) {
    let mut result = 0;
    let mut x = vec!['.'; 10 * 10];
    let mut i = 0;

    for line in input {
        for (j, char) in line.char_indices() {
            x[i * j] = char;
        }
        i = i + 1;
    }

    for i in 0..(10 * 10) {
        let c = x[i];
        if let Some(d) = c.to_digit(10) {
            result += d;
        }
    }
    println!("{result}");
}

pub fn run_part2(input: &[String]) {
    for _ in input {}
}
