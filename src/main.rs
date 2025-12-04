mod year2023;

fn main() {
    let supported_puzzles = ["2023/1", "2023/2", "2023/3", "2024/1"];
    println!("Input the year, day, and part of the puzzle to solve. e.g. 2025/1/1 or 25/1/1");
    println!("Supported puzzles are: ");
    for supported in supported_puzzles {
        print!("{supported}");
    }
    print!("\n");

    let Some(puzzle) = read_input() else {
        panic!("No input received!");
    };

    if !supported_puzzles.iter().any(|s| puzzle.starts_with(s)) {
        panic!("Puzzle {puzzle} is not supported!");
    }

    // TODO: Fix?
    // 2023/1/1 -> [2023, 1]
    let v: Vec<i32> = puzzle
        .split('/')
        .take(3)
        .filter_map(|p| p.parse::<i32>().ok())
        .collect();

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

    // TODO: Switch to a trait for the different days, so we don't grow this exponentially year over year?
    // TODO: Switch to a trait for the part1/2 functions
    match v[..] {
        [2023, 1, 1] => year2023::day1::run_part1(puzzle_input),
        [2023, 1, 2] => year2023::day1::run_part2(puzzle_input),
        [2023, 2, 1] => year2023::day2::run_part1(puzzle_input),
        [2023, 2, 2] => year2023::day2::run_part2(puzzle_input),
        [2023, 3, 1] => year2023::day3::run_part1(puzzle_input),
        [2023, 3, 2] => year2023::day3::run_part2(puzzle_input),
        _ => panic!("Puzzle not supported, you entered {v:?}"),
    }
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
