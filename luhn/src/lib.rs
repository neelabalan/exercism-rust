/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let string = code
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<String>();
    println!("{}", string);
    let len = string.len();
    if len > 1 && string.chars().all(|x| x.is_ascii_digit()) {
        for (count, ch) in string.chars().rev().enumerate() {
            let val = ch.to_string().parse::<u8>().unwrap();
            match count % 2 {
                1 => if (val*2)>9 {sum += (val*2)-9} else {sum+= val*2},
                0 => sum+=val,
                _ => {}
            }
        }
        return if sum%10 == 0 {true} else {false};
    } 
    else {
        return false;
    }
}
