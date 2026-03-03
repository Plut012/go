# KataGo Integration - Territory Estimation

## Vision

**Add subtle territory estimation overlay to help players learn the flow and direction of play.**

Enable real-time territory visualization where players can:
- See subtle territory/influence visualization during play
- Understand direction of play through visual feedback
- Learn the flow of the game (Kajiwara's philosophy)
- Works seamlessly on any device

**Note:** AI opponent feature was removed 2026-02-28 to simplify scope. See `docs/archive/removed_feature_ai_opponent.md`.

The integration maintains architectural cleanliness: backend handles all game logic and KataGo coordination, frontend only renders the visual overlay.

---

## Goals

### Primary Goals
- Display territory ownership estimation as subtle visual overlay
- Teach "direction of play" through visual feedback
- Work seamlessly on mobile (phone practice mode)
- Maintain zero-dependency philosophy (single binary + assets)
- Fast response time (~1s per query on CPU)

### Learning Focus
- Help players understand **influence** and **direction** (Kajiwara's core teaching)
- Visual feedback shows where stones project power
- See contested vs secure territory in real-time
- Learn from exploring variations, not just playing strong moves

---

## Scope

### In Scope
- KataGo integration in backend (spawn as subprocess)
- JSON Analysis Engine mode (efficient queries)
- Territory ownership estimation per intersection
- Subtle overlay visualization (heat map or opacity-based)
- Ownership data sent via WebSocket to frontend
- CPU-only support (works everywhere, no GPU required)
- Cache ownership calculations (only recalculate after moves)
- Optimized for speed (50 visits = ~1s response time)

### Out of Scope
- AI opponent (removed - see `docs/archive/removed_feature_ai_opponent.md`)
- GPU acceleration (nice-to-have, not required)
- Move suggestions/hints UI (focus on territory only)
- Detailed analysis/variation tree exploration
- Win rate percentages or detailed statistics
- GTP protocol (using JSON Analysis Engine instead)

### Maybe Later
- AI difficulty presets (beginner, intermediate, advanced)
- "Exploration mode" - place stones freely, see ownership change
- Influence/direction arrows showing stone projection
- AI commentary or teaching messages

---

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────┐
│ Rust Backend (Axum)                                 │
│                                                      │
│  ┌────────────────────────────────────────────┐    │
│  │ Game Logic (existing)                      │    │
│  │ - Board state, rules, validation           │    │
│  └─────────────┬──────────────────────────────┘    │
│                │                                     │
│  ┌─────────────▼──────────────────────────────┐    │
│  │ KataGo Service (new module)                │    │
│  │ - Spawn katago binary on startup           │    │
│  │ - Manage analysis process lifecycle        │    │
│  │ - Send position queries (JSON)             │    │
│  │ - Receive ownership data                   │    │
│  │ - Request AI moves                         │    │
│  │ - Cache results by position hash           │    │
│  │ - Async with Tokio                         │    │
│  └─────────────┬──────────────────────────────┘    │
│                │ stdin/stdout                       │
│                │ (JSON queries/responses)           │
└────────────────┼────────────────────────────────────┘
                 │
         ┌───────▼────────────────┐
         │ KataGo Process         │
         │ (analysis mode)        │
         │ - Reads JSON from stdin│
         │ - Writes JSON to stdout│
         └────────────────────────┘
```

### Data Flow

**AI Move Generation:**
1. Human player makes move → backend validates
2. Backend queries KataGo for next move
3. KataGo returns suggested move + ownership data
4. Backend applies AI move to game state
5. Backend broadcasts: `{ board_state, ai_move, ownership }`
6. Frontend renders move + territory overlay

**Territory Overlay (any game):**
1. After any move, backend queries KataGo for ownership
2. KataGo returns ownership array (-1 to +1 per intersection)
3. Backend sends to frontend via WebSocket
4. Frontend renders subtle heat map overlay on SVG board

---

## Technical Design

### Backend Module Structure

**New file: `src/katago/mod.rs`**

```rust
pub struct KataGoService {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    cache: Arc<Mutex<HashMap<u64, OwnershipData>>>,
    config: KataGoConfig,
}

pub struct KataGoConfig {
    binary_path: PathBuf,
    model_path: PathBuf,
    config_path: PathBuf,
    max_visits: u32,          // Lower = faster, 50-100 for territory estimation
    enable_ownership: bool,
}

pub struct OwnershipData {
    ownership: Vec<Vec<f32>>,      // -1.0 to 1.0 per intersection
    ownership_stdev: Vec<Vec<f32>>, // Confidence/uncertainty
}

pub struct AnalysisQuery {
    id: String,
    moves: Vec<(Color, (u8, u8))>,
    max_visits: u32,
    include_ownership: bool,
    include_moves_ownership: bool,
}

pub struct AnalysisResponse {
    id: String,
    ownership: Option<Vec<Vec<f32>>>,
    move_infos: Vec<MoveInfo>,  // For AI move selection
}

impl KataGoService {
    pub async fn new(config: KataGoConfig) -> Result<Self>;
    pub async fn get_ownership(&mut self, board: &Board) -> Result<OwnershipData>;
    pub async fn get_ai_move(&mut self, board: &Board) -> Result<(Move, OwnershipData)>;
    async fn send_query(&mut self, query: AnalysisQuery) -> Result<AnalysisResponse>;
    fn position_hash(board: &Board) -> u64;  // For caching
}
```

**Modified: `src/game/mod.rs`**

```rust
pub enum GameMode {
    HumanVsHuman,
    HumanVsAI { ai_color: Color },
}

pub struct Game {
    board: [[Option<Color>; 19]; 19],
    turn: Color,
    mode: GameMode,
    // ... existing fields
}
```

**Modified: `src/websocket.rs`**

```rust
// New message types
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    ChooseColor { color: Color },
    SetGameMode { mode: String, ai_color: Option<Color> },  // NEW
    Move { x: u8, y: u8 },
    Pass,
    Reset,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ServerMessage {
    State {
        board: Vec<Vec<Option<Color>>>,
        turn: Color,
        ownership: Option<Vec<Vec<f32>>>,  // NEW
        mode: String,                       // NEW
        // ... existing fields
    },
    Error { message: String },
    YourColor { color: Option<Color> },
}
```

### Frontend Changes

**New Svelte component: `OwnershipOverlay.svelte`**

```svelte
<script>
  export let ownership = null;  // 2D array of -1.0 to 1.0
  export let boardSize = 19;
  export let cellSize = 30;

  // Convert ownership to subtle color overlay
  function getOverlayColor(value) {
    if (!value) return 'transparent';

    // Subtle heat map: -1 = black territory, +1 = white territory
    const intensity = Math.abs(value) * 0.3;  // Max 30% opacity
    const color = value < 0 ? '0,0,0' : '255,255,255';
    return `rgba(${color},${intensity})`;
  }
</script>

{#if ownership}
  <g class="ownership-overlay">
    {#each ownership as row, y}
      {#each row as value, x}
        <rect
          x={x * cellSize}
          y={y * cellSize}
          width={cellSize}
          height={cellSize}
          fill={getOverlayColor(value)}
          pointer-events="none"
        />
      {/each}
    {/each}
  </g>
{/if}
```

**Modified: `Board.svelte`**

- Add `<OwnershipOverlay>` component beneath stones layer
- Add game mode selector UI (Human vs Human / Human vs AI)
- Add AI color selector (if AI mode)
- Show "AI is thinking..." indicator during AI moves

---

## Installation & Deployment

### Required Assets

**1. KataGo Binary**
- Download from: https://github.com/lightvector/KataGo/releases
- For MVP: `katago-v1.15.1-eigen-linux-x64` (CPU-only, ~40MB)
- Place in: `assets/katago/katago` (executable)

**2. Neural Network Model**
- Download from: https://katagotraining.org/networks/
- For MVP: Small/fast network (~20-50MB)
  - Example: `kata1-b10c128-s1141046784-d204142634.bin.gz`
  - Smaller network = faster analysis, still good for territory estimation
- Place in: `assets/katago/model.bin.gz`

**3. Analysis Config File**
- Create: `assets/katago/analysis.cfg`
- Minimal config for CPU-only analysis mode:

```cfg
# Analysis mode config
logFile = katago.log
logAllGTPCommunication = false
logSearchInfo = false

# CPU-only backend
numSearchThreads = 4
nnCacheSizePowerOfTwo = 20

# Analysis settings
maxVisits = 100
maxPlayouts = 10000000

# Rules
rules = tromp-taylor
koRule = POSITIONAL
scoringRule = AREA
```

**4. Startup Script**
- Backend checks for assets on startup
- Clear error message if missing: "KataGo binary not found at assets/katago/katago"
- Option to disable AI features if assets missing (graceful degradation)

### Deployment Impact

**Binary size increase:**
- Before: ~5-10MB (Rust binary)
- After: ~60-100MB (Rust binary + KataGo + model)
- Still single self-contained deployment

**Runtime requirements:**
- No change (still zero dependencies)
- CPU usage: +1-2 cores when AI is active
- Memory: +200-500MB for KataGo process

---

## WebSocket Protocol Changes

### New Client Messages

```json
// Switch to AI mode
{
  "type": "set_game_mode",
  "mode": "human_vs_ai",
  "ai_color": "white"
}

// Switch back to human vs human
{
  "type": "set_game_mode",
  "mode": "human_vs_human"
}
```

### Updated Server State Message

```json
{
  "type": "state",
  "board": [[null, "black", "white", ...], ...],
  "turn": "black",
  "prisoners": { "black": 3, "white": 5 },
  "players": { "black": true, "white": true },
  "passes": 0,

  // NEW FIELDS
  "mode": "human_vs_ai",
  "ai_color": "white",
  "ownership": [
    [-0.95, -0.82, -0.45, 0.0, 0.12, ...],  // Row 1: -1 = black, +1 = white
    [-0.89, -0.76, -0.32, 0.05, 0.23, ...], // Row 2
    // ... 19 rows total
  ],
  "ai_thinking": false  // True while waiting for AI move
}
```

---

## Implementation Phases

### Phase 1: KataGo Integration Foundation ✅ COMPLETE
- [x] Download and test KataGo binary locally (manual verification)
- [x] Create `src/katago/mod.rs` module skeleton (230 lines)
- [x] Add assets directory structure (`assets/katago/`)
- [x] Create minimal `analysis.cfg` config file
- [ ] Implement `KataGoService::new()` - spawn process, verify communication (partial - validation done)
- [ ] Implement basic JSON query/response parsing
- [ ] Write unit test: spawn KataGo, send query, receive response
- [ ] **Blocker**: Resolve OpenSSL 1.1 dependency for pre-built binary

**Status**: Foundation complete. Module structure in place with full API design. Config file optimized for CPU. Binary downloaded but needs OpenSSL 1.1 compat library.

**Files Created**:
- `src/katago/mod.rs` - Full module with types, service, and stubs
- `assets/katago/analysis.cfg` - CPU-optimized configuration
- `assets/katago/katago` - Binary executable (v1.15.3)

**Next**: Install OpenSSL 1.1 compat OR implement JSON query/response (can be tested with mock later)

### Phase 2: Ownership Calculation
- [ ] Implement `get_ownership()` method
- [ ] Convert board state to KataGo move format
- [ ] Parse ownership response from JSON
- [ ] Add position caching with board hash
- [ ] Test: query ownership for simple positions, verify output format
- [ ] Test: cache hit/miss behavior

### Phase 3: AI Move Generation
- [ ] Implement `get_ai_move()` method
- [ ] Parse `moveInfos` from response
- [ ] Select best move (highest visit count or win rate)
- [ ] Return move + ownership data together
- [ ] Test: generate moves for various positions
- [ ] Test: AI makes legal moves only

### Phase 4: Backend Game Logic Integration
- [ ] Add `GameMode` enum to game state
- [ ] Add mode switching logic (human vs human / human vs AI)
- [ ] Integrate AI move generation into game loop
- [ ] After each move, query ownership (cached)
- [ ] Broadcast ownership data via WebSocket
- [ ] Handle AI "thinking" state (don't block other operations)

### Phase 5: Frontend - Game Mode UI
- [ ] Add game mode selector (radio buttons or toggle)
- [ ] Add AI color selector (only visible in AI mode)
- [ ] Update WebSocket message handlers for new message types
- [ ] Show "AI is thinking..." indicator when AI turn
- [ ] Disable board interaction during AI turn

### Phase 6: Frontend - Ownership Overlay
- [ ] Create `OwnershipOverlay.svelte` component
- [ ] Integrate into `Board.svelte` (layer beneath stones)
- [ ] Implement subtle heat map visualization
  - Black territory: subtle dark overlay
  - White territory: subtle light overlay
  - Contested: no overlay or neutral color
- [ ] Add toggle to show/hide overlay (user preference)
- [ ] Test: ensure overlay doesn't interfere with stone placement clicks

### Phase 7: Performance & Polish
- [ ] Implement async ownership queries (don't block game moves)
- [ ] Add timeout handling (KataGo query takes too long)
- [ ] Error handling: KataGo process crashes, restart gracefully
- [ ] Adjust `maxVisits` for good speed/quality balance
- [ ] Test on phone: verify overlay renders correctly
- [ ] Test on phone: verify AI response time acceptable

### Phase 8: Testing & Validation
- [ ] Unit tests: KataGo service methods
- [ ] Integration test: full game against AI
- [ ] Manual test: ownership overlay accuracy
- [ ] Manual test: game mode switching mid-game
- [ ] Manual test: AI makes reasonable moves
- [ ] Performance test: ownership query latency

### Phase 9: Documentation
- [ ] Update `docs/setup.md` with KataGo installation steps
- [ ] Document asset download URLs
- [ ] Add troubleshooting section (common errors)
- [ ] Document ownership overlay visualization design decisions

---

## Test Cases

### Unit Tests (Rust)

**KataGo Service Tests:**

```rust
#[tokio::test]
async fn test_spawn_katago_process() {
    // Spawn KataGo, verify process is running
    // Send simple query, verify response received
}

#[tokio::test]
async fn test_ownership_query() {
    // Create simple board position
    // Query ownership
    // Verify response has 19x19 array of floats
    // Verify values in range -1.0 to 1.0
}

#[tokio::test]
async fn test_ai_move_generation() {
    // Create board position
    // Request AI move
    // Verify move is legal
    // Verify move coordinates are valid
}

#[tokio::test]
async fn test_position_caching() {
    // Query ownership for position A
    // Query ownership for position A again
    // Verify cache hit (no second query to KataGo)
    // Query ownership for position B
    // Verify cache miss (new query to KataGo)
}

#[tokio::test]
async fn test_katago_process_restart() {
    // Spawn KataGo
    // Kill process manually
    // Send query
    // Verify service detects failure and restarts process
}
```

### Integration Tests (Manual)

**Scenario 1: AI Opponent Plays Legal Moves**
- Start game in AI mode (human = black, AI = white)
- Play several moves
- Verify AI responds with legal moves only
- Verify game state updates correctly after each AI move

**Scenario 2: Ownership Overlay Accuracy**
- Set up position with clear black/white territory
- Toggle ownership overlay on
- Verify black territory shows dark overlay
- Verify white territory shows light overlay
- Verify contested areas show minimal/no overlay

**Scenario 3: Mode Switching**
- Start in human vs human mode
- Switch to human vs AI mid-game
- Verify AI takes over for selected color
- Verify ownership overlay appears
- Switch back to human vs human
- Verify ownership overlay still works

**Scenario 4: Mobile Performance**
- Load game on phone
- Enable AI mode
- Verify AI responds within 2-3 seconds
- Verify ownership overlay renders clearly
- Verify no lag when placing stones

**Scenario 5: Error Handling**
- Remove KataGo binary temporarily
- Start server
- Verify clear error message
- Verify graceful fallback (AI mode disabled)

---

## Acceptance Criteria

Feature is complete when:

- [ ] Can start game in AI mode with configurable AI color
- [ ] AI generates legal moves consistently
- [ ] AI responds within reasonable time (< 5 seconds on CPU)
- [ ] Ownership overlay displays after each move
- [ ] Overlay is subtle and doesn't obscure board
- [ ] Can toggle ownership overlay on/off
- [ ] Can switch between human vs human and human vs AI modes
- [ ] Works on mobile (phone screen, touch input)
- [ ] KataGo process lifecycle handled gracefully (startup, shutdown, crashes)
- [ ] Clear error messages if KataGo assets missing
- [ ] Documentation updated with installation steps

---

## Configuration Options

**Adjustable Parameters (for tuning):**

```rust
pub struct KataGoConfig {
    // Speed vs strength tradeoff
    max_visits: u32,           // 50 = very fast, 500 = strong

    // Visual overlay settings
    ownership_opacity: f32,    // 0.0 to 1.0, how visible the overlay is

    // Performance settings
    cache_enabled: bool,
    cache_max_size: usize,

    // Paths (for deployment flexibility)
    binary_path: PathBuf,
    model_path: PathBuf,
    config_path: PathBuf,
}
```

**Recommended defaults:**
- `max_visits: 100` - Good balance for territory estimation
- `ownership_opacity: 0.3` - Subtle but visible
- `cache_enabled: true` - Significant performance improvement

---

## Future Enhancements (Not in Scope)

- **Exploration Mode**: Place stones freely, see ownership change in real-time
- **Difficulty Levels**: Beginner AI (weakened), Intermediate, Advanced
- **Direction Arrows**: Show influence projection from groups (Kajiwara-style)
- **Move Suggestions**: Highlight top AI suggestions (teaching mode)
- **Variation Analysis**: Explore "what if" sequences
- **AI Commentary**: Text explanations of good/bad moves
- **GPU Support**: Faster analysis with CUDA/OpenCL backends

---

## Risks & Mitigations

**Risk: KataGo process management complexity**
- Mitigation: Use tokio::process for async handling, implement health checks

**Risk: Slow AI response time on weak hardware**
- Mitigation: Adjustable `maxVisits`, test on low-end devices early

**Risk: Large binary size with assets**
- Mitigation: Document separate asset download, optional AI features

**Risk: Ownership overlay confusing for beginners**
- Mitigation: Default to off, clear toggle UI, gentle visual design

**Risk: KataGo compatibility issues across platforms**
- Mitigation: Start with Linux CPU-only (widest compatibility), document tested platforms

---

## Success Criteria

Integration is successful when:

1. Players can practice against AI from their phone
2. Ownership overlay teaches "direction of play" visually
3. Implementation maintains clean architecture (backend handles logic)
4. Zero new runtime dependencies
5. Performance is acceptable on CPU-only hardware
6. Code is maintainable and well-documented

This feature enables the core vision: **learning to feel the flow of Go** through visual feedback and solo practice.
