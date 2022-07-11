pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut factor = 2;
    let mut n = n;
    while factor * factor <= n {
        if n % factor != 0 {
            factor += 1
        }
        else {
            n = n/factor;
            factors.push(factor)
        }
    }
    if n > 1 {
        factors.push(n);
    }
    return factors;
}