pub fn map<T, S>(input: Vec<T>, mut closure: impl FnMut(T) -> S) -> Vec<S> {
    let mut output: Vec<S> = Vec::with_capacity(input.len());
    for element in input {
        output.push(closure(element))
    }
    output
}
