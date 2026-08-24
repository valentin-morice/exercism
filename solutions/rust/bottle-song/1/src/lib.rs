const NUMBERS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn bottles(count: u32) -> String {
    let number = NUMBERS[count as usize];
    let noun = if count == 1 { "bottle" } else { "bottles" };
    format!("{number} green {noun}")
}

fn verse(count: u32) -> String {
    let hanging = format!("{} hanging on the wall,\n", bottles(count));
    let capitalized = hanging[..1].to_uppercase() + &hanging[1..];
    format!(
        "{capitalized}{capitalized}\
         And if one green bottle should accidentally fall,\n\
         There'll be {} hanging on the wall.\n",
        bottles(count - 1)
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (start_bottles + 1 - take_down..=start_bottles)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
