mod board;
mod rules;
mod types;

pub use types::{Color, Position};

/// Main game state
pub struct Game {
    board: board::Board,
    turn: Color,
    prisoners: (u32, u32), // (black_captured, white_captured)
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: board::Board::new(),
            turn: Color::Black,
            prisoners: (0, 0),
        }
    }

    /// Attempt to place a stone at the given position
    pub fn place_stone(&mut self, pos: Position, color: Color) -> Result<(), String> {
        // TODO: Implement move validation and application
        // - Check if intersection is empty
        // - Check if it's the right player's turn
        // - Apply move
        // - Detect and remove captured stones
        // - Check for ko violation
        // - Update turn
        Ok(())
    }

    /// Pass turn
    pub fn pass(&mut self) {
        self.turn = self.turn.opposite();
    }

    /// Reset game to initial state
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
