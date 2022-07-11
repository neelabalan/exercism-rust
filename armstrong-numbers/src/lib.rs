pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.to_string().len();
    let mut number = 0;
    for val in num.to_string().chars() {
        number += val.to_string().parse::<u32>().unwrap().pow(num_len as u32);
    }
    return number == num;
}
