/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }
    let mut sum = 0;
    let filter_dashes: String = isbn.chars().filter(|&x| x != '-').collect();

    if (!filter_dashes.ends_with('X') && !filter_dashes.chars().last().unwrap().is_ascii_digit())
        || !(filter_dashes
            .chars()
            .filter(|&x| x != 'X')
            .all(|x| x.is_ascii_digit()))
        || (filter_dashes.len() != 10)
    {
        return false;
    }
    for (count, val) in filter_dashes.chars().enumerate() {
        match val {
            'X' => sum+=10,
            _ => sum+= val.to_digit(10).unwrap() * (10 - count as u32) // for rest of the digits
        }
    }
    return sum % 11 == 0;
}
