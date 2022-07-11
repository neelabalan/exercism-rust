// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut occurence_note: HashMap<String, usize> = HashMap::new();
    let mut occurence_magazine: HashMap<String, usize> = HashMap::new();
    for val in note {
        *occurence_note.entry(val.to_string()).or_default() += 1;
    }
    for val in magazine {
        *occurence_magazine.entry(val.to_string()).or_default() += 1;
    }
    let mut return_val = true;
    for (val, count) in &occurence_note {
        match occurence_magazine.get(val) {
            Some(value) => {
                if value < count {
                    return_val = false;
                }
            },
            None => return_val = false
        }
    }
    // println!("{:?}", occurence_note);
    // println!("{:?}", occurence_magazine);
    return return_val;
}
