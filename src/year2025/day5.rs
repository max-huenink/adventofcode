pub fn run_part1(input: Vec<String>) {
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

    println!("{count}");
}

pub fn run_part2(input: Vec<String>) {
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
    for idx in 0..fresh_id_ranges.len() {
        let range = &fresh_id_ranges[idx];

        if idx > 0 {
            let before_ranges = &fresh_id_ranges[..idx];

            if before_ranges
                .iter()
                .any(|r| r.contains(&range.start) && r.contains(&range.end))
            {
                continue;
            }
            for before in before_ranges {
                if before.contains(&range.start) && !before.contains(&range.end) {
                    count -= (range.start..before.end).count() as i64;
                }
            }
        }

        if (idx + 1) < fresh_id_ranges.len() {
            let after_ranges = &fresh_id_ranges[(idx + 1)..];

            if after_ranges
                .iter()
                .any(|r| r.contains(&range.start) && r.contains(&range.end))
            {
                continue;
            }

            for after in after_ranges {
                if after.contains(&range.start) && !after.contains(&range.end) {
                    count -= (range.start..after.end).count() as i64;
                }
            }
        }

        count += range.clone().count() as i64;
    }

    println!("{count}");
}
