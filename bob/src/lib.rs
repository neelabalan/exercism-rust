pub fn is_upper_and_alphabetic(message: String) -> bool {
    let mut iter = message.chars().filter(|x| x.is_ascii_alphabetic());
    if iter.clone().count() > 0 && iter.all(|x| x.is_uppercase()) {
        return true;
    }
    else {
        return false;
    }
}
pub fn reply(message: &str) -> &str {
    let fil_message: String= message.chars().filter(|&x| !x.is_whitespace()).collect();

    if fil_message.ends_with('?') {
        if is_upper_and_alphabetic(fil_message) {
            return "Calm down, I know what I'm doing!";
        }
        else {
            return "Sure."
        }
    } 
    else if fil_message.chars().all(|x| x.is_whitespace()) {
        return "Fine. Be that way!";
    } 
    else if is_upper_and_alphabetic(fil_message) {
        return "Whoa, chill out!";
    }
    else {
        return "Whatever.";
    }
}
