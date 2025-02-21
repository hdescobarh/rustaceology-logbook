use AminoAcid::*;
use RnaNucleotide::*;

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
    InvalidSequenceLength,
}

enum RnaNucleotide {
    A,
    G,
    C,
    U,
}

impl TryFrom<u8> for RnaNucleotide {
    type Error = Error;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            b'a' | b'A' => Ok(Self::A),
            b'g' | b'G' => Ok(Self::G),
            b'c' | b'C' => Ok(Self::C),
            b'u' | b'U' => Ok(Self::U),
            _ => Err(Error::InvalidNucleotideCode),
        }
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
enum AminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
}

impl From<AminoAcid> for &str {
    fn from(value: AminoAcid) -> Self {
        match value {
            Ala => "Alanine",
            Arg => "Arginine",
            Asn => "Asparagine",
            Asp => "Aspartate",
            Cys => "Cysteine",
            Gln => "Glutamine",
            Glu => "Glutamate",
            Gly => "Glycine",
            His => "Histidine",
            Ile => "Isoleucine",
            Leu => "Leucine",
            Lys => "Lysine",
            Met => "Methionine",
            Phe => "Phenylalanine",
            Pro => "Proline",
            Ser => "Serine",
            Thr => "Threonine",
            Trp => "Tryptophan",
            Tyr => "Tyrosine",
            Val => "Valine",
        }
    }
}
