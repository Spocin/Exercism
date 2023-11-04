pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };

    let mut computed_minefield: Vec<Vec<i32>> = vec![vec![0; width]; height];

    for (y, row) in minefield.iter().enumerate() {
        for (x, field) in row.chars().enumerate() {
            if field == '*' {
                computed_minefield[y][x] = -1;
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
                        &field if field == 0 => ' '.to_string(),
                        &field => field.to_string()
                    }
                ).collect()
        )
        .collect();
}

fn increment_around_bomb(x: usize, y: usize, computed_minefield: &mut Vec<Vec<i32>>) {
    if y == 0 {
        increment_row(x, &mut computed_minefield[y]);
        increment_row(x, &mut computed_minefield[y+1]);
        return;
    }

    if y == computed_minefield.len() - 1 {
        increment_row(x, &mut computed_minefield[y-1]);
        increment_row(x, &mut computed_minefield[y]);
        return;
    }

    increment_row(x, &mut computed_minefield[y-1]);
    increment_row(x, &mut computed_minefield[y]);
    increment_row(x, &mut computed_minefield[y+1]);
}

fn increment_row(x: usize, row: &mut Vec<i32>) {
    if x == 0 {
        increment_field(&mut row[x]);
        increment_field(&mut row[x+1]);
        return;
    }

    if x == row.len() - 1 {
        increment_field(&mut row[x-1]);
        increment_field(&mut row[x]);
        return;
    }

    increment_field(&mut row[x-1]);
    increment_field(&mut row[x]);
    increment_field(&mut row[x+1]);
}

fn increment_field(row: &mut i32) {
    if *row != -1 {
        *row += 1;
    }
}