pub fn abbreviate(phrase: &str) -> String {
    let punctuations = ['_', '-', ':'];
    let filtered_phrase: String = phrase.chars().map(|x| if punctuations.contains(&x) {' '} else {x}).collect();
    let collection: Vec<&str> = filtered_phrase.split(' ').collect();
    let mut abbr: String = String::from("");
    if phrase.len() > 0 {
        for string in collection {
            for (index, ch) in string.trim().chars().enumerate() {
                if string.chars().all(|x| x.is_uppercase()) {
                    abbr.push(ch);
                    break;
                }
                match (index, ch.is_uppercase()) {
                    (0, _) => abbr.push(ch),
                    (_, true) => abbr.push(ch),
                    (_, _) => {}

                }
            }
        }
    }
    return abbr.to_ascii_uppercase();
}
// println!("{}", abbr);
    // else if index == 0 {
        // abbr.push(ch);
        // abbr.push(*ch.to_uppercase().collect::<Vec<_>>().get(0).unwrap())
    // }

    // else if ch.is_uppercase(){
    //     abbr.push(ch)
    // }
    // else {}
