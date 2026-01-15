use super::types::{Color, Position};

const BOARD_SIZE: usize = 19;

pub struct Board {
    grid: [[Option<Color>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn get(&self, pos: Position) -> Option<Color> {
        if pos.is_valid() {
            self.grid[pos.y][pos.x]
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: Position, color: Option<Color>) {
        if pos.is_valid() {
            self.grid[pos.y][pos.x] = color;
        }
    }

    pub fn is_empty(&self, pos: Position) -> bool {
        self.get(pos).is_none()
    }

    /// Count liberties for a stone at the given position
    pub fn count_liberties(&self, pos: Position) -> usize {
        // TODO: Implement liberty counting for groups
        // - Find all stones in the group (connected same-color stones)
        // - Count empty adjacent intersections
        0
    }
}
