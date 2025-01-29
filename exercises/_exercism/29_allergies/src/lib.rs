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

impl TryFrom<u32> for Allergen {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let result = match value {
            0 => Self::Eggs,
            1 => Self::Peanuts,
            2 => Self::Shellfish,
            3 => Self::Strawberries,
            4 => Self::Tomatoes,
            5 => Self::Chocolate,
            6 => Self::Pollen,
            7 => Self::Cats,
            _ => return Err("Non valid code"),
        };

        Ok(result)
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergens = Self::decompose_score(score)
            .iter()
            .filter_map(|&value| Allergen::try_from(value).ok())
            .collect();
        Self { score, allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        todo!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }

    fn decompose_score(score: u32) -> Vec<u32> {
        let mut result = Vec::new();
        let mut remaining_score = if score % 2 != 0 {
            result.push(0);
            score - 1
        } else {
            score
        };

        let mut two_as_factor = 0;
        while remaining_score > 0 {
            if remaining_score % 2 == 0 {
                two_as_factor += 1;
                remaining_score /= 2;
            } else if remaining_score == 1 {
                result.push(two_as_factor);
                remaining_score = 0;
            } else {
                result.push(two_as_factor);
                remaining_score -= 1;
            }
        }

        result
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
