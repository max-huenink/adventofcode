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
    let mut duplicates_seen = Vec::<std::ops::Range<u64>>::new();
    for idx in 0..fresh_id_ranges.len() {
        let range = &fresh_id_ranges[idx];
        println!("Range {range:?}");

        let mut ranges_to_check: [&[std::ops::Range<u64>]; 2] = [&[], &[]];

        if idx > 0 {
            ranges_to_check[0] = &fresh_id_ranges[..idx];
        }

        if (idx + 1) < fresh_id_ranges.len() {
            ranges_to_check[1] = &fresh_id_ranges[(idx + 1)..];
        }

        let all_ranges_to_check = ranges_to_check.iter().flat_map(|r| r.iter());

        let count_duplicate_ranges = all_ranges_to_check
            .clone()
            .filter(|r| r.start == range.start && r.end == range.end)
            .count();
        let already_seen_duplicate = duplicates_seen
            .clone()
            .into_iter()
            .filter(|r| r.start == range.start && r.end == range.end)
            .count()
            == 1;

        if count_duplicate_ranges > 0 && !already_seen_duplicate {
            duplicates_seen.push(range.clone());
            count += range.clone().count() as i64;
        }

        if all_ranges_to_check
            .clone()
            .any(|r| r.contains(&range.start) && (r.contains(&range.end) || r.end == range.end))
        {
            println!("\tSkipping {range:?}");
            continue;
        }

        println!("\tAdding {} to count ({count})", { range.clone().count() });
        count += range.clone().count() as i64;

        for check in all_ranges_to_check {
            println!("\tChecking {check:?} against {range:?}");
            if range.contains(&check.start)
                && !(range.contains(&check.end) || range.end == check.end)
            {
                println!(
                    "\tSubtracting {} from count ({count})",
                    (check.start..range.end).count()
                );
                count -= (check.start..range.end).count() as i64;
            }
        }
    }

    println!("{count}");
}
