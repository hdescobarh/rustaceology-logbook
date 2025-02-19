#[derive(Debug)]
pub enum Error {
    InvalidAction,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            x: self.x,
            y: self.y,
            direction,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            x: self.x,
            y: self.y,
            direction,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };
        Self {
            x,
            y,
            direction: self.direction,
        }
    }

    fn parse_instruction(self, instruction: char) -> Result<Self, Error> {
        let output = match instruction {
            'R' => self.turn_right(),
            'L' => self.turn_left(),
            'A' => self.advance(),
            _ => return Err(Error::InvalidAction),
        };
        Ok(output)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .try_fold(self, |robot, instruction| {
                robot.parse_instruction(instruction)
            })
            .unwrap()
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
