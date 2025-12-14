mod year2023;
mod year2025;

fn main() {
    let supported_puzzles = [
        "2023/1", "2023/2", "2023/3", "2025/1", "2025/2", "2025/3", "2025/4", "2025/5",
    ];
    println!("Supported puzzles are: ");
    for supported in supported_puzzles {
        println!("{supported}");
    }
    println!("");
    println!("Input the year, day, and part of the puzzle to solve. e.g. 2025/1/1");

    let Some(puzzle) = read_input() else {
        panic!("No input received!");
    };

    if !supported_puzzles.iter().any(|s| puzzle.starts_with(s)) {
        panic!("Puzzle {puzzle} is not supported!");
    }

    let puzzle_parts: Vec<i32> = puzzle
        .split('/')
        .take(3)
        .filter_map(|p| p.trim().parse::<i32>().ok())
        .collect();

    let puzzle_func = match puzzle_parts[..] {
        [2023, 1, 1] => year2023::day1::run_part1,
        [2023, 1, 2] => year2023::day1::run_part2,
        [2023, 2, 1] => year2023::day2::run_part1,
        [2023, 2, 2] => year2023::day2::run_part2,
        [2023, 3, 1] => year2023::day3::run_part1,
        [2023, 3, 2] => year2023::day3::run_part2,
        [2025, 1, 1] => year2025::day1::run_part1,
        [2025, 1, 2] => year2025::day1::run_part2,
        [2025, 2, 1] => year2025::day2::run_part1,
        [2025, 2, 2] => year2025::day2::run_part2,
        [2025, 3, 1] => year2025::day3::run_part1,
        [2025, 3, 2] => year2025::day3::run_part2,
        [2025, 4, 1] => year2025::day4::run_part1,
        [2025, 4, 2] => year2025::day4::run_part2,
        [2025, 5, 1] => year2025::day5::run_part1,
        [2025, 5, 2] => year2025::day5::run_part2,
        _ => panic!("Puzzle not supported, you entered {puzzle_parts:?}"),
    };

    println!("Please enter the puzzle input followed by two empty lines to run the puzzle:");

    let mut puzzle_input = Vec::<String>::new();
    let mut empty_seen = false;
    loop {
        if let Some(line) = read_input() {
            let current_empty = line.trim().is_empty();
            if empty_seen && current_empty {
                break;
            }
            empty_seen = false;

            puzzle_input.push(line);
            if current_empty {
                empty_seen = true;
            }
        }
    }
    // Remove the last line which is empty
    puzzle_input.pop();

    puzzle_func(puzzle_input);
}

fn read_input() -> Option<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(err) => {
            println!("Error reading input {err}");
            None
        }
    }
}
