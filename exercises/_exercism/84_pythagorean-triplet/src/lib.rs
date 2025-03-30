use std::collections::HashSet;

// From solving the (in)equations: 3 < small_leg < sum * (2 - sqrt(2)) / 2
const CONSTRAINT: f64 = 0.2928932188134524;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // From solving the equations: If sum (perimeter) is odd, then one leg is not an integer.
    if sum % 2 != 0 {
        return HashSet::new();
    }
    (3..(sum as f64 * CONSTRAINT).ceil() as u32)
        .filter_map(|small_leg| {
            PythagoreanTriangle::new(sum, small_leg).map(|triangle| triangle.to_slice())
        })
        .collect()
}

struct PythagoreanTriangle {
    small_leg: u32,
    large_leg: u32,
    hypothenuse: u32,
}

impl PythagoreanTriangle {
    pub fn new(perimeter: u32, small_leg: u32) -> Option<PythagoreanTriangle> {
        // sqrt(x**2 + y**2) = p - x - y, 0 < x < y < a
        Self::find_large_leg(small_leg, perimeter).map(|large_leg| Self {
            small_leg,
            large_leg,
            hypothenuse: perimeter - small_leg - large_leg,
        })
    }

    fn find_large_leg(small_leg: u32, perimeter: u32) -> Option<u32> {
        // y = p * (p - 2x) / (2p - 2x)
        let y_numerator = perimeter * (perimeter - 2 * small_leg);
        let y_denominator = 2 * (perimeter - small_leg);
        (y_numerator % y_denominator == 0).then_some(y_numerator / y_denominator)
    }

    pub fn to_slice(&self) -> [u32; 3] {
        [self.small_leg, self.large_leg, self.hypothenuse]
    }
}
