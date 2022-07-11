use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for factor in factors {
        if *factor == 0 {
            break
        }
        let mut num = 0;
        while factor*num < limit {
            multiples.insert(factor*num);
            num += 1;
            // println!("{} - {}", num, limit);
        }
    }
    return multiples.iter().fold(0, |a, b| a+b);
}
