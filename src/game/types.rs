use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_valid(&self) -> bool {
        self.x < 19 && self.y < 19
    }

    /// Get adjacent positions (up, down, left, right)
    pub fn adjacent(&self) -> Vec<Position> {
        let mut positions = Vec::new();

        if self.x > 0 {
            positions.push(Position::new(self.x - 1, self.y));
        }
        if self.x < 18 {
            positions.push(Position::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            positions.push(Position::new(self.x, self.y - 1));
        }
        if self.y < 18 {
            positions.push(Position::new(self.x, self.y + 1));
        }

        positions
    }
}
