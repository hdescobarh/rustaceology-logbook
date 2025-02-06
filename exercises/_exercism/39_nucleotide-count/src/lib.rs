use std::collections::HashMap;

const DNA_NUCLEOTIDES: [char; 4] = ['A', 'G', 'C', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut result = if DNA_NUCLEOTIDES.contains(&nucleotide) {
        0
    } else {
        return Err(nucleotide);
    };
    for c in dna.chars() {
        if c == nucleotide {
            result += 1
        } else if !DNA_NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
    }
    Ok(result)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::from_iter(DNA_NUCLEOTIDES.iter().map(|&c| (c, 0)));
    for c in dna.chars() {
        match result.get_mut(&c) {
            Some(d) => *d += 1,
            None => return Err(c),
        }
    }
    Ok(result)
}
