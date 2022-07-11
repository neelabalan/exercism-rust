use std::collections::HashMap;

const SEQUENCES: &'static [char] = &['G', 'A', 'C', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !dna.chars().all(|x| SEQUENCES.contains(&x)) || !SEQUENCES.contains(&nucleotide){
        return Err('X')
    }
    else {
        return Ok(dna.chars().filter(|x| x==&nucleotide).count());
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nuc_counts = HashMap::new();
    for ch in ['G', 'A', 'C', 'T'] {
        match count(ch, dna) {
            Ok(val) => nuc_counts.insert(ch, val),
            Err(val) => return Err(val),
        };
    }
    return Ok(nuc_counts);
}
