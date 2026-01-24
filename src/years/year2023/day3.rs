use crate::PuzzleParts;

pub struct Puzzle;

impl PuzzleParts for Puzzle {
    fn run_part1(&self, input: &[String]) -> String {
        let mut result = 0;
        let mut x = vec!['.'; 10 * 10];
        let mut i = 0;

        for line in input {
            for (j, char) in line.char_indices() {
                x[i * j] = char;
            }
            i = i + 1;
        }

        for i in 0..(10 * 10) {
            let c = x[i];
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
