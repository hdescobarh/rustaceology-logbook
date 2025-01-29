pub struct Allergies {
    score: u32,
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        todo!("Given the '{score}' score, construct a new Allergies struct.");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        todo!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }

    fn decompose_score(score: u32) -> Vec<u32> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::Allergies;

    #[test]
    fn get_allergen_code() {
        let tests = [
            (0, vec![]),
            (3, vec![0, 1]),
            (5, vec![0, 2]),
            (12, vec![2, 3]),
            (14, vec![1, 2, 3]),
            (255, vec![0, 1, 2, 3, 4, 5, 6, 7]),
            (257, vec![0, 8]),
            (509, vec![0, 2, 3, 4, 5, 6, 7, 8]),
            (1025, vec![0, 10]),
        ];
        for (input, expected) in tests {
            assert_eq!(expected, Allergies::decompose_score(input));
        }
    }
}
