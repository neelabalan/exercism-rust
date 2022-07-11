use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    return input.to_string().graphemes(true).rev().collect();
}
