pub fn raindrops(n: u32) -> String {
    let mut sounds = vec![];
    if n % 3 == 0 {
        sounds.push(String::from("Pling"));
    }
    if n % 5 == 0 {
        sounds.push(String::from("Plang"));
    }
    if n % 7 == 0 {
        sounds.push(String::from("Plong"));
    }
    return if sounds.is_empty() {n.to_string()} else {sounds.join("")};
}