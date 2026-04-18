use crate::PuzzleParts;

pub struct Puzzle;

impl PuzzleParts for Puzzle {
    fn run_part1(&self, input: &[String]) -> String {
        let mut result = 0;
        let mut x = vec!['.'; 10 * 10];

        for (i, line) in input.iter().enumerate() {
            for (j, char) in line.char_indices() {
                x[i * j] = char;
            }
        }

        for c in x.iter().take(10 * 10) {
            if let Some(d) = c.to_digit(10) {
                result += d;
            }
        }

        result.to_string()
    }

    fn run_part2(&self, input: &[String]) -> String {
        for _ in input {}

        "".to_owned()
    }
}
