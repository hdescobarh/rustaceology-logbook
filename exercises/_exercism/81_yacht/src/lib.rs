use Category::*;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let times = dice.iter().fold(vec![0; 6], |mut acc, &number| {
        acc[number as usize - 1] += 1;
        acc
    });
    let mut score = 0;
    match category {
        Ones => score = times[0],
        Twos => score = 2 * times[1],
        Threes => score = 3 * times[2],
        Fours => score = 4 * times[3],
        Fives => score = 5 * times[4],
        Sixes => score = 6 * times[5],
        FullHouse if times.contains(&3) && times.contains(&2) => score = dice.iter().sum(),
        FourOfAKind => {
            score = times
                .iter()
                .position(|&t| t >= 4)
                .map(|num| (num + 1) as u8 * 4)
                .unwrap_or_default()
        }
        LittleStraight if times[..5].iter().all(|&t| t == 1) => score = 30,
        BigStraight if times[1..].iter().all(|&t| t == 1) => score = 30,
        Choice => score = dice.iter().sum(),
        Yacht if times.contains(&5) => score = 50,
        _ => (),
    };
    score
}
