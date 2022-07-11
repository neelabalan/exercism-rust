#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn find_multiples_sum(num: u64) -> u64 {
    return (1..num).filter(|x| num%x==0).sum();
}

pub fn classify(num: u64) -> Option<Classification> {
    let sum = find_multiples_sum(num);
    match (sum, num) {
        (_, 0) => None,
        _ if sum>num => Some(Classification::Abundant),
        _ if num>sum => Some(Classification::Deficient),
        _ if num==sum => Some(Classification::Perfect),
        _ => None
    }

}
