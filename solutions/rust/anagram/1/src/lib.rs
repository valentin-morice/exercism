use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower = word.to_lowercase();
    let mut a = lower.chars().collect::<Vec<char>>();
    a.sort_unstable();

    let mut result: HashSet<&str> = HashSet::new();

    for anagram in possible_anagrams {
        let candidate = anagram.to_lowercase();

        if candidate == lower {
            continue;
        }

        let mut b = candidate.chars().collect::<Vec<char>>();
        b.sort_unstable();

        if a == b {
            result.insert(anagram);
        }
    }

    result
}
