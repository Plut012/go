# Removed Feature: AI Opponent

**Removed:** 2026-02-28
**Reason:** Simplified scope to focus on territory estimation only

## What Was Implemented

KataGo integration included both territory estimation and AI opponent functionality:

### Code Removed

**Backend (src/katago/mod.rs):**
- `get_ai_move()` method (lines 163-208)
  - Queried KataGo for best move suggestions
  - Selected move with highest visit count
  - Returned position + ownership data
  - Used 2x `max_visits` for stronger play

**WebSocket Handler (src/ws.rs):**
- `handle_ai_move()` function (lines ~415-449)
  - Triggered after human move in AI mode
  - Queried KataGo for AI response
  - Applied move to game state
  - Broadcast updated board + ownership

**JSON Types (src/katago/mod.rs):**
- `MoveInfo` struct (lines 373-382)
  - Parsed move suggestions from KataGo response
  - Fields: `move_coord`, `visits`, `winrate`, `score_lead`

### What Remains

**Territory Estimation (kept):**
- `get_ownership()` method - queries KataGo for territory ownership
- `OwnershipData` struct - stores ownership values per intersection
- Caching system - hash-based position cache
- Board serialization/GTP conversion
- Process lifecycle management

## How It Worked

1. User selects "Human vs AI" mode
2. Human places stone
3. Backend queries KataGo: `maxVisits = 200` (2x normal)
4. KataGo returns `moveInfos` array with suggestions
5. Backend selects move with highest `visits` count
6. AI move applied to board state
7. Ownership data broadcast to frontend

**Performance:** ~2-5 seconds per AI move (CPU-only, 200 visits)

## Restoration Guide

To restore AI opponent feature:

1. **Restore code:**
   - Revert commits or cherry-pick from this SHA: `[insert commit hash]`
   - Or reconstruct from `docs/todo_tasks/katago_integration.md` Phase 3-5

2. **Add to katago module:**
```rust
pub fn get_ai_move(&mut self, board: &[Vec<Option<Color>>], board_size: usize, color: Color)
    -> Result<(Position, OwnershipData), String> {
    // Query with 2x maxVisits for stronger play
    // Parse moveInfos from response
    // Select best move by visits
    // Return position + ownership
}
```

3. **Add WebSocket handler:**
```rust
async fn handle_ai_move(state: &AppState, current_turn: Color) {
    // Query katago.get_ai_move()
    // Apply move to game state
    // Broadcast state update
}
```

4. **Update frontend:**
   - Add game mode selector (Human vs Human / Human vs AI)
   - Add "AI thinking..." indicator
   - Disable board during AI turn

## Why Removed

**Complexity vs Value:**
- Territory estimation provides core learning value (understanding influence)
- AI opponent adds significant complexity (game modes, turn handling, UI)
- Simpler scope = faster iteration on territory visualization

**Still Accessible:**
- Users can practice against KataGo directly via GTP
- Focus this project on unique value: real-time territory overlay during human games

**May Restore Later:**
- If territory overlay proves valuable for learning
- After core multiplayer experience is solid
- As optional feature toggle

## References

- Full integration plan: `docs/todo_tasks/katago_integration.md`
- Progress tracker: `docs/todo_tasks/katago_progress.md`
- KataGo docs: https://github.com/lightvector/KataGo
