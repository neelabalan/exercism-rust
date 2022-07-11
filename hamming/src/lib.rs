/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut distance: usize = 0;
    if s1.len() != s2.len() {
        return None;
    }
    else {
        let mut s2_chars = s2.chars();
        for value in s1.chars() {
            if value != s2_chars.next().unwrap() {
                distance+=1;
            }
            else {}
        }
    }
    return Some(distance);
}

// better solution using zip
/*

pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { None }
    else {
        let distance = s1
            .chars()
            .zip(s2.chars())
            .filter(|(s1_c, s2_c)| s1_c != s2_c)
            .count();
        Some(distance)
    }
}
*/
