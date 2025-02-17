use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt::Display;
const COLUMN_DELIMITER: char = ';';
const WIN_SCORE: u16 = 3;
const DRAW_SCORE: u16 = 1;
const LOSS_SCORE: u16 = 0;

pub fn tally(match_results: &str) -> String {
    let mut parsed_data: BTreeMap<&str, TeamStats> = BTreeMap::new();
    for line in match_results.lines() {
        let mut match_result = None::<MatchResult>;
        for (col, value) in line.split(COLUMN_DELIMITER).rev().enumerate() {
            let team_result = match col {
                0 => {
                    match_result = MatchResult::try_from(value).ok();
                    continue;
                }
                1 => &match_result.as_ref().unwrap().as_rival_result(),
                2 => match_result.as_ref().unwrap(),
                _ => panic!("Wrong format. Expects three columns"),
            };
            parsed_data
                .entry(value)
                .and_modify(|team| team.add_match_result(team_result))
                .or_insert(TeamStats::new(value).with_match(team_result));
        }
    }
    Summary::new(parsed_data).to_string()
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
    fn as_rival_result(&self) -> MatchResult {
        match &self {
            MatchResult::Loss => Self::Win,
            MatchResult::Draw => Self::Draw,
            MatchResult::Win => Self::Loss,
        }
    }
}

#[derive(Clone)]
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

    fn with_match(mut self, result: &MatchResult) -> Self {
        self.add_match_result(result);
        self
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

struct Summary(Vec<TeamStats>);

impl Summary {
    fn new(data: BTreeMap<&str, TeamStats>) -> Self {
        let mut sorted_data: Vec<TeamStats> = data.into_values().collect();
        sorted_data.sort_by_key(|team| Reverse(team.points));
        Self(sorted_data)
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w1 = 31;
        let w2 = 2;
        writeln!(
            f,
            "{:<w1$}| {:>w2$} | {:>w2$} | {:>w2$} | {:>w2$} | {:>w2$}",
            "Team", "MP", "W", "D", "L", "P"
        )?;

        for stats in &self.0[..self.0.len() - 1] {
            writeln!(
                f,
                "{:<w1$}| {:>w2$} | {:>w2$} | {:>w2$} | {:>w2$} | {:>w2$}",
                stats.team, stats.games, stats.won, stats.tied, stats.lost, stats.points
            )?;
        }

        if let Some(stats) = self.0.last() {
            write!(
                f,
                "{:<w1$}| {:>w2$} | {:>w2$} | {:>w2$} | {:>w2$} | {:>w2$}",
                stats.team, stats.games, stats.won, stats.tied, stats.lost, stats.points
            )?;
        }

        Ok(())
    }
}
