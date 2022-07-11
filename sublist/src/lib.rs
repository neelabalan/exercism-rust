#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// could be a match statement (refactoring required)
// #![feature(array_windows)]
pub fn sublist<T: PartialEq+ PartialOrd>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let f_len = _first_list.len();
    let s_len = _second_list.len();
    if f_len == s_len && _first_list.eq(_second_list) {
        return Comparison::Equal;
    }
    else if f_len > s_len {
        if s_len == 0 {
            return Comparison::Superlist;
        }
        if _first_list.windows(s_len).any(|x| x.eq(_second_list)) {
            return Comparison::Superlist;
        }
        else {
            return Comparison::Unequal;
        }
    }
    else if f_len < s_len {
        if f_len == 0 {
            return Comparison::Sublist;
        }
        else if _second_list.windows(f_len).any(|x| x.eq(_first_list)) {
            return Comparison::Sublist;
        }
        else {
            return Comparison::Unequal;
        }
    }
    else {
        return Comparison::Unequal;
    }
}
