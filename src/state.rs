use std::sync::Arc;
use tokio::sync::Mutex;
use crate::game::Game;

/// Shared application state
pub struct AppState {
    pub game: Arc<Mutex<Game>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            game: Arc::new(Mutex::new(Game::new())),
        }
    }
}
