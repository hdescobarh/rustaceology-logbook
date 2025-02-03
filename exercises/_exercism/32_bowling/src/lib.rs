pub enum FrameStatus {
    Unbegun,
    Unfinished(u16),
    Open(u16),
    Spare(u16),
    Strike,
}

struct Frame {
    status: FrameStatus,
    fill_balls: Option<Vec<FrameStatus>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
pub struct BowlingGame {
    current: u8,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            current: 0,
            frames: (0..10)
                .map(|i| {
                    let fill_balls = if i < 8 { Some(Vec::new()) } else { None };
                    Frame {
                        status: FrameStatus::Unbegun,
                        fill_balls,
                    }
                })
                .collect(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        todo!("Record that {pins} pins have been scored");
    }

    pub fn score(&self) -> Option<u16> {
        todo!("Return the score if the game is complete, or None if not.");
    }
}
