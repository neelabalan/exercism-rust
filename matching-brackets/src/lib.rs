pub fn brackets_are_balanced(string: &str) -> bool {
    pub fn get_matching_right_bracket(bracket: char) -> char {
        return match bracket {
            '}' => '{',
            ']' => '[',
            ')' => '(',
            _ => ' ',
        };
    }
    let mut stack = vec![];
    let left_brackets = "{[(";
    let right_brackets = "}])";

    let mut flag = true;
    for ch in string.chars() {
        if left_brackets.contains(ch) {
            stack.push(ch);
        } 
        else if right_brackets.contains(ch){
            let val = stack.pop();
            if let Some(k) = val {
                if get_matching_right_bracket(ch) != k {
                    flag = false
                } 
            }
            else {
                return false
            }
        }
        else {}
    }
    return if stack.is_empty() {flag} else {false};
}
