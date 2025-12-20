pub fn run_part1(input: &[String]) {
    let mut numbers = Vec::<Vec<i64>>::new();
    let mut operations = Vec::<char>::new();

    for line in input {
        let mut columns = line.split_whitespace();

        let mut idx = 0;
        while let Some(cell) = columns.next() {
            if let Ok(num) = cell.parse::<i64>() {
                if idx >= numbers.len() {
                    numbers.push(vec![]);
                }

                numbers[idx].push(num);
            } else if cell.len() == 1
                && let Some(op) = cell.chars().next()
            {
                operations.push(op);
            }

            idx += 1;
        }
    }

    let mut sum = 0;
    for idx in 0..operations.len() {
        sum += match operations[idx] {
            '*' => numbers[idx]
                .iter()
                .fold(0, |acc, n| if acc == 0 { *n } else { acc * n }),
            '+' => numbers[idx].iter().sum::<i64>(),
            _ => 0,
        };
    }

    println!("{sum}");
}

pub fn run_part2(input: &[String]) {
    let mut numbers = Vec::<Vec<&str>>::new();

    let Some(last_line) = input.last() else {
        panic!("No last line?");
    };
    let operations_with_indices = last_line
        .match_indices(|c: char| !c.is_whitespace())
        .collect::<Vec<_>>();

    for line in &input[..(input.len() - 1)] {
        for idx in 0..operations_with_indices.len() {
            let r = if idx == operations_with_indices.len() - 1 {
                operations_with_indices[idx].0..(line.len() - 1)
            } else {
                operations_with_indices[idx].0..(operations_with_indices[idx + 1].0 - 1)
            };
            let cell = &line[r];

            if idx >= numbers.len() {
                numbers.push(vec![cell]);
            } else {
                numbers[idx].push(cell);
            }
        }
    }

    let mut sum = 0;
    let mut idx = 0;
    let number_columns = numbers
        .iter()
        .map(|col| col.iter().map(|c| c.chars().rev()).collect::<Vec<_>>());

    for mut col in number_columns {
        let op = operations_with_indices[idx].1;
        let mut accumulator = 0;
        let mut found_a_number = true;

        while found_a_number {
            found_a_number = false;
            let mut new_number = 0;

            for cell in &mut col {
                if let Some(c) = cell.next()
                    && let Some(d) = c.to_digit(10)
                {
                    new_number = (new_number * 10) + d as u64;
                    found_a_number = true;
                }
            }

            if new_number != 0 {
                match op {
                    "*" => {
                        if accumulator == 0 {
                            accumulator = new_number;
                        } else {
                            accumulator *= new_number;
                        }
                    }
                    "+" => accumulator += new_number,
                    _ => (),
                };
            }
        }

        sum += accumulator;
        idx += 1;
    }

    println!("{sum}");
}
