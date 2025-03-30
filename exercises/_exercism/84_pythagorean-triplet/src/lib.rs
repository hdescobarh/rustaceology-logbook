use std::collections::HashSet;

// From solving the (in)equations: 3 < small_leg < sum * (2 - sqrt(2)) / 2
const CONSTRAINT: f64 = 0.2928932188134524;
//  15 - log(u32::MAX, 10) ≈ 5.36
const TOLERANCE: f64 = 1e-5;

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
        // x + y + sqrt(x**2 + y**2) = p
        let (x, p) = (small_leg as f64, perimeter as f64);
        let large_leg = Self::into_integer(p * (p - 2.0 * x) / (2.0 * (p - x)))?;
        let y = large_leg as f64;
        let hypothenuse = Self::into_integer((x.powi(2) + y.powi(2)).sqrt())?;
        if small_leg + large_leg + hypothenuse == perimeter {
            return Some(Self {
                small_leg,
                large_leg,
                hypothenuse,
            });
        }
        None
    }

    fn into_integer(value: f64) -> Option<u32> {
        if (value.round() - value).abs() < TOLERANCE && value <= u32::MAX as f64 && value > 0.0 {
            return Some(value as u32);
        }
        None
    }

    pub fn to_slice(&self) -> [u32; 3] {
        [self.small_leg, self.large_leg, self.hypothenuse]
    }
}
