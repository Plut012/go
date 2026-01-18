use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crate::game::{Game, Color};

/// Unique identifier for each WebSocket connection
pub type ConnectionId = u64;

/// Information about a connected player
pub struct PlayerConnection {
    pub color: Option<Color>,
    pub sender: mpsc::UnboundedSender<String>, // Channel to send messages to this connection
}

/// Shared application state
pub struct AppState {
    pub game: Arc<Mutex<Game>>,
    pub connections: Arc<Mutex<HashMap<ConnectionId, PlayerConnection>>>,
    pub next_connection_id: Arc<Mutex<ConnectionId>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            game: Arc::new(Mutex::new(Game::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            next_connection_id: Arc::new(Mutex::new(0)),
        }
    }

    /// Generate a unique connection ID
    pub async fn new_connection_id(&self) -> ConnectionId {
        let mut id = self.next_connection_id.lock().await;
        let current = *id;
        *id += 1;
        current
    }
}
