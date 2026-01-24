pub mod years;

pub trait PuzzleParts {
    fn run_part1(&self, input: &[String]) -> String;
    fn run_part2(&self, input: &[String]) -> String;
}
