pub fn run_part1(input: &[String]) {
    let points = input
        .iter()
        .map(|line| line.trim().chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for row_idx in 0..points.len() {
        let row = &points[row_idx];
        for col_idx in 0..row.len() {
            let col = row[col_idx];
            if col && check_bounds(row_idx, col_idx, &points) {
                count += 1;
            }
        }
    }

    println!("{count}");
}

pub fn run_part2(input: &[String]) {
    let mut points = input
        .iter()
        .map(|line| line.trim().chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut points_found: Vec<(usize, usize)> = vec![];

    let mut sum = 0;
    let mut count = 1;
    while count != 0 {
        count = 0;

        for row_idx in 0..points.len() {
            let row = &points[row_idx];
            for col_idx in 0..row.len() {
                let col = &row[col_idx];
                if *col && check_bounds(row_idx, col_idx, &points) {
                    points_found.push((row_idx, col_idx));
                    count += 1;
                }
            }
        }

        sum += count;

        for p in &points_found {
            points[p.0][p.1] = false;
        }
        points_found.clear();
    }

    println!("{sum}");
}

fn check_bounds(start_row_idx: usize, start_col_idx: usize, points: &Vec<Vec<bool>>) -> bool {
    let mut count = 0;

    let row_start_range = match start_row_idx.checked_sub(1) {
        Some(v) => v,
        None => 0,
    };
    let col_start_range = match start_col_idx.checked_sub(1) {
        Some(v) => v,
        None => 0,
    };

    for row_idx in row_start_range..(start_row_idx + 2) {
        if row_idx < points.len() {
            let row = &points[row_idx];

            for col_idx in col_start_range..(start_col_idx + 2) {
                if col_idx < row.len() {
                    let point = row[col_idx];
                    if point && (row_idx != start_row_idx || col_idx != start_col_idx) {
                        count += 1;
                        if count >= 4 {
                            return false;
                        }
                    }
                }
            }
        }
    }

    count < 4
}
