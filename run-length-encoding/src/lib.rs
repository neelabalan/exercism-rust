pub fn encode(source: &str) -> String {
    let mut final_string = String::from("");
    let mut position: Option<usize> = Some(0);
    let mut last = source;

    loop {
        if last.len() == 0 {
            break;
        }
        (_, last) = last.split_at(position.unwrap());
        let ch = last.chars().nth(0).unwrap();
        position = last.chars().position(|x| x != ch);
        match position {
            Some(val) => {
                if val > 1 {
                    final_string.push_str(&val.to_string());
                }
                final_string.push_str(&ch.to_string());
            }
            None => {
                if last.len() > 1 {
                    final_string.push_str(&last.len().to_string());
                }
                final_string.push_str(&ch.to_string());
                break;
            }
        }
    }
    return final_string;
}

pub fn decode(source: &str) -> String {
    let mut final_string: String = String::from("");
    let mut num: String = String::from("");
    for ch in source.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
        } 
        else if (num.is_empty()) && (ch.is_alphabetic() || ch.is_whitespace()) {
            final_string.push(ch);
        } 
        else if (!num.is_empty()) && (ch.is_alphabetic() || ch.is_whitespace()) {
            final_string.push_str(&ch.to_string().repeat(num.parse::<usize>().unwrap()));
            num=String::from("");
        }
    }
    return final_string;
}
