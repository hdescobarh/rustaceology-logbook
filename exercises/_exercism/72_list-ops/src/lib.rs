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

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(_iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}

pub fn length<I: Iterator>(_iter: I) -> usize {
    todo!("return the total number of items within iter")
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}

pub fn foldl<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    todo!("starting with initial, fold (reduce) each iter item into the accumulator from the left")
}

pub fn foldr<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    todo!("starting with initial, fold (reduce) each iter item into the accumulator from the right")
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(_iter: I) -> impl Iterator<Item = I::Item> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}
