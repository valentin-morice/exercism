pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, square)| match square {
                    b'*' => '*',
                    _ => match adjacent_flowers(garden, row, col) {
                        0 => ' ',
                        count => (b'0' + count) as char,
                    },
                })
                .collect()
        })
        .collect()
}

fn adjacent_flowers(garden: &[&str], row: usize, col: usize) -> u8 {
    garden[row.saturating_sub(1)..(row + 2).min(garden.len())]
        .iter()
        .flat_map(|line| {
            let bytes = line.as_bytes();
            &bytes[col.saturating_sub(1)..(col + 2).min(bytes.len())]
        })
        .filter(|&&square| square == b'*')
        .count() as u8
}
