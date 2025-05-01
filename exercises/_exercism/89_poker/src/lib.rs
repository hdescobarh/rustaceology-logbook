use std::collections::{BTreeMap, HashMap, HashSet};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    todo!("Out of {hands:?}, which hand wins?")
}

#[derive(Debug, PartialEq, Eq)]
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

struct Hand<'a> {
    category: HandCategory,
    reference: &'a str,
}

impl<'a> Hand<'a> {
    fn try_new(input: &'a str) -> Option<Self> {
        let mut rank_count: BTreeMap<u8, u8> = BTreeMap::new();
        let mut suit_groups: HashMap<char, Vec<u8>> = HashMap::new();
        for (index, card_str) in input.split_ascii_whitespace().enumerate() {
            if index > 4 {
                return None;
            };
            let (rank, suit) = Self::validate_card(card_str)?;
            *rank_count.entry(rank).or_insert(0) += 1;
            suit_groups.entry(suit).or_default().push(rank);
        }

        let category = Self::categorize(&rank_count, &suit_groups);
        Some(Self {
            category,
            reference: input,
        })
    }

    fn validate_card(card: &str) -> Option<(u8, char)> {
        let mut iter = card.bytes().peekable();
        let rank = match iter.next() {
            Some(v) if (b'2'..=b'9').contains(&v) => v - b'0',
            Some(b'1') if iter.peek().is_some_and(|v| *v == 0) => {
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
        rank_count: &BTreeMap<u8, u8>,
        suits_observed: &HashMap<char, Vec<u8>>,
    ) -> HandCategory {
        match (rank_count.len(), suits_observed.len()) {
            (5, 1) => todo!(),          // 1 Straight flush or 4 flush
            (2, s) if s > 2 => todo!(), // 2 Four of a kind or 3 Full house
            (5, 4) => todo!(),          // 5 Straight
            (3, s) if s > 1 => todo!(), // 6 Three of a kind or 7 Two pair
            (4, s) if s > 1 => todo!(), // 8 one pair
            _ => HandCategory::HighCard,
        }
    }
}
