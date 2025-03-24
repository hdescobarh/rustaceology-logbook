use std::{fmt::Display, ops::Rem};

pub struct Matcher<T: Display + Copy> {
    op: Box<dyn Fn(T) -> Option<String>>,
}

impl<T: Display + Copy> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Display + 'static,
    {
        let op = move |value: T| {
            if matcher(value) {
                return Some(subs.to_string());
            }
            None
        };

        Self { op: Box::new(op) }
    }

    pub fn check(&self, element: T) -> Option<String> {
        (self.op)(element)
    }
}

pub struct Fizzy<T: Display + Copy> {
    rules: Vec<Matcher<T>>,
}

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    #[must_use]
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        let mut rules = self.rules;
        rules.push(matcher);
        Self { rules }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |item| {
            let out = self.rules.iter().fold(String::new(), |mut acc, matcher| {
                if let Some(v) = matcher.check(item) {
                    acc.push_str(&v);
                };
                acc
            });
            if out.is_empty() {
                return item.to_string();
            }
            out
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + Copy + Rem<Output = T> + From<u8> + PartialEq,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|item: T| item % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|item: T| item % 5.into() == 0.into(), "buzz"))
}
