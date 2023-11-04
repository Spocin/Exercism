pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let mut width = minefield[0].len();

    let mut computed_minefield: Vec<Vec<i32>> = vec![vec![0; width]; height];

    for (y, row) in minefield.iter().enumerate() {
        for (x, field) in row.chars().enumerate() {
            if field == '*' {
                increment_around_bomb(x, y, &mut computed_minefield);
            }
        }
    }

    return computed_minefield
        .iter()
        .map(|row|
            row.iter()
                .map(|field|
                    match field {
                        &field if field == -1 => '*'.to_string(),
                        &field if field == 0 => '0'.to_string(),
                        &field => field.to_string()
                    }
                ).collect()
        )
        .collect();
}

fn increment_around_bomb(x: usize, y: usize, minefield: &mut Vec<Vec<i32>>) {
    if y == 0 {
        increment_row(x, &mut minefield[y]);
        increment_row(x, &mut minefield[y+1]);
        return;
    }

    if y == minefield.len() {
        increment_row(x, &mut minefield[y-1]);
        increment_row(x, &mut minefield[y]);
        return;
    }

    increment_row(x, &mut minefield[y-1]);
    increment_row(x, &mut minefield[y]);
    increment_row(x, &mut minefield[y+1]);
}

fn increment_row(x: usize, row: &mut Vec<i32>) {
    if x == 0 {
        for field in x..x+1 {
            if row[field] != -1 {
                row[field] += 1;
            }
        }
        return;
    }

    if x == row.len() {
        for field in x-1..x {
            if row[field] != -1 {
                row[field] += 1;
            }
        }
        return;
    }

    for field in x - 1..x + 1 {
        if row[field] != -1 {
            row[field] += 1;
        }
    }
}