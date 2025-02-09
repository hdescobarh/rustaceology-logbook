#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NUCLEOTIDE {
    A,
    G,
    C,
    T,
    U,
}

impl TryFrom<char> for NUCLEOTIDE {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'a' => Ok(Self::A),
            'G' | 'g' => Ok(Self::G),
            'C' | 'c' => Ok(Self::C),
            'T' | 't' => Ok(Self::T),
            'U' | 'u' => Ok(Self::U),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    chain: Vec<NUCLEOTIDE>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    chain: Vec<NUCLEOTIDE>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut chain = Vec::with_capacity(dna.len());
        for (index, c) in dna.chars().enumerate() {
            match NUCLEOTIDE::try_from(c) {
                Ok(NUCLEOTIDE::U) | Err(_) => return Err(index),
                Ok(n) => chain.push(n),
            };
        }
        Ok(Self { chain })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            chain: self
                .chain
                .iter()
                .map(|n| match n {
                    NUCLEOTIDE::A => NUCLEOTIDE::U,
                    NUCLEOTIDE::G => NUCLEOTIDE::C,
                    NUCLEOTIDE::C => NUCLEOTIDE::G,
                    _ => NUCLEOTIDE::A,
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut chain = Vec::with_capacity(rna.len());
        for (index, c) in rna.chars().enumerate() {
            match NUCLEOTIDE::try_from(c) {
                Ok(NUCLEOTIDE::T) | Err(_) => return Err(index),
                Ok(n) => chain.push(n),
            };
        }
        Ok(Self { chain })
    }
}
