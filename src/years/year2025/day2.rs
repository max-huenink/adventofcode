use crate::PuzzleParts;

pub struct Puzzle;

impl PuzzleParts for Puzzle {
    fn run_part1(&self, input: &[String]) -> String {
        if input.len() > 1 {
            panic!("Input should only be a single line!");
        }
        let Some(line) = input.first() else {
            panic!("Input should only be a single line!")
        };
        let ans: i64 = line
            .split(',')
            .map(|r| {
                let mut id_range = r.split('-').map(|i| i.trim());

                if let Some(start) = id_range.next()
                    && let Some(end) = id_range.next()
                    && (start.len() % 2 == 0 || end.len() % 2 == 0 || start.len() - end.len() > 1)
                {
                    let mut count = 0;
                    let start = start.parse::<i64>().unwrap();
                    let end = end.parse::<i64>().unwrap() + 1;

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

        ans.to_string()
    }

    fn run_part2(&self, input: &[String]) -> String {
        if input.len() > 1 {
            panic!("Input should only be a single line!");
        }
        let Some(line) = input.first() else {
            panic!("Input should only be a single line!")
        };

        let ans: i64 = line
            .split(',')
            .map(|r| {
                let mut id_range = r.split('-').filter_map(|i| i.trim().parse::<i64>().ok());

                let mut count = 0;
                if let Some(start) = id_range.next()
                    && let Some(end) = id_range.next()
                {
                    for id in start..end {
                        let id_str = id.to_string();
                        let length = id_str.len();

                        if length == 1 {
                            continue;
                        } else if id_str.chars().all(|c| c == id_str.chars().nth(0).unwrap())
                            || id_has_repeating_pattern(&id_str, length)
                        {
                            count += id;
                        }
                    }
                }

                count
            })
            .sum();

        ans.to_string()
    }
}

fn id_has_repeating_pattern(id_str: &str, length: usize) -> bool {
    for partition_size in 2..(length / 2) + 1 {
        if length.is_multiple_of(partition_size) {
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
