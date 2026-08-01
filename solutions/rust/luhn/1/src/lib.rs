/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let Some(digits): Option<Vec<u32>> = code
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect()
    else {
        return false;
    };

    if digits.len() <= 1 {
        return false;
    }

    let result: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 1 {
                let m = d * 2;

                if m > 9 { m - 9 } else { m }
            } else {
                *d
            }
        })
        .sum();

    result.is_multiple_of(10)
}
