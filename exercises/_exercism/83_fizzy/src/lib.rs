// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::fmt::Display;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T: Display + Copy> {
    op: Box<dyn Fn(T) -> String>,
}

impl<T: Display + Copy> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Display + 'static,
    {
        let op = move |value: T| {
            if matcher(value) {
                return subs.to_string();
            }
            value.to_string()
        };

        Self { op: Box::new(op) }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T: Display + Copy> {
    rules: Vec<Matcher<T>>,
}

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        todo!()
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, _matcher: Matcher<T>) -> Self {
        todo!()
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String> {
        // todo!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        Vec::new().into_iter()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Display + Copy>() -> Fizzy<T> {
    todo!()
}
