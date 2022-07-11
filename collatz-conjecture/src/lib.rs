// Take any positive integer n. If n is even, divide n by 2 to get n / 2.
// If n is odd, multiply n by 3 and add 1 to get 3n + 1
pub fn collatz(n: u64) -> Option<u64> {
    let mut m = n;
    let mut steps = 0;
    loop {
        match m {
            0 => break,
            1 => return Some(steps),
            _ if m%2==0_ => m/=2,
            _ => {
                let result = m.checked_mul(3).and_then(|x| x.checked_add(1));
                if result.is_none() {
                    break
                }
                else {
                    m = result.unwrap();
                }
            } 
        }
        steps+=1;
    }
    return None;
}
