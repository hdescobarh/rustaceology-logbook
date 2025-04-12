use std::collections::{HashMap, HashSet};

// Given an array [a_1, ..., a_5] representing the count of each title in the series,
// partition them into max(a_i) bins b_j, each with a capacity of 5.
// Each bin can contain at most one copy of each title.
// The value of a bin is a function of its size:
//   f(1) = 0
//   f(2) = 2 * 0.05
//   f(3) = 3 * 0.10
//   f(4) = 4 * 0.20
//   f(5) = 5 * 0.25
// The goal is to maximize the total value S = sum of f(size_j) over all bins.
// The total cost is: unitary_cost * (total_books - S)

const UNITARY_PRICE: f64 = 800.0;

pub fn lowest_price(books: &[u32]) -> u32 {
    let frequency_by_title = id_frequencies_decreasing(books);
    let bins = match frequency_by_title.first() {
        Some(size) => *size,
        None => return 0,
    };
    let mut best_partition = [1_usize].repeat(bins);
    let mut best_discount = 0.0;
    for title_count in frequency_by_title.iter().skip(1) {
        (best_partition, best_discount) = add_title(*title_count, &best_partition);
    }
    (UNITARY_PRICE * (best_partition.into_iter().sum::<usize>() as f64 - best_discount)).round()
        as u32
}

/// add the title choosing the optime arrangement for it
fn add_title(title_count: usize, current_partition: &[usize]) -> (Vec<usize>, f64) {
    let mut title_arrangements = HashSet::new();
    let mut base_case = [
        [1].repeat(title_count),
        [0].repeat(current_partition.len() - title_count),
    ]
    .concat();
    unique_permutations(base_case.len(), &mut base_case, &mut title_arrangements);

    let mut best_partition = vec![];
    let mut best_discount = 0.0;
    for p in title_arrangements {
        let candidate: Vec<usize> = current_partition
            .iter()
            .zip(p)
            .map(|(a, b)| a + b)
            .collect();
        let candidate_discount = candidate
            .iter()
            .try_fold(0.0, |acc, bin_size| {
                bin_discount(*bin_size).map(|value| value + acc)
            })
            .unwrap();
        if candidate_discount > best_discount {
            best_partition = candidate;
            best_discount = candidate_discount;
        }
    }
    (best_partition, best_discount)
}

fn bin_discount(unique_books: usize) -> Option<f64> {
    let result = match unique_books {
        1 => 0.0,
        2 => 0.10,
        3 => 0.30,
        4 => 0.80,
        5 => 1.25,
        _ => return None,
    };
    Some(result)
}

fn id_frequencies_decreasing(books: &[u32]) -> Vec<usize> {
    let mut frequency_by_id: HashMap<&u32, usize> = HashMap::new();
    for id in books {
        *frequency_by_id.entry(id).or_default() += 1;
    }
    let mut frequencies: Vec<usize> = frequency_by_id.values().copied().collect();
    frequencies.sort_by(|a, b| b.cmp(a));
    frequencies
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
