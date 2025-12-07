pub fn run_part1(input: Vec<String>) {
    if input.len() > 1 {
        panic!("Input should only be a single line!");
    }
    let Some(line) = input.first() else {
        panic!("Input should only be a single line!")
    };
    let ans: i64 = line
        .split(',')
        .map(|r| {
            let id_range: Vec<&str> = r.split('-').map(|i| i.trim()).collect();

            if id_range.len() == 2
                && (id_range[0].len() % 2 == 0
                    || id_range[1].len() % 2 == 0
                    || id_range[1].len() - id_range[0].len() > 1)
            {
                let mut count = 0;
                let start = id_range[0].parse::<i64>().unwrap();
                let end = id_range[1].parse::<i64>().unwrap() + 1;

                for id in start..end {
                    let id_str = id.to_string();
                    let id_len = id_str.len();
                    let half = id_len / 2;

                    if id_len % 2 == 0 && id_str[..half] == id_str[half..] {
                        count += id;
                    }
                }

                count
            } else {
                0
            }
        })
        .sum();

    println!("{ans}");
}

pub fn run_part2(input: Vec<String>) {
    if input.len() > 1 {
        panic!("Input should only be a single line!");
    }
    let Some(line) = input.first() else {
        panic!("Input should only be a single line!")
    };

    let ans: i64 = line
        .split(',')
        .map(|r| {
            let id_range: Vec<&str> = r.split('-').map(|i| i.trim()).collect();

            if id_range.len() == 2 {
                let mut count = 0;
                let start = id_range[0].parse::<i64>().unwrap();
                let end = id_range[1].parse::<i64>().unwrap() + 1;

                for id in start..end {
                    let id_str = id.to_string();
                    let length = id_str.len();

                    if length == 1 {
                        continue;
                    } else if id_str.chars().all(|c| c == id_str.chars().nth(0).unwrap()) {
                        count += id;
                    } else if id_has_repeating_pattern(&id_str, length) {
                        count += id;
                    }
                }
                count
            } else {
                0
            }
        })
        .sum();

    println!("{ans}");
}

fn id_has_repeating_pattern(id_str: &str, length: usize) -> bool {
    for partition_size in 2..(length / 2) + 1 {
        if length % partition_size == 0 {
            if partition_size == length / 2 {
                if id_str[..partition_size] == id_str[partition_size..] {
                    return true;
                }
            } else {
                if partition_repeats(id_str, length, partition_size) {
                    return true;
                }
            }
        }
    }

    false
}

fn partition_repeats(id_str: &str, length: usize, partition_size: usize) -> bool {
    let to_match = &id_str[..partition_size];
    let mut prev = partition_size;

    for _ in 1..(length / partition_size) {
        if to_match != &id_str[prev..prev + partition_size] {
            return false;
        }

        prev += partition_size;
    }

    true
}
