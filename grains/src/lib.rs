pub fn square(s: u32) -> u64 {
    if s>0 && s<65 {
        return 2_u64.pow(s-1);
    }
    else {
        panic!("Square must be between 1 and 64");
    }
}

pub fn total() -> u64 {
    return (1..=64).map(|x| square(x)).fold(0, |a, b| a+b);
}
