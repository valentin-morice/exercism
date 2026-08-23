pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let len = digits.len() as u32;

    let sum: u64 = digits
        .chars()
        .map(|c| u64::from(c.to_digit(10).unwrap()).pow(len))
        .sum();

    sum == u64::from(num)
}
