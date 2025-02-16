use std::{collections::HashMap, fmt::Display};
const COLUMN_DELIMITER: char = ';';
const WIN_SCORE: u16 = 3;
const DRAW_SCORE: u16 = 1;
const LOSS_SCORE: u16 = 0;

pub fn tally(match_results: &str) -> String {
    todo!(
        "Given the result of the played matches '{match_results}' return a properly formatted tally table string."
    );
}

enum MatchResult {
    Loss,
    Draw,
    Win,
}

impl TryFrom<&str> for MatchResult {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let out = match value {
            "loss" => Self::Loss,
            "draw" => Self::Draw,
            "win" => Self::Win,
            _ => return Err(()),
        };
        Ok(out)
    }
}

impl MatchResult {
    fn as_rival_result(&self) -> Self {
        match &self {
            MatchResult::Loss => Self::Win,
            MatchResult::Draw => Self::Draw,
            MatchResult::Win => Self::Loss,
        }
    }
}

struct TeamStats {
    team: String,
    games: u16,
    won: u16,
    tied: u16,
    lost: u16,
    points: u16,
}

impl TeamStats {
    fn new(team: &str) -> Self {
        Self {
            team: team.to_string(),
            games: 0,
            won: 0,
            tied: 0,
            lost: 0,
            points: 0,
        }
    }

    fn add_match_result(&mut self, result: &MatchResult) {
        self.games += 1;
        match result {
            MatchResult::Loss => {
                self.lost += 1;
                self.points += LOSS_SCORE
            }
            MatchResult::Draw => {
                self.tied += 1;
                self.points += DRAW_SCORE
            }
            MatchResult::Win => {
                self.won += 1;
                self.points += WIN_SCORE
            }
        }
    }
}

impl Display for TeamStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:31}| {:2} | {:2} | {:2} | {:2} | {:2}",
            self.team, self.games, self.won, self.tied, self.lost, self.points
        )
    }
}
