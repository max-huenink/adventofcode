use crate::lines::read;

pub fn run_part1() {
    let mut result = 0;
    // Read lines of input
    if let Ok(lines) = read("./inputs/day2/input.txt") {
        // Iterate over the lines
        for line_result in lines {
            // If the line is okay
            if let Ok(line) = line_result {
                let mut game_info = line.split(": ");
                if let Some(game_id) = game_info.nth(0) {
                    if let Some(id_str) = game_id.split(' ').nth(1) {
                        if let Ok(id) = id_str.parse::<u32>() {
                            if let Some(sets) = game_info.nth(0) {
                                if is_game_possible_part1(sets) {
                                    result = result + id;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{result}");
}

fn is_game_possible_part1(sets: &str) -> bool {
    for set in sets.split("; ") {
        let cubes = set.split(", ");
        for cube in cubes {
            let mut section = cube.split(' ');
            if let Some(count_str) = section.nth(0) {
                if let Ok(count) = count_str.parse::<u32>() {
                    if let Some(color_str) = section.nth(0) {
                        let x = match color_str {
                            "red" => count <= 12,
                            "green" => count <= 13,
                            "blue" => count <= 14,
                            _ => false,
                        };
                        if !x {
                            return false;
                        }
                    }
                }
            }
        }
    }
    return true;
}

pub fn run_part2() {
    let mut result = 0;
    // Read lines of input
    if let Ok(lines) = read("./inputs/day2/input.txt") {
        // Iterate over the lines
        for line_result in lines {
            // If the line is okay
            if let Ok(line) = line_result {
                let mut game_info = line.split(": ");
                if let Some(sets) = game_info.nth(1) {
                    result = result + min_cubes_in_game(sets);
                }
            }
        }
    }
    println!("{result}");
}

fn min_cubes_in_game(sets: &str) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in sets.split("; ") {
        let cubes = set.split(", ");
        for cube in cubes {
            let mut section = cube.split(' ');
            if let Some(count_str) = section.nth(0) {
                if let Ok(count) = count_str.parse::<u32>() {
                    if let Some(color_str) = section.nth(0) {
                        match color_str {
                            "red" if count > red => {
                                red = count;
                            }
                            "green" if count > green => {
                                green = count;
                            }
                            "blue" if count > blue => {
                                blue = count;
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
    }
    return red * green * blue;
}
