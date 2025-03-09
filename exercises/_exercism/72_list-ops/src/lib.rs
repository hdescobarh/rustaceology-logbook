// Solution avoiding reallocations
struct Append<I: Iterator, J: Iterator<Item = I::Item>>(I, J);

impl<I: Iterator, J: Iterator<Item = I::Item>> Iterator for Append<I, J> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().or_else(|| self.1.next())
    }
}
/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    Append(a, b)
}

struct Concat<I: Iterator<Item: Iterator>> {
    pending: I,
    current: Option<I::Item>,
}

impl<I: Iterator<Item: Iterator>> Concat<I> {
    fn new(mut iter: I) -> Self {
        let current = iter.next();
        Self {
            pending: iter,
            current,
        }
    }
}

impl<I: Iterator<Item: Iterator>> Iterator for Concat<I> {
    type Item = <I::Item as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.current.as_mut().and_then(|iter| iter.next());
        if output.is_some() {
            return output;
        }
        self.pending.next().and_then(|iter| {
            self.current = Some(iter);
            self.next()
        })
    }
}
/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    Concat::new(nested_iter)
}

struct Filter<I: Iterator, F: Fn(&I::Item) -> bool> {
    iter: I,
    predicate: F,
}

impl<I: Iterator, F: Fn(&I::Item) -> bool> Iterator for Filter<I, F> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) if (self.predicate)(&item) => Some(item),
            Some(_) => self.next(),
            None => None,
        }
    }
}
/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filter { iter, predicate }
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;
    for _ in iter {
        count += 1;
    }
    count
}

struct Map<I: Iterator, F: Fn(I::Item) -> U, U> {
    iter: I,
    function: F,
}

impl<I: Iterator, F: Fn(I::Item) -> U, U> Iterator for Map<I, F, U> {
    type Item = U;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => Some((self.function)(item)),
            None => None,
        }
    }
}
/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map { iter, function }
}

pub fn foldl<I: Iterator, F: Fn(U, I::Item) -> U, U>(mut iter: I, initial: U, function: F) -> U {
    let mut acc = match iter.next() {
        Some(item) => (function)(initial, item),
        None => return initial,
    };
    for item in iter {
        acc = (function)(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = match iter.next_back() {
        Some(item) => (function)(initial, item),
        None => return initial,
    };
    for item in iter {
        acc = (function)(acc, item);
    }
    acc
}

struct Reverse<I: DoubleEndedIterator>(I);

impl<I: DoubleEndedIterator> Iterator for Reverse<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}
/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    Reverse(iter)
}
