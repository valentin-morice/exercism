#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (
        contains(second_list, first_list), // first is inside second
        contains(first_list, second_list), // second is inside first
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

fn contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle)
}
