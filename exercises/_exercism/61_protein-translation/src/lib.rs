pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.as_bytes()
        .chunks(3)
        .map_while(|code| {
            Codon::new(code)
                .map(|codon| codon.translate_from_std_table().map(|aa| aa.into()))
                .transpose()
        })
        .collect::<Result<Vec<&str>, Error>>()
        .ok()
}

enum Error {
    InvalidNucleotideCode,
    InvalidSequenceLenght,
}

enum RnaNucleotide {
    A,
    G,
    C,
    U,
}

impl TryFrom<u8> for RnaNucleotide {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        todo!()
    }
}
struct Codon(Vec<RnaNucleotide>);

impl Codon {
    fn new(code: &[u8]) -> Result<Self, Error> {
        todo!()
    }

    fn translate_from_std_table(&self) -> Option<AminoAcid> {
        todo!()
    }
}
enum AminoAcid {}

impl From<AminoAcid> for &str {
    fn from(value: AminoAcid) -> Self {
        todo!()
    }
}
