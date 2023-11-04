pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = minefield[0].len();

    let computed_minefield: Vec<Vec<i32>> = vec![vec![0; width]; height];

    for (y, row) in minefield.iter().enumerate() {
        for (x, field) in row.chars().enumerate() {
            if field == '*' {
                increment_around_bomb(&x, &y, &computed_minefield);
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

fn increment_around_bomb(x: &usize, y: &usize, minefield: &Vec<Vec<i32>>) {

}