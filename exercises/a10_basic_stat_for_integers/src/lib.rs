use std::collections::HashMap;
use std::ops::Div;

pub struct DataList {
    data: Vec<isize>,
}

impl DataList {
    pub fn new(mut data_list: Vec<isize>) -> Self {
        assert!(data_list.len() > 0);
        data_list.sort();
        Self { data: data_list }
    }

    pub fn median(&self) -> f64 {
        let index = self.data.len().checked_div_euclid(2usize).unwrap();
        if self.data.len().checked_rem_euclid(2usize).unwrap() == 0 {
            let mut total: f64 = 0.0;
            total += *self.data.get(index).unwrap() as f64;
            total += *self
                .data
                .get(index.checked_add_signed(-1).unwrap())
                .unwrap() as f64;
            return total.div(2.0);
        } else {
            return *self.data.get(index).unwrap() as f64;
        }
    }

    pub fn frequency_table(&self) -> HashMap<isize, usize> {
        let mut absolute_frequency: HashMap<isize, usize> = HashMap::new();
        for value in self.data.iter() {
            let count = absolute_frequency.entry(*value).or_default();
            *count += 1;
        }
        return absolute_frequency;
    }

    pub fn mode(&self) -> Vec<isize> {
        let frequencies = self.frequency_table();
        let max_value = frequencies.values().max().unwrap();
        let mut mode: Vec<isize> = Vec::new();

        for pair in frequencies.iter() {
            if *pair.1 == *max_value {
                mode.push(*pair.0);
            }
        }

        mode.sort();
        return mode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_median_correctly() {
        let nonnegative_odd = DataList::new(vec![0, 4, 2, 3, 1]);
        assert_eq!(2.0, nonnegative_odd.median());

        let nonnegative_even = DataList::new(vec![10, 8, 7, 5, 6, 9]);
        assert_eq!(7.5, nonnegative_even.median());

        let withnegative_odd = DataList::new(vec![3, -1, -5, -10, -3]);
        assert_eq!(-3.0, withnegative_odd.median());

        let with_negative_even = DataList::new(vec![10, -60, 3, -50, -5, 8]);
        assert_eq!(-1.0, with_negative_even.median());

        let nonnegtive_with_repetition = DataList::new(vec![1, 5, 9, 9, 8, 1, 9, 1, 5, 10]);

        assert_eq!(6.5, nonnegtive_with_repetition.median());

        let with_negative_with_repetition = DataList::new(vec![100, -100, -20, -50, 80, -10, -30]);
        assert_eq!(-20.0, with_negative_with_repetition.median());
    }

    #[test]
    fn get_absolute_frequency_table_correctly() {
        let only_one_repeated = DataList::new(vec![42, 42, 42, 42, 42, 42, 42]);
        assert_eq!(
            7usize,
            *only_one_repeated.frequency_table().get(&42).unwrap()
        );

        let all_absolute_frequency_equalls_to_one = DataList::new(vec![0, 1, 2, 3, 5, 8, 13, 21]);
        let all_one_frequency_table = all_absolute_frequency_equalls_to_one.frequency_table();

        for value in &all_absolute_frequency_equalls_to_one.data {
            assert_eq!(1usize, *all_one_frequency_table.get(value).unwrap());
        }
    }

    #[test]
    fn get_mode_correctly() {
        let single_mode: Vec<isize> = DataList::new(vec![0, 1, 1, 3, 100, 100, 100]).mode();
        assert_eq!(vec![100isize], single_mode);

        let two_modes: Vec<isize> = DataList::new(vec![100, 500, 500, 100, 3]).mode();
        assert_eq!(vec![100isize, 500isize], two_modes);

        let multiple_modes: Vec<isize> = DataList::new(vec![324, 42, 60, 999, 10000]).mode();
        assert_eq!(vec![42, 60, 324, 999, 10000], multiple_modes);
    }
}
