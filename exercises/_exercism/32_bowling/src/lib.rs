pub enum FrameStatus {
    Unbegun,
    Unfinished(u16),
    Open(u16),
    Spare(u16),
    Strike,
}

struct Frame {
    status: FrameStatus,
    fill_balls: Option<Vec<u16>>,
}

impl Frame {
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.status {
            FrameStatus::Unbegun => self.status = FrameStatus::Unfinished(pins),
            FrameStatus::Unfinished(knocked) => {
                self.status = match knocked.checked_add(pins) {
                    Some(new_knocked) if new_knocked < 10 => FrameStatus::Open(new_knocked),
                    Some(10) => FrameStatus::Spare(pins),
                    _ => return Err(Error::NotEnoughPinsLeft),
                }
            }
            FrameStatus::Spare(_) if self.fill_balls.as_ref().is_some_and(|v| v.is_empty()) => {
                if let Some(v) = self.fill_balls.as_mut() {
                    v.push(pins);
                };
            }
            FrameStatus::Strike if self.fill_balls.as_ref().is_some_and(|v| v.len() < 2) => {
                if let Some(v) = self.fill_balls.as_mut() {
                    v.push(pins);
                }
            }
            _ => return Err(Error::NotEnoughPinsLeft),
        };
        Ok(())
    }
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
