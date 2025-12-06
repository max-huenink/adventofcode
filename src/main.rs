mod year2023;
mod year2025;

fn main() {
    let supported_puzzles = ["2023/1", "2023/2", "2023/3", "2025/1"];
    println!("Supported puzzles are: ");
    for supported in supported_puzzles {
        println!("{supported}");
    }
    println!("");
    println!("Input the year, day, and part of the puzzle to solve. e.g. 2025/1/1 or 25/1/1");

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
        _ => panic!("Puzzle not supported, you entered {puzzle_parts:?}"),
    };

    println!("Please enter the puzzle input followed by an empty line to run the puzzle:");

    let mut puzzle_input = Vec::<String>::new();
    loop {
        if let Some(line) = read_input() {
            if !line.trim().is_empty() {
                puzzle_input.push(line);
                continue;
            }
        }
        break;
    }

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
