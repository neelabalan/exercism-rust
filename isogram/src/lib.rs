use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let filtered_candidate = candidate
        .to_ascii_lowercase()
        .chars()
        .filter(|&x| x.is_alphabetic())
        .collect::<String>();
    let chars = filtered_candidate.chars().collect::<HashSet<_>>();
    return chars.len() == filtered_candidate.len();
}
