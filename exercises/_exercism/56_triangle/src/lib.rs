use std::ops::Add;

pub struct Triangle<T>
where
    T: Copy + Add<Output = T> + PartialOrd + PartialEq,
{
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Copy + Add<Output = T> + PartialOrd + PartialEq,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        if a + b < c || b + c < a || c + a < b || Self::any_zero(a, b, c) {
            return None;
        }
        Some(Self { a, b, c })
    }

    fn any_zero(a: T, b: T, c: T) -> bool {
        let sum = a + b + c;
        sum + a == sum || sum + b == sum || sum + c == sum
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c && self.c == self.b
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.c != self.b
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.c == self.b
    }
}
