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
    fn new(codes: &[u8]) -> Result<Self, Error> {
        if codes.len() != 3 {
            return Err(Error::InvalidSequenceLength);
        }
        Ok(Self(
            codes
                .iter()
                .map(|&b| RnaNucleotide::try_from(b))
                .collect::<Result<Vec<RnaNucleotide>, Error>>()?,
        ))
    }

    fn translate_from_std_table(&self) -> Option<AminoAcid> {
        let aa = match (&self.0[0], &self.0[1], &self.0[2]) {
            (G, C, _) => Ala,
            (C, G, _) | (A, G, A) | (A, G, G) => Arg,
            (A, A, U) | (A, A, C) => Asn,
            (G, A, U) | (G, A, C) => Asp,
            (U, G, U) | (U, G, C) => Cys,
            (C, A, A) | (C, A, G) => Gln,
            (G, A, A) | (G, A, G) => Glu,
            (G, G, _) => Gly,
            (C, A, U) | (C, A, C) => His,
            (A, U, U) | (A, U, C) | (A, U, A) => Ile,
            (C, U, _) | (U, U, A) | (U, U, G) => Leu,
            (A, A, A) | (A, A, G) => Lys,
            (A, U, G) => Met,
            (U, U, U) | (U, U, C) => Phe,
            (C, C, _) => Pro,
            (U, C, _) | (A, G, U) | (A, G, C) => Ser,
            (A, C, _) => Thr,
            (U, G, G) => Trp,
            (U, A, U) | (U, A, C) => Tyr,
            (G, U, _) => Val,
            (U, A, A) | (U, G, A) | (U, A, G) => return None,
        };
        Some(aa)
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
