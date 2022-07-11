#[derive(Debug)]
pub struct Position(i8, i8);

pub fn is_mine_at_position(minefield: &[&str], position: Position) -> bool {
    if position.0.is_negative()
        || position.1.is_negative()
        || position.0 > (minefield.len()-1) as i8
        || position.1 > (minefield[0].len()-1) as i8
    {
        return false;
    }
    if minefield[position.0 as usize].as_bytes()[position.1 as usize] as char == '*' {
        return true;
    } else {
        return false;
    }
}

pub fn mines_around(minefield: &[&str], position: Position) -> u8 {
    let mut num = 0;
    if is_mine_at_position(minefield, Position(position.0 - 1, position.1 + 1)) {
        // upper right
        num += 1;
    }
    if is_mine_at_position(minefield, Position(position.0 + 1, position.1 - 1)) {
        // lower left
        num += 1;
    }
    if is_mine_at_position(minefield, Position(position.0 - 1, position.1 - 1)) {
        // upper left
        num += 1;
    }
    if is_mine_at_position(minefield, Position(position.0, position.1 - 1)) {
        // left
        num += 1;
    }
    if is_mine_at_position(minefield, Position(position.0 - 1, position.1)) {
        // top
        num += 1;
    }
    if is_mine_at_position(minefield, Position(position.0 + 1, position.1 + 1)) {
        // lower right
        num += 1;
    }

    if is_mine_at_position(minefield, Position(position.0, position.1 + 1)) {
        // right
        num += 1;
    }
    if is_mine_at_position(minefield, Position(position.0 + 1, position.1)) {
        // bottom
        num += 1;
    }

    return num;
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut sweeped = vec![];
    for (i, line) in minefield.iter().enumerate() {
        let mut str_line = String::from("");
        for (j, ch) in line.chars().enumerate() {
            if ch == '*' {
                str_line.push('*');
                continue;
            }
            let number = mines_around(minefield, Position(i as i8, j as i8));
            if number == 0 {
                str_line.push(' ');
            } else {
                str_line.push(number.to_string().chars().nth(0).unwrap());
            }
            // println!("{}, {}, {}", i, j, ch);
            // println!("{}",val[i].as_bytes()[j] as char);
        }
        sweeped.push(str_line);
    }
    return sweeped;
}
