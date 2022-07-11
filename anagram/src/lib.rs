use std::collections::HashSet;

pub fn str_to_sorted_word(word: &str) -> String {
    let mut sorted_vec = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_vec.sort_unstable();
    return sorted_vec.iter().collect();
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let sorted_word = str_to_sorted_word(word);
    for anagram in possible_anagrams {
        let sorted_anagram = str_to_sorted_word(anagram);
        if !(word.to_lowercase() == anagram.to_lowercase()) && sorted_anagram == sorted_word {
            anagrams.insert(&anagram[..]);
        }
    }
    return anagrams;
}
