pub fn get_bottle_str(n: u32) -> String {
    if n == 1 {
        return String::from("bottle")
    }
    else {
        return String::from("bottles")
    }
}
pub fn count_str(n: u32) -> String {
    if n > 0 {
        return n.to_string();
    }
    else {
        return String::from("no more")
    }
}
pub fn verse(n: u32) -> String {
    if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    let first_line = format!(
        "{start} {bottle} of beer on the wall, {start} {bottle} of beer.",
        start = count_str(n),
        bottle = get_bottle_str(n)
    );
    let second_line = format!(
        "\nTake {what} down and pass it around, {end} {bottle} of beer on the wall.\n",
        end = count_str(n-1),
        bottle = get_bottle_str(n-1),
        what = if n == 1 {
            String::from("it")
        } else {
            String::from("one")
        }
    );
    return format!("{}{}", first_line, second_line);
}

pub fn sing(start: u32, end: u32) -> String {
    let mut final_song = vec![];
    for n in (end..=start).rev() {
        final_song.push(verse(n));
    }
    return final_song.join("\n");
}
