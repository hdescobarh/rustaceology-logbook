use std::collections::HashSet;

// From solving the (in)equations: 5 < small_leg < sum * (2-sqrt(2))/2
const CONSTRAINT: f64 = 0.2928932188134524;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (5..(sum as f64 * CONSTRAINT).ceil() as u32)
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
        let large_leg = Self::get_large_leg(small_leg, perimeter)?;
        let hypothenuse = Self::find_integer_hypotenuse(small_leg, large_leg)?;
        Some(Self {
            small_leg,
            large_leg,
            hypothenuse,
        })
    }

    fn get_large_leg(small_leg: u32, perimeter: u32) -> Option<u32> {
        todo!()
    }

    fn find_integer_hypotenuse(small_leg: u32, large_leg: u32) -> Option<u32> {
        todo!()
    }

    pub fn to_slice(self) -> [u32; 3] {
        [self.small_leg, self.large_leg, self.hypothenuse]
    }
}
