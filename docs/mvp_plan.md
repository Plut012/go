# MVP Plan

## Vision

**Build a working Go game first. Make it magical later.**

The MVP is standard Go—traditional rules, simple visuals, rock-solid implementation. Two players connect, choose Black or White, and play a complete game. No fancy themes. No evolving pieces. Just circles on a board and bulletproof game logic.

The theme system (scouts → legions, army selection, visual evolution) comes after the game works perfectly.

---

## Scope

### In Scope
- Single game state on server (ephemeral—no persistence)
- Manual color assignment (simple "Black" / "White" buttons)
- Standard 19×19 Go rules (placement, capture, ko, pass)
- Real-time move synchronization via WebSocket
- Reconnection (refresh page, game state restored)
- New game reset
- **Default theme: basic SVG circles (black/white)**
- Laptop screen assumed (defer mobile optimization)
- Prisoner count display
- Turn indicator

### Out of Scope (for MVP)
- Multiple concurrent games
- User accounts or authentication
- Game history or replay
- AI opponent
- Time controls
- Chat
- Mobile optimization (laptop screen assumed)
- Game state persistence (ephemeral games accepted)
- Scoring UI (manual territory counting)
- **Visual themes (Grim Toon, army selection, evolving pieces)**
- **Theme selection UI**
- **Faction identity (just "Black" and "White" for now)**

The goal is a complete, correct implementation of Go. Theme magic comes after.

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

## Testing Strategy

### Principle: Test Game Logic, Trust the Framework

The game rules are complex. The web framework (Axum) and UI framework (Svelte) are not. Focus testing effort where bugs hide: capture detection, ko validation, liberty counting.

### Game Logic Tests (Rust Unit Tests)

Test game rules in complete isolation. No WebSocket. No server. Just pure functions.

```rust
// src/game/rules.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_capture() {
        // Place white stones to surround black stone
        // Verify black stone is captured
    }

    #[test]
    fn test_ko_detection() {
        // Create ko situation
        // Verify immediate recapture is blocked
    }

    #[test]
    fn test_suicide_illegal() {
        // Attempt move that would have zero liberties
        // Verify move is rejected
    }

    #[test]
    fn test_suicide_legal_when_capturing() {
        // Place stone with zero liberties that captures opponent
        // Verify move is allowed (capture happens first)
    }
}
```

**Coverage Required:**
- Basic stone placement (valid and invalid positions)
- Liberty counting for single stones and groups
- Capture detection (single stone, groups, multiple groups in one move)
- Ko rule (simple ko only for MVP)
- Suicide rule (illegal by default, legal when capturing)
- Group detection (finding connected stones)
- Snapback (special case of capture—should work naturally)

**Run tests:** `cargo test`

### Integration Testing (Manual Protocol Verification)

WebSocket flows are simple enough to test manually. Use a checklist.

**Test Scenarios:**

1. **Two Player Connection**
   - Open two browser windows
   - Both connect successfully
   - Choose colors (Black and White)
   - Verify both see color assignments

2. **Basic Move Flow**
   - Player A places stone
   - Player B sees stone appear immediately
   - Player B places stone
   - Player A sees stone appear immediately

3. **Capture**
   - Set up surrounded group
   - Place final stone to capture
   - Verify captured stones removed on both screens
   - Verify prisoner count updated

4. **Ko Detection**
   - Create ko situation
   - Attempt immediate recapture
   - Verify move rejected with error message
   - Pass or play elsewhere
   - Verify recapture now allowed

5. **Reconnection**
   - Mid-game, refresh one player's browser
   - Verify WebSocket reconnects
   - Verify full game state restored
   - Verify play continues normally

6. **Pass and Game End**
   - Both players pass consecutively
   - Verify "Game Over" state

**Tools:**
- Two browser windows (Chrome + Firefox, or two profiles)
- Browser DevTools Network tab (watch WebSocket messages)
- Server logs (`println!` debugging is fine for MVP)

### Acceptance Criteria (Definition of Done)

MVP is complete when these scenarios work correctly:

- [ ] Can play a full game from empty board to two consecutive passes
- [ ] Captures work correctly (single stones and groups)
- [ ] Ko rule prevents immediate recapture
- [ ] Suicide rule blocks illegal moves
- [ ] Both players see identical board state at all times
- [ ] Reconnection restores game state correctly
- [ ] Prisoner count is accurate
- [ ] Turn enforcement works (can't play out of turn)

### What We're NOT Testing (Trust the Tools)

- Axum's WebSocket implementation (it works)
- Svelte's reactivity (it works)
- Browser's SVG rendering (it works)
- JSON serialization (serde handles it)

Focus effort on game logic where bugs actually hide.

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
