pub fn run_part1(input: Vec<String>) {
    let mut point = 50;
    let mut zeros = 0;

    for line in input {
        let direction = line.chars().nth(0);
        let Ok(count) = line[1..].trim().parse::<i32>() else {
            panic!("Count was not a number, parsing {line}");
        };

        point += count
            * match direction {
                Some('L') => -1,
                Some('R') => 1,
                _ => panic!("Direction was not 'L' or 'R', parsing {line}"),
            };

        point %= 100;

        if point == 0 {
            zeros += 1;
        }
    }

    println!("{zeros}");
}

pub fn run_part2(input: Vec<String>) {
    let mut point = 50;
    let mut zeros = 0;

    for line in input {
        let direction = line.chars().nth(0);
        let Ok(count) = line[1..].trim().parse::<i32>() else {
            panic!("Count was not a number, parsing {line}");
        };

        point += count
            * match direction {
                Some('L') => {
                    if point != 0 && count > point {
                        zeros += 1;
                    }

                    -1
                }
                Some('R') => 1,
                _ => panic!("Direction was not 'L' or 'R', parsing {line}"),
            };

        if point == 0 {
            zeros += 1
        } else {
            zeros += point.abs() / 100;
            point %= 100;
            if point < 0 {
                point += 100;
            }
        }
    }

    println!("{zeros}");
}
