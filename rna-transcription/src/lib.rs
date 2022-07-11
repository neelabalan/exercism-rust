#[derive(Debug, PartialEq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    sequence: String,
}

fn is_invalid_sequence(sequence: &str, possible_values: &[char]) -> Option<usize> {
    return sequence.chars().position(|x| !possible_values.contains(&x));
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match is_invalid_sequence(dna, &['G', 'C', 'A', 'T']) {
            Some(val) => Err(val),
            None => Ok(Dna {
                sequence: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> Rna {
        return Rna {
            sequence: self
                .sequence
                .chars()
                .map(|x| match x {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => ' ',
                })
                .collect::<String>(),
        };
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match is_invalid_sequence(rna, &['G', 'C', 'A', 'U']) {
            Some(val) => Err(val),
            None => Ok(Rna {
                sequence: rna.to_string(),
            }),
        }
    }
}
/*
G -> C
C -> G
T -> A
A -> U
 */
