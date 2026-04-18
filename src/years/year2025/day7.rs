use std::collections::{HashMap, HashSet};

use crate::PuzzleParts;

pub struct Puzzle;

impl PuzzleParts for Puzzle {
    fn run_part1(&self, input: &[String]) -> String {
        let mut splits = 0;
        let mut beams = HashSet::<usize>::new();

        let Some(first_line) = input.first() else {
            panic!("No first line!");
        };

        let Some(start) = first_line.char_indices().find(|c| c.1 == 'S') else {
            panic!("Could not find 'S' in first line!");
        };

        beams.insert(start.0);

        for line in &input[1..] {
            for char_idx in line.char_indices() {
                if beams.contains(&char_idx.0) && char_idx.1 == '^' {
                    splits += 1;
                    beams.remove(&char_idx.0);

                    if char_idx.0 > 0 {
                        beams.insert(char_idx.0 - 1);
                    }

                    if char_idx.0 < line.len() - 1 {
                        beams.insert(char_idx.0 + 1);
                    }
                }
            }
        }

        splits.to_string()
    }

    fn run_part2(&self, input: &[String]) -> String {
        let mut beams = HashMap::<usize, u64>::new();

        let Some(first_line) = input.first() else {
            panic!("No first line!");
        };

        let Some(start) = first_line.char_indices().find(|c| c.1 == 'S') else {
            panic!("Could not find 'S' in first line!");
        };

        beams.insert(start.0, 1);

        for line in &input[1..] {
            for char_idx in line.char_indices() {
                if beams.contains_key(&char_idx.0) && char_idx.1 == '^' {
                    let Some(current_beam_count) = beams.remove(&char_idx.0) else {
                        panic!("A beam should exist here!");
                    };

                    if char_idx.0 > 0 {
                        let new_idx = char_idx.0 - 1;
                        let new_count = if let Some(count) = beams.get(&new_idx) {
                            count + current_beam_count
                        } else {
                            current_beam_count
                        };

                        beams.insert(new_idx, new_count);
                    }

                    if char_idx.0 < line.len() - 1 {
                        let new_idx = char_idx.0 + 1;
                        let new_count = if let Some(count) = beams.get(&new_idx) {
                            count + current_beam_count
                        } else {
                            current_beam_count
                        };

                        beams.insert(new_idx, new_count);
                    }
                }
            }
        }

        let timelines = beams.values().sum::<u64>();

        timelines.to_string()
    }
}
