use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
};

pub fn winning_hands<'a>(input: &[&'a str]) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut hands = Vec::with_capacity(input.len());
    for single_hand in input {
        hands.push(Hand::try_new(single_hand).unwrap());
    }
    let max_hand = hands.iter().max().unwrap();
    for hand in hands.iter() {
        if hand == max_hand {
            result.push(hand.reference);
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug)]
struct Hand<'a> {
    category: HandCategory,
    hand_ranking: Vec<u8>,
    reference: &'a str,
}

impl<'a> Hand<'a> {
    fn try_new(input: &'a str) -> Option<Self> {
        let mut rank_count: BTreeMap<u8, usize> = BTreeMap::new();
        let mut suits: HashSet<char> = HashSet::new();
        let mut iter = input.split_ascii_whitespace().enumerate().peekable();
        while let Some((index, card_str)) = iter.next() {
            if iter.peek().is_none() && index != 4 {
                return None;
            }
            let (rank, suit) = Self::validate_card(card_str)?;
            *rank_count.entry(rank).or_insert(0) += 1;
            suits.insert(suit);
        }
        let (category, hand_ranking) = Self::categorize(rank_count, suits);
        Some(Self {
            category,
            hand_ranking,
            reference: input,
        })
    }

    fn validate_card(card: &str) -> Option<(u8, char)> {
        let mut iter = card.bytes().peekable();
        let rank = match iter.next() {
            Some(v) if (b'2'..=b'9').contains(&v) => v - b'0',
            Some(b'1') if iter.peek().is_some_and(|v| *v == b'0') => {
                iter.next();
                10
            }
            Some(b'J') => 11,
            Some(b'Q') => 12,
            Some(b'K') => 13,
            Some(b'A') => 14,
            _ => return None,
        };
        let suit = match iter.next() {
            Some(v) if [b'C', b'D', b'H', b'S'].contains(&v) => v as char,
            _ => return None,
        };
        Some((rank, suit))
    }

    fn categorize(
        rank_count: BTreeMap<u8, usize>,
        suits: HashSet<char>,
    ) -> (HandCategory, Vec<u8>) {
        if rank_count.len() == 5 {
            let (hand_ranking, is_sequential) = Self::sort_sequential(&rank_count);
            let category = match (suits.len(), is_sequential) {
                (1, true) => HandCategory::StraightFlush,
                (1, false) => HandCategory::Flush,
                (_, true) => HandCategory::Straight,
                _ => HandCategory::HighCard,
            };
            return (category, hand_ranking);
        }

        let (mut hand_ranking, max_repeats) = Self::sort_by_repeats(&rank_count);
        let category = match (rank_count.len(), max_repeats) {
            (2, 4) => HandCategory::FourOfAKind,
            (2, 3) => HandCategory::FullHouse,
            (3, 3) => HandCategory::ThreeOfAKind,
            (3, 2) => HandCategory::TwoPair,
            (4, 2) => HandCategory::OnePair,
            _ => {
                hand_ranking = rank_count.keys().rev().copied().collect();
                HandCategory::HighCard
            }
        };
        (category, hand_ranking)
    }

    /// Returns the hand sorted and true if it in sequential order
    fn sort_sequential(rank_count: &BTreeMap<u8, usize>) -> (Vec<u8>, bool) {
        let ranks: Vec<u8> = rank_count.keys().rev().copied().collect();
        if ranks == [14, 13, 12, 11, 10] {
            return (vec![14, 13, 12, 11, 10], true);
        } else if ranks == [14, 5, 4, 3, 2] {
            return (vec![5, 4, 3, 2, 1], true);
        }
        let mut current_highest = ranks[0];
        for rank in ranks[1..].iter() {
            if rank + 1 != current_highest {
                return (ranks, false);
            }
            current_highest = *rank;
        }
        (ranks, true)
    }

    /// Returns the hand sorted by deceasing, first rank, then number of repeats,
    /// and the max number of repeats
    fn sort_by_repeats(rank_count: &BTreeMap<u8, usize>) -> (Vec<u8>, usize) {
        let mut hand_ranking = Vec::with_capacity(rank_count.len());
        for rank in rank_count.keys() {
            hand_ranking.push(*rank);
        }
        hand_ranking.sort_by_key(|rank| std::cmp::Reverse(rank_count[rank]));
        let max_repeats = rank_count[&hand_ranking[0]];
        (hand_ranking, max_repeats)
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.hand_ranking == other.hand_ranking
    }
}

impl Eq for Hand<'_> {}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Equal => self.hand_ranking.cmp(&other.hand_ranking),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
