use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};

/// WebSocket connection handler
pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket) {
    // TODO: Implement WebSocket message handling
    // - Accept connections
    // - Parse messages (move, pass, choose_color, reset)
    // - Validate and apply moves
    // - Broadcast state updates
    println!("WebSocket connection established");
}
