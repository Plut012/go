mod board;
mod rules;
mod types;

pub use types::{Color, Position};

/// Main game state
pub struct Game {
    board: board::Board,
    turn: Color,
    prisoners: (u32, u32), // (black_captured, white_captured)
    history: Vec<u64>,     // Board hashes for ko detection
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: board::Board::new(),
            turn: Color::Black,
            prisoners: (0, 0),
            history: Vec::new(),
        }
    }

    /// Attempt to place a stone at the given position
    pub fn place_stone(&mut self, pos: Position, color: Color) -> Result<(), String> {
        // 1. Check if it's the right player's turn
        if color != self.turn {
            return Err("Not your turn".to_string());
        }

        // 2. Check if position is valid
        if !pos.is_valid() {
            return Err("Invalid position".to_string());
        }

        // 3. Check if intersection is empty
        if !self.board.is_empty(pos) {
            return Err("Intersection occupied".to_string());
        }

        // 4. Check suicide rule
        if rules::is_suicide(&self.board, pos, color) {
            return Err("Suicide move not allowed".to_string());
        }

        // 5. Save board state for potential rollback
        let saved_board = self.board.grid.clone();

        // 6. Place the stone
        self.board.set(pos, Some(color));

        // 7. Remove captured opponent groups
        let opponent_color = color.opposite();
        let captures = rules::find_captures(&self.board, opponent_color);
        let num_captures = captures.len() as u32;

        for capture_pos in &captures {
            self.board.set(*capture_pos, None);
        }

        // 8. Check for ko violation
        let board_hash = rules::hash_board(&self.board);
        if rules::is_ko_violation(board_hash, &self.history) {
            // Rollback the board
            self.board.grid = saved_board;
            return Err("Ko rule violation".to_string());
        }

        // 9. Update prisoner count
        match color {
            Color::Black => self.prisoners.1 += num_captures, // Black captured white stones
            Color::White => self.prisoners.0 += num_captures, // White captured black stones
        }

        // 10. Update history
        self.history.push(board_hash);

        // 11. Switch turn
        self.turn = self.turn.opposite();

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

    /// Get the current board state as a 2D vector for serialization
    pub fn get_board(&self) -> Vec<Vec<Option<Color>>> {
        let mut result = Vec::with_capacity(19);
        for y in 0..19 {
            let mut row = Vec::with_capacity(19);
            for x in 0..19 {
                row.push(self.board.get(Position::new(x, y)));
            }
            result.push(row);
        }
        result
    }

    /// Get the current turn
    pub fn get_turn(&self) -> Color {
        self.turn
    }

    /// Get prisoner counts (black_captured, white_captured)
    pub fn get_prisoners(&self) -> (u32, u32) {
        self.prisoners
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_move_placement() {
        let mut game = Game::new();

        // Black plays first
        assert!(game.place_stone(Position::new(3, 3), Color::Black).is_ok());

        // White plays next
        assert!(game.place_stone(Position::new(3, 4), Color::White).is_ok());

        // Black plays again
        assert!(game.place_stone(Position::new(4, 3), Color::Black).is_ok());
    }

    #[test]
    fn test_turn_enforcement() {
        let mut game = Game::new();

        // Black plays first
        assert!(game.place_stone(Position::new(3, 3), Color::Black).is_ok());

        // Black tries to play again - should fail
        let result = game.place_stone(Position::new(4, 4), Color::Black);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not your turn");

        // White plays - should succeed
        assert!(game.place_stone(Position::new(4, 4), Color::White).is_ok());
    }

    #[test]
    fn test_occupied_intersection() {
        let mut game = Game::new();

        game.place_stone(Position::new(5, 5), Color::Black).unwrap();

        // Try to place white stone on same spot
        let result = game.place_stone(Position::new(5, 5), Color::White);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Intersection occupied");
    }

    #[test]
    fn test_capture_updates_prisoners() {
        let mut game = Game::new();

        // Set up a capture situation - surround a single white stone
        // White stone at (5,5), surrounded by black
        game.place_stone(Position::new(5, 4), Color::Black).unwrap();
        game.place_stone(Position::new(5, 5), Color::White).unwrap();
        game.place_stone(Position::new(4, 5), Color::Black).unwrap();
        game.place_stone(Position::new(10, 10), Color::White).unwrap(); // White plays elsewhere
        game.place_stone(Position::new(6, 5), Color::Black).unwrap();
        game.place_stone(Position::new(11, 11), Color::White).unwrap(); // White plays elsewhere

        // Black captures white stone at (5,5) by playing at (5,6)
        game.place_stone(Position::new(5, 6), Color::Black).unwrap();

        // Check prisoner count - black captured 1 white stone
        assert_eq!(game.prisoners, (0, 1));
    }

    #[test]
    fn test_ko_rule() {
        let mut game = Game::new();

        // Set up a proper ko situation
        //   0 1 2 3
        // 0 . B W .
        // 1 B W . W
        // 2 . B W .

        game.place_stone(Position::new(1, 0), Color::Black).unwrap();
        game.place_stone(Position::new(2, 0), Color::White).unwrap();
        game.place_stone(Position::new(0, 1), Color::Black).unwrap();
        game.place_stone(Position::new(1, 1), Color::White).unwrap();
        game.place_stone(Position::new(1, 2), Color::Black).unwrap();
        game.place_stone(Position::new(2, 2), Color::White).unwrap();
        game.place_stone(Position::new(10, 10), Color::Black).unwrap(); // Black plays elsewhere
        game.place_stone(Position::new(3, 1), Color::White).unwrap();

        // Black captures White at (1,1) by playing at (2,1)
        game.place_stone(Position::new(2, 1), Color::Black).unwrap();

        // Now (1,1) is empty, and White at (1,1) was captured
        // If White plays at (1,1), it recaptures Black at (2,1)
        // This would recreate the board position before Black's last move - ko!
        let result = game.place_stone(Position::new(1, 1), Color::White);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ko rule violation");
    }

    #[test]
    fn test_suicide_blocked() {
        let mut game = Game::new();

        // Create a surrounded position (alternating turns correctly)
        game.place_stone(Position::new(0, 1), Color::Black).unwrap();
        game.place_stone(Position::new(5, 5), Color::White).unwrap(); // White plays elsewhere
        game.place_stone(Position::new(2, 1), Color::Black).unwrap();
        game.place_stone(Position::new(6, 6), Color::White).unwrap(); // White plays elsewhere
        game.place_stone(Position::new(1, 0), Color::Black).unwrap();
        game.place_stone(Position::new(7, 7), Color::White).unwrap(); // White plays elsewhere
        game.place_stone(Position::new(1, 2), Color::Black).unwrap();

        // White tries suicide at (1,1)
        let result = game.place_stone(Position::new(1, 1), Color::White);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Suicide move not allowed");
    }

    #[test]
    fn test_pass() {
        let mut game = Game::new();

        assert_eq!(game.turn, Color::Black);
        game.pass();
        assert_eq!(game.turn, Color::White);
        game.pass();
        assert_eq!(game.turn, Color::Black);
    }

    #[test]
    fn test_reset() {
        let mut game = Game::new();

        game.place_stone(Position::new(3, 3), Color::Black).unwrap();
        game.place_stone(Position::new(4, 4), Color::White).unwrap();

        game.reset();

        // Board should be empty, turn should be Black
        assert_eq!(game.turn, Color::Black);
        assert_eq!(game.prisoners, (0, 0));
        assert!(game.board.is_empty(Position::new(3, 3)));
        assert!(game.board.is_empty(Position::new(4, 4)));
    }
}
