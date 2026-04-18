use crate::PuzzleParts;

pub struct Puzzle;

impl PuzzleParts for Puzzle {
    fn run_part1(&self, input: &[String]) -> String {
        let mut result = 0;
        for line in input {
            let mut game_info = line.split(": ");
            if let Some(game_id) = game_info.nth(0)
                && let Some(id_str) = game_id.split(' ').nth(1)
                && let Ok(id) = id_str.parse::<u32>()
                && let Some(sets) = game_info.nth(0)
                && is_game_possible(sets)
            {
                result += id;
            }
        }

        result.to_string()
    }

    fn run_part2(&self, input: &[String]) -> String {
        let mut result = 0;
        for line in input {
            let mut game_info = line.split(": ");
            if let Some(sets) = game_info.nth(1) {
                result += min_cubes_in_game(sets);
            }
        }

        result.to_string()
    }
}

fn is_game_possible(sets: &str) -> bool {
    for set in sets.split("; ") {
        let cubes = set.split(", ");
        for cube in cubes {
            let mut section = cube.split(' ');
            if let Some(count_str) = section.next()
                && let Ok(count) = count_str.parse::<u32>()
                && let Some(color_str) = section.next()
            {
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

    true
}

fn min_cubes_in_game(sets: &str) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in sets.split("; ") {
        let cubes = set.split(", ");
        for cube in cubes {
            let mut section = cube.split(' ');
            if let Some(count_str) = section.next()
                && let Ok(count) = count_str.parse::<u32>()
                && let Some(color_str) = section.next()
            {
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

    red * green * blue
}
