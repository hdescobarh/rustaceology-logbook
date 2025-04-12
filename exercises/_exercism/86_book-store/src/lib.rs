use std::collections::{HashMap, HashSet};

const UNITARY_PRICE: f64 = 800.0;

pub fn lowest_price(books: &[u32]) -> u32 {
    todo!("Find the lowest price of the bookbasket with books {books:?}")
}

fn count_by_id(books: &[u32]) -> Vec<usize> {
    let mut frequency_by_id: HashMap<&u32, usize> = HashMap::new();
    for id in books {
        *frequency_by_id.entry(id).or_default() += 1;
    }
    frequency_by_id.values().copied().collect()
}

// A sightly modified Heap's algorithm
fn unique_permutations(k: usize, elements: &mut [usize], output: &mut HashSet<Vec<usize>>) {
    if k <= 1 {
        output.insert(elements.to_vec());
        return;
    }
    unique_permutations(k - 1, elements, output);
    for index in 0..k - 1 {
        if k % 2 == 0 {
            elements.swap(index, k - 1);
        } else {
            elements.swap(0, k - 1);
        }
        unique_permutations(k - 1, elements, output);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn unique_permutations_empty() {
        let mut input = vec![];
        let expected: HashSet<Vec<usize>> = [vec![]].into_iter().collect();
        let mut actual = HashSet::new();
        unique_permutations(input.len(), &mut input, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_permutations_single_element() {
        let mut input = vec![1];
        let expected: HashSet<Vec<usize>> = [vec![1]].into_iter().collect();
        let mut actual = HashSet::new();
        unique_permutations(input.len(), &mut input, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_permutations_two_elements() {
        let mut input = vec![1, 2];
        let mut expected: HashSet<Vec<usize>> = [vec![1, 2], vec![2, 1]].into_iter().collect();
        let mut actual = HashSet::new();
        unique_permutations(input.len(), &mut input, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_permutations_three_elements() {
        let mut input = vec![1, 2, 3];
        let expected: HashSet<Vec<usize>> = [
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]
        .into_iter()
        .collect();
        let mut actual = HashSet::new();
        unique_permutations(input.len(), &mut input, &mut actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_permutations_duplicates() {
        let mut input = vec![1, 1];
        let expected: HashSet<Vec<usize>> = [vec![1, 1], vec![1, 1]].into_iter().collect();
        let mut actual = HashSet::new();
        unique_permutations(input.len(), &mut input, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_permutations_duplicates_with_uniques() {
        let mut input = vec![1, 1, 0];
        let mut expected: HashSet<Vec<usize>> = [vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]
            .into_iter()
            .collect();
        let mut actual = HashSet::new();
        unique_permutations(input.len(), &mut input, &mut actual);
        assert_eq!(actual, expected);
    }
}
