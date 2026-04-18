use crate::PuzzleParts;

pub struct Puzzle;

impl PuzzleParts for Puzzle {
    fn run_part1(&self, input: &[String]) -> String {
        let mut sections = input.split(|line| line.trim().is_empty());
        let Some(fresh_ranges_str) = sections.next() else {
            panic!("No fresh ingredient ranges present!");
        };
        let Some(available_ids) = sections.next() else {
            panic!("NO available ingredient IDs present!");
        };

        let mut fresh_id_ranges = Vec::<std::ops::Range<u64>>::new();
        for range_str in fresh_ranges_str {
            let mut range = range_str
                .split('-')
                .filter_map(|i| i.trim().parse::<u64>().ok());
            if let Some(first) = range.next()
                && let Some(second) = range.next()
            {
                fresh_id_ranges.push(first..(second + 1));
            }
        }

        let mut count = 0;
        for id in available_ids
            .iter()
            .filter_map(|i| i.trim().parse::<u64>().ok())
        {
            if fresh_id_ranges.iter().any(|r| r.contains(&id)) {
                count += 1;
            }
        }

        count.to_string()
    }

    fn run_part2(&self, input: &[String]) -> String {
        let mut sections = input.split(|line| line.trim().is_empty());
        let Some(fresh_ranges_str) = sections.next() else {
            panic!("No fresh ingredient ranges present!");
        };

        let mut fresh_id_ranges = Vec::<std::ops::Range<u64>>::new();
        for range_str in fresh_ranges_str {
            let mut range = range_str
                .split('-')
                .filter_map(|i| i.trim().parse::<u64>().ok());
            if let Some(first) = range.next()
                && let Some(second) = range.next()
            {
                fresh_id_ranges.push(first..(second + 1));
            }
        }

        let mut count = 0;
        let mut duplicates_seen = Vec::<std::ops::Range<u64>>::new();
        for idx in 0..fresh_id_ranges.len() {
            let range = &fresh_id_ranges[idx];

            let mut ranges_to_check: [&[std::ops::Range<u64>]; 2] = [&[], &[]];

            if idx > 0 {
                ranges_to_check[0] = &fresh_id_ranges[..idx];
            }

            if (idx + 1) < fresh_id_ranges.len() {
                ranges_to_check[1] = &fresh_id_ranges[(idx + 1)..];
            }

            let all_ranges_to_check = ranges_to_check.iter().flat_map(|r| r.iter());

            if duplicates_seen
                .clone()
                .into_iter()
                .filter(|r| r.start == range.start && r.end == range.end)
                .count()
                == 1
            {
                continue;
            }

            if all_ranges_to_check
                .clone()
                .filter(|r| r.start == range.start && r.end == range.end)
                .count()
                > 0
            {
                duplicates_seen.push(range.clone());
            }

            if all_ranges_to_check
                .clone()
                .any(|r| r.contains(&range.start) && r.contains(&range.end))
            {
                continue;
            }

            count += range.clone().count() as i64;

            let mut end = 0;
            for check in all_ranges_to_check {
                if check.contains(&range.start)
                    && (!(range.contains(&check.start)
                        && (range.contains(&check.end) || range.end == check.end)))
                    && check.end > end
                {
                    end = check.end;
                }
            }

            if end > 0 {
                count -= (range.start..end).count() as i64;
            }
        }

        count.to_string()
    }
}
