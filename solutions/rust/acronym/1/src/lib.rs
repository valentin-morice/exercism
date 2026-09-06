pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(['-', '_', ' '])
        .filter(|word| !word.is_empty())
        .flat_map(split_camel_case)
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn split_camel_case(word: &str) -> Vec<&str> {
    let indices: Vec<(usize, char)> = word.char_indices().collect();
    let mut parts = Vec::new();
    let mut start = 0;

    for pair in indices.windows(2) {
        let (_, prev) = pair[0];
        let (idx, curr) = pair[1];
        if prev.is_lowercase() && curr.is_uppercase() {
            parts.push(&word[start..idx]);
            start = idx;
        }
    }

    parts.push(&word[start..]);
    parts
}
