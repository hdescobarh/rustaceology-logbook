use std::cmp::Ordering;

#[cfg_attr(test, derive(Debug, PartialEq))]
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

#[cfg(test)]
mod test {
    use crate::Error::*;
    use crate::{Frame, Status};

    #[test]
    fn frame_unbegun_to_unfinished() {
        let mut frame = Frame::new(false);
        assert_eq!(frame.throws.first(), None);
        assert_eq!(frame.roll(5), Ok(()));
        assert_eq!(frame.status, Status::Unfinished);
        assert_eq!(frame.throws[0], 5);
        assert_eq!(frame.throws.get(1), None);
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
        assert_eq!(frame.roll(5), Ok(()));
    }

    #[test]
    fn frame_unbegun_to_strike() {
        let mut frame = Frame::new(false);
        assert_eq!(frame.throws.first(), None);
        assert_eq!(frame.roll(10), Ok(()));
        assert_eq!(frame.status, Status::Strike);
        assert_eq!(frame.throws[0], 10);
        assert_eq!(frame.throws.get(1), None);
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
    }

    #[test]
    fn frame_unfinished_to_spare() {
        let mut frame = Frame::new(false);
        let _ = frame.roll(4);
        let _ = frame.roll(6);
        assert_eq!(frame.status, Status::Spare);
        assert_eq!(frame.throws[0], 4);
        assert_eq!(frame.throws[1], 6);
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
    }

    #[test]
    fn frame_unfinished_to_open() {
        let mut frame = Frame::new(false);
        let _ = frame.roll(7);
        let _ = frame.roll(1);
        assert_eq!(frame.status, Status::Open);
        assert_eq!(frame.throws[0], 7);
        assert_eq!(frame.throws[1], 1);
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
    }

    #[test]
    fn fillballs_spare_open() {
        let mut frame = Frame::new(true);
        let _ = frame.roll(3);
        let _ = frame.roll(7);
        assert_eq!(frame.status, Status::Spare);
        assert_eq!(frame.throws[0], 3);
        assert_eq!(frame.throws[1], 7);
        let _ = frame.roll(2);
        assert_eq!(frame.fillballs.as_ref().unwrap()[0], 2);
        assert_eq!(frame.fillballs.as_ref().unwrap().get(1), None);
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
    }
    #[test]
    fn fillballs_spare_strike() {
        let mut frame = Frame::new(true);
        let _ = frame.roll(1);
        let _ = frame.roll(9);
        assert_eq!(frame.status, Status::Spare);
        assert_eq!(frame.throws[0], 1);
        assert_eq!(frame.throws[1], 9);
        let _ = frame.roll(10);
        assert_eq!(frame.fillballs.as_ref().unwrap()[0], 10);
        assert_eq!(frame.fillballs.as_ref().unwrap().get(1), None);
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
    }
    #[test]
    fn fillballs_strike_spare() {
        let mut frame = Frame::new(true);
        let _ = frame.roll(10);
    }
    #[test]
    fn fillballs_strike_open() {
        let mut frame = Frame::new(true);
        let _ = frame.roll(10);
    }
    #[test]
    fn fillballs_triple_strike() {
        let mut frame = Frame::new(true);
        assert_eq!(frame.roll(10), Ok(()));
        assert_eq!(frame.status, Status::Strike);
        assert_eq!(frame.roll(10), Ok(()));
        assert_eq!(frame.roll(10), Ok(()));
        assert_eq!(frame.roll(10), Err(NotEnoughPinsLeft));
        assert_eq!(*frame.fillballs.as_ref().unwrap(), vec![10, 10]);
        assert_eq!(frame.throws, vec![10])
    }
    #[test]
    fn fillballs_strike_strike_open() {
        let mut frame = Frame::new(true);
        assert_eq!(frame.roll(10), Ok(()));
        assert_eq!(frame.status, Status::Strike);
        assert_eq!(frame.roll(10), Ok(()));
        assert_eq!(frame.roll(8), Ok(()));
        assert_eq!(frame.roll(2), Err(NotEnoughPinsLeft));
        assert_eq!(*frame.fillballs.as_ref().unwrap(), vec![10, 8]);
        assert_eq!(frame.throws, vec![10])
    }
}
