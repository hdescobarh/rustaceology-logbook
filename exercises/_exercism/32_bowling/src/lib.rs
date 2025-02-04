use std::cmp::Ordering;

pub enum Status {
    Unbegun,
    Unfinished,
    Open,
    Spare,
    Strike,
}

struct Frame {
    status: Status,
    throws: Vec<u16>,
    fillballs: Option<Vec<u16>>,
}

impl Frame {
    pub fn new(is_last: bool) -> Self {
        Self {
            status: Status::Unbegun,
            throws: Vec::new(),
            fillballs: if is_last { Some(Vec::new()) } else { None },
        }
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.status {
            Status::Unbegun => {
                self.status = if pins < 10 {
                    Status::Unfinished
                } else {
                    Status::Strike
                };
                self.throws.push(pins)
            }
            Status::Unfinished => {
                match (self.throws[0] + pins).cmp(&10) {
                    Ordering::Less => self.status = Status::Open,
                    Ordering::Equal => self.status = Status::Spare,
                    Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                }
                self.throws.push(pins)
            }
            Status::Spare if self.fillballs.as_ref().is_some_and(|v| v.is_empty()) => {
                self.fillballs.as_mut().unwrap().push(pins);
            }
            Status::Strike if self.fillballs.as_ref().is_some_and(|v| v.is_empty()) => {
                self.fillballs.as_mut().unwrap().push(pins);
            }
            Status::Strike
                if self
                    .fillballs
                    .as_ref()
                    .is_some_and(|v| v.len() == 1 && v[0] < 10) =>
            {
                match (self.fillballs.as_ref().unwrap()[0] + pins).cmp(&10) {
                    Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                    _ => self.fillballs.as_mut().unwrap().push(pins),
                };
            }
            Status::Strike if self.fillballs.as_ref().is_some_and(|v| v.len() == 1) => {
                self.fillballs.as_mut().unwrap().push(pins)
            }
            _ => return Err(Error::NotEnoughPinsLeft),
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current: usize,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            current: 0,
            frames: (0..10).map(|i| Frame::new(i > 8)).collect(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        todo!("Record that {pins} pins have been scored");
    }

    pub fn score(&self) -> Option<u16> {
        todo!("Return the score if the game is complete, or None if not.");
    }
}
