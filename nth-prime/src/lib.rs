pub fn nth(n: u32) -> u32 {
    let mut prime_list = vec![];
    let mut counter = 1;
    while prime_list.len() != (n+1) as usize {
        if is_prime(counter) {
            prime_list.push(counter);
        }
        counter += 1;
    }
    return prime_list[n as usize];
}

pub fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n == 1 {
        return false;
    }
    let mut factor = 2;
    while factor < n {
        if n%factor == 0 {
            return false
        }
        factor += 1;
    }
    return true; 
}

#[test]
#[ignore]
fn is_one_prime() {
    assert_eq!(is_prime(1), false);
}

