#[derive(Debug)]
pub struct ChessPosition {
    file: i8,
    ranK: i8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
            return None;
        }
        Some(Self {
            file: file as i8,
            ranK: rank as i8,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (
            self.position.file - other.position.file,
            self.position.ranK - other.position.ranK,
        ) {
            (0, _) => true,
            (_, 0) => true,
            (a, b) if a.abs() == b.abs() => true,
            _ => false,
        }
    }
}
