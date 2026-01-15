# MVP Plan

## Vision
Two people playing Go together. One hosts a server, both connect from any device (phone or laptop), choose their color, and play. Simple, robust, beautiful.

---

## Scope

### In Scope
- Single game state on server
- Manual player color assignment ("I'm Black" / "I'm White" buttons)
- Standard 19×19 Go rules (placement, capture, ko, pass)
- Real-time move synchronization via WebSocket
- Reconnection (refresh page, rejoin game)
- New game reset
- Basic SVG board with circles (no theming yet)
- Mobile-friendly touch targets
- Prisoner count display
- Turn indicator

### Out of Scope (for MVP)
- Multiple concurrent games
- User accounts or authentication
- Game history or replay
- AI opponent
- Time controls
- Chat
- Advanced themes (Grim Toon comes later)
- Scoring UI (manual count for now)
- Spectators

---

## Architecture

### Single Binary Deployment
```
Rust Server (Axum)
├── Serves static frontend files (Svelte build)
├── WebSocket endpoint at /ws
└── Single global Game state

No database. No game IDs. One game. Simple.
```

### Server State
```rust
struct AppState {
    game: Arc<Mutex<Game>>,
    connections: Arc<Mutex<HashMap<SocketId, PlayerInfo>>>,
}

struct Game {
    board: [[Option<Color>; 19]; 19],
    turn: Color,
    history: Vec<u64>,  // board hashes for ko detection
    prisoners: (u32, u32),  // (black_captured, white_captured)
}

struct PlayerInfo {
    socket: WebSocket,
    color: Option<Color>,  // None until they choose
}
```

---

## User Flow

### 1. Starting a Game
- Host runs server: `cargo run` or `./go-server`
- Server starts on port 3000
- Host shares URL: `http://server-ip:3000` (or ngrok link)

### 2. Connecting
- Both players open URL in browser
- Frontend loads, immediately connects WebSocket to `/ws`
- Server sends current game state
- Player sees board and "Choose Color" buttons (Black / White)

### 3. Choosing Colors
- Each player clicks "I'm Black" or "I'm White"
- Frontend sends `{ "type": "choose_color", "color": "black" }`
- Server validates (color not already taken) and assigns
- Server broadcasts updated player assignments
- UI hides color buttons, shows "You are Black" indicator

### 4. Playing
- Player clicks intersection on their turn
- Frontend sends `{ "type": "move", "x": 10, "y": 5 }`
- Server validates:
  - Is it this player's turn?
  - Is intersection empty?
  - Would it be suicide (without capture)?
  - Would it violate ko?
- If valid: server applies move, updates state, broadcasts to all
- If invalid: server sends error back to requesting player only
- Both boards update in real-time

### 5. Passing
- Player clicks "Pass" button
- Server records pass
- If both players pass consecutively, game ends (show "Game Over")

### 6. New Game
- Either player clicks "New Game" button
- Server resets board, clears color assignments
- Both players see empty board and "Choose Color" buttons again

### 7. Reconnection
- Player closes tab, loses connection, or refreshes
- WebSocket auto-reconnects on page load
- Server sends current game state and player's assigned color
- Player continues where game is at

---

## WebSocket Protocol

### Client → Server
```json
{ "type": "choose_color", "color": "black" }
{ "type": "move", "x": 10, "y": 5 }
{ "type": "pass" }
{ "type": "reset" }
```

### Server → Client (broadcast unless noted)
```json
// Sent to all on every state change
{
  "type": "state",
  "board": [[null, "black", "white", ...], ...],
  "turn": "black",
  "prisoners": { "black": 3, "white": 5 },
  "players": { "black": true, "white": true },  // which colors are assigned
  "passes": 0  // consecutive passes (0, 1, or 2)
}

// Sent to requesting player only
{ "type": "error", "message": "Intersection occupied" }
{ "type": "error", "message": "Not your turn" }
{ "type": "error", "message": "Color already taken" }

// Sent to requesting player only (on reconnect)
{ "type": "your_color", "color": "black" }  // or null if not assigned
```

---

## Mobile Considerations

### Touch Targets
- Intersection hit areas: 30×30px minimum (board may be smaller than this, scale accordingly)
- Buttons: 44×44px minimum (iOS guidelines)
- Prevent zoom on double-tap (viewport meta tag)

### Responsive Layout
- Board scales to fit screen width (with margins)
- Portrait and landscape support
- UI controls below board (Pass, New Game buttons)
- Turn indicator and prisoner count above board

### Network Resilience
- Auto-reconnect WebSocket on disconnect (exponential backoff)
- Show "Reconnecting..." toast when connection lost
- On reconnect, server resends full state
- Optimistic UI: place stone immediately, rollback if server rejects

---

## Implementation Phases

### Phase 1: Backend Core
- [ ] Game logic module (board, rules, capture, ko detection)
- [ ] Single Game state with Arc<Mutex>
- [ ] WebSocket connection handling (accept, track, broadcast)
- [ ] Message parsing and validation
- [ ] Manual color assignment logic

### Phase 2: Frontend Core
- [ ] SVG board rendering (19×19 grid)
- [ ] WebSocket connection with auto-reconnect
- [ ] Color selection UI
- [ ] Click to place stone (send move message)
- [ ] Receive state updates, re-render board
- [ ] Pass button
- [ ] Turn indicator and prisoner count display

### Phase 3: Polish
- [ ] New Game button and reset flow
- [ ] Error feedback UI (toast or inline message)
- [ ] Mobile responsive layout
- [ ] Touch target optimization
- [ ] Loading and reconnection states
- [ ] Game over detection (two consecutive passes)

### Phase 4: Deployment
- [ ] Build script (backend + frontend together)
- [ ] Static file serving from Rust binary
- [ ] Deployment guide (local, VPS, or ngrok)
- [ ] Test on actual phones over WiFi

### Phase 5: Optional Persistence
- [ ] Serialize game state to JSON file on each move
- [ ] Load state on server startup
- [ ] Survive server restarts gracefully

---

## Technical Decisions

### Why Manual Color Assignment?
- Simplest UX: no "who connects first" confusion
- Flexible: players can switch colors for next game easily
- No implicit rules to remember

### Why No Scoring UI Yet?
- Scoring (marking dead stones, territory counting) is complex
- MVP: players count manually or use external tool
- Can add later once core gameplay is solid

### Why In-Memory State?
- No database setup or maintenance
- Instant deployment
- Optional: save to JSON file for persistence (20 lines of code)

### Why Single Game Only?
- This is for two specific people playing together
- No need for matchmaking or game lobbies
- Eliminates 90% of state management complexity

---

## Success Criteria

MVP is complete when:
1. Two people can connect from different devices
2. Both can choose colors and see each other's choice
3. They can play a full game with standard rules enforced
4. Moves appear in real-time on both screens
5. Reconnection works (refresh page, game continues)
6. Game can be reset for a new match
7. Works on mobile Safari and Chrome

---

## Next Steps After MVP

- Implement scoring UI (dead stone marking, territory calculation)
- Add "Grim Toon" theme (visual evolution based on group size)
- Add sound effects (stone placement, capture)
- Add game history / undo
- Optional: simple replay feature
- Optional: multiple game rooms (if use case expands)
