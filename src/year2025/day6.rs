pub fn run_part1(input: &[String]) {
    let mut numbers = Vec::<Vec<i64>>::new();
    let mut operations = Vec::<char>::new();

    for line in input {
        let mut columns = line
            .split(char::is_whitespace)
            .filter(|s| !s.trim().is_empty());

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
                .copied()
                .reduce(|acc, n| acc * n)
                .unwrap_or(0),
            '+' => numbers[idx].iter().sum::<i64>(),
            _ => 0,
        };
    }

    println!("{sum}");
}

pub fn run_part2(input: &[String]) {
    let mut numbers = Vec::<Vec<&str>>::new();
    let mut operations = Vec::<char>::new();

    for line in input {
        let mut columns = line
            .split(char::is_whitespace)
            .filter(|s| !s.trim().is_empty());

        let mut idx = 0;
        while let Some(cell) = columns.next() {
            if let Ok(_) = cell.parse::<i64>() {
                if idx >= numbers.len() {
                    numbers.push(vec![]);
                }

                numbers[idx].push(cell);
            } else if cell.len() == 1
                && let Some(op) = cell.chars().next()
            {
                operations.push(op);
            }

            idx += 1;
        }
    }

    let mut sum = 0;
    let mut idx = 0;
    let number_columns = numbers.iter().map(|col| col.iter().map(|c| c.chars()));

    for mut col in number_columns {
        let op = operations[idx];
        let mut acc = 0;
        let mut f = true;

        while f {
            f = false;
            let mut new_number = 0;

            for mut cell in &mut col {
                if let Some(c) = cell.next()
                    && let Some(d) = c.to_digit(10)
                {
                    new_number += (new_number * 10) + d;
                    f = true;
                }
            }

            match op {
                '*' => acc *= new_number,
                '+' => acc += new_number,
                _ => (),
            };
        }

        sum += acc;
        idx += 1;
    }

    println!("{sum}");
}
