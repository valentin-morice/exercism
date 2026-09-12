use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let addends: Vec<&str> = left.split('+').map(str::trim).collect();
    let result = right.trim();

    let words: Vec<&str> = addends.iter().copied().chain(std::iter::once(result)).collect();

    let leading_letters: HashSet<char> = words.iter().map(|word| word.chars().next().unwrap()).collect();

    let mut seen = HashSet::new();
    let letters: Vec<char> = words
        .iter()
        .flat_map(|word| word.chars())
        .filter(|c| seen.insert(*c))
        .collect();

    let mut assignment = HashMap::new();
    let mut used_digits = [false; 10];

    backtrack(&letters, &addends, result, &leading_letters, &mut assignment, &mut used_digits)
        .then_some(assignment)
}

fn backtrack(
    remaining: &[char],
    addends: &[&str],
    result: &str,
    leading_letters: &HashSet<char>,
    assignment: &mut HashMap<char, u8>,
    used_digits: &mut [bool; 10],
) -> bool {
    let Some((&letter, rest)) = remaining.split_first() else {
        let sum: u64 = addends.iter().map(|word| word_value(word, assignment)).sum();
        return sum == word_value(result, assignment);
    };

    (0..=9u8).any(|digit| {
        if used_digits[digit as usize] || (digit == 0 && leading_letters.contains(&letter)) {
            return false;
        }

        used_digits[digit as usize] = true;
        assignment.insert(letter, digit);

        let solved = backtrack(rest, addends, result, leading_letters, assignment, used_digits);
        if !solved {
            used_digits[digit as usize] = false;
            assignment.remove(&letter);
        }

        solved
    })
}

fn word_value(word: &str, assignment: &HashMap<char, u8>) -> u64 {
    word.chars().fold(0, |number, c| number * 10 + assignment[&c] as u64)
}
