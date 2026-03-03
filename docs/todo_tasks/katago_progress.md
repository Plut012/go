# KataGo Integration - Progress Tracker

**Last Updated**: 2026-02-05
**Current Phase**: Phase 1 Complete → Phase 2 Next

---

## Overview

Integrating KataGo to provide:
1. **AI Opponent** - Practice Go against computer
2. **Territory Estimation** - Visual overlay showing influence/direction
3. **Learning Tool** - Help players understand "flow" (Kajiwara's philosophy)

**Architecture**: Rust backend manages KataGo subprocess, sends JSON queries, receives ownership data. Frontend renders subtle heat map overlay.

---

## Phase 1: Foundation ✅ COMPLETE

### Completed Tasks

| Task | Status | Notes |
|------|--------|-------|
| Download KataGo binary | ✅ | v1.15.3 CPU/Eigen for Linux x64 (174MB) |
| Create assets directory | ✅ | `assets/katago/` with binary and configs |
| Create module skeleton | ✅ | `src/katago/mod.rs` (230 lines) |
| Create config file | ✅ | `analysis.cfg` optimized for CPU |
| Define API types | ✅ | Config, OwnershipData, Service, Query/Response |
| Asset validation | ✅ | `KataGoService::new()` validates paths |
| Process spawning | ✅ | `spawn_process()` with stdin/stdout pipes |
| Position hashing | ✅ | For caching ownership results |
| Unit tests | ✅ | Hash consistency, config defaults |
| Update documentation | ✅ | `docs/setup.md` with full setup guide |

### Module Structure

```rust
// Public API
pub struct KataGoConfig { ... }
pub struct OwnershipData { ownership: Vec<Vec<f32>>, ... }
pub struct KataGoService { ... }

impl KataGoService {
    pub fn new(config: KataGoConfig) -> Result<Self, String>
    pub fn get_ownership(&mut self, board) -> Result<OwnershipData, String>  // stub
    pub fn get_ai_move(&mut self, board, color) -> Result<(Position, OwnershipData), String>  // stub

    fn spawn_process(&mut self) -> Result<(), String>  // implemented
    fn send_query(&mut self, query) -> Result<AnalysisResponse, String>  // stub
    fn position_hash(board) -> u64  // implemented
}

// Private JSON types
struct AnalysisQuery { id, moves, maxVisits, includeOwnership }
struct AnalysisResponse { id, ownership, moveInfos }
struct MoveInfo { move_coord, visits, winrate, score_lead }
```

### Configuration

**`assets/katago/analysis.cfg`** - Optimized for CPU usage:
- `maxVisits = 100` - Fast territory estimation
- `numAnalysisThreads = 1`
- `numSearchThreadsPerAnalysisThread = 4`
- `nnMaxBatchSize = 8` - Small for CPU
- `nnCacheSizePowerOfTwo = 20` - Modest cache

### Known Issues

⚠️ **OpenSSL Dependency**: Pre-built binary requires `libssl.so.1.1` (OpenSSL 1.1)
- Arch Linux uses OpenSSL 3 by default
- **Solution**: Install `openssl-1.1` from AUR (`yay -S openssl-1.1`)
- **Alternative**: Build KataGo from source

⚠️ **Neural Network Missing**: Model file not yet downloaded
- Need to download from GitHub releases or katagotraining.org
- Recommended: b10c128 network (~20MB, fast)

---

## Phase 2: Ownership Calculation (Next)

### Goals
- Implement JSON query/response communication
- Convert board state to KataGo move format
- Parse ownership data from JSON response
- Implement caching

### Tasks

- [ ] Implement `send_query()` - Write JSON to stdin, read from stdout
- [ ] Implement board → move sequence conversion
- [ ] Implement `get_ownership()` - Full flow with caching
- [ ] Test with real KataGo process (requires OpenSSL fix)
- [ ] Add error handling and retry logic
- [ ] Performance testing

### Technical Challenges

**Board Serialization**:
```rust
// Convert [[Option<Color>; 19]; 19] to moves list
// ["B", "Q4"], ["W", "D16"], ...
```

**JSON Communication**:
```rust
// Write query to stdin
serde_json::to_writer(&mut stdin, &query)?;
writeln!(&mut stdin)?;  // KataGo expects newline-delimited JSON

// Read response from stdout
let line = stdout.read_line()?;
let response: AnalysisResponse = serde_json::from_str(&line)?;
```

**Caching**:
```rust
let hash = Self::position_hash(&board);
if let Some(cached) = self.cache.lock().unwrap().get(&hash) {
    return Ok(cached.clone());
}
// ... query KataGo ...
self.cache.lock().unwrap().insert(hash, ownership_data);
```

---

## Phase 3: AI Move Generation

### Goals
- Use KataGo's `moveInfos` to generate AI moves
- Select best move based on visits/winrate
- Return move + ownership together

### Tasks

- [ ] Extend `send_query()` to request move suggestions
- [ ] Parse `MoveInfo` from response
- [ ] Implement move selection strategy (highest visits vs winrate)
- [ ] Implement `get_ai_move()` - Full flow
- [ ] Test AI makes legal moves
- [ ] Test various difficulty levels (adjust maxVisits)

---

## Phase 4: Backend Integration

### Goals
- Add `GameMode` to game state
- Integrate KataGo service into WebSocket handler
- Broadcast ownership data to frontend

### Tasks

- [ ] Add `mode: GameMode` to `Game` struct
- [ ] Create `KataGoService` in `AppState`
- [ ] Handle AI moves in game loop
- [ ] Query ownership after each move
- [ ] Extend WebSocket protocol with ownership field
- [ ] Add error handling for KataGo failures

---

## Phase 5-9: Frontend & Testing

See `docs/todo_tasks/katago_integration.md` for full plan.

---

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Ownership query latency | < 2s | CPU-only, maxVisits=100 |
| AI move latency | < 5s | Acceptable for practice |
| Memory usage | < 500MB | KataGo process overhead |
| Cache hit rate | > 50% | For repeated positions |

---

## Dependencies Status

| Dependency | Status | Notes |
|------------|--------|-------|
| Rust (Tokio) | ✅ | Already in Cargo.toml |
| Serde JSON | ✅ | Already in Cargo.toml |
| KataGo binary | ⚠️ | Downloaded, needs OpenSSL 1.1 |
| Neural network | ⚠️ | Needs manual download |
| libzip | ✅ | System package (user installed) |
| OpenSSL 1.1 | ❌ | User needs to install from AUR |

---

## Testing Strategy

### Unit Tests
- ✅ Position hashing consistency
- ✅ Config defaults
- [ ] Board → move sequence conversion
- [ ] JSON serialization/deserialization
- [ ] Cache behavior

### Integration Tests
- [ ] Spawn KataGo process
- [ ] Send query, receive response
- [ ] Full ownership calculation flow
- [ ] AI move generation
- [ ] Process crash recovery

### Manual Testing
- [ ] Verify ownership overlay accuracy
- [ ] Test AI move quality
- [ ] Performance on low-end hardware
- [ ] Mobile device testing

---

## Documentation Status

| Document | Status | Notes |
|----------|--------|-------|
| `docs/setup.md` | ✅ Updated | Full KataGo setup guide |
| `docs/todo_tasks/katago_integration.md` | ✅ Updated | Phase 1 marked complete |
| `docs/todo_tasks/katago_progress.md` | ✅ Created | This file |
| Code comments | ✅ | All structs/methods documented |
| README.md | ⏸️ | Update after Phase 4 |

---

## Next Steps

### Immediate (Can do now)
1. ✅ Update documentation (DONE)
2. Implement JSON query/response logic (no KataGo needed for coding)
3. Write unit tests with mock data
4. Board → moves conversion

### Blocked (Needs OpenSSL)
1. Test real KataGo communication
2. Download neural network model
3. End-to-end integration testing

### Decision Point
- **Option A**: Install OpenSSL 1.1 now, test everything immediately
- **Option B**: Implement Phase 2 code first, test with mocks, then unblock
- **Option C**: Build KataGo from source to avoid dependency issues

**Recommendation**: Option A (install openssl-1.1) - fastest path to working prototype.

---

## Success Metrics

Phase 1 is complete when:
- ✅ Module compiles without errors
- ✅ Assets directory structure created
- ✅ Config file optimized
- ✅ Documentation updated
- ✅ API design validated

**Status**: ✅ All criteria met!

Phase 2 will be complete when:
- [ ] Can send query to KataGo and receive ownership data
- [ ] Ownership data correctly parsed and cached
- [ ] Integration test passes with real KataGo process

---

**Progress**: 1/9 phases complete (11%)
**Lines of Code**: 230 (katago module)
**Time Spent**: ~2 hours (Phase 1)
**Estimated Remaining**: ~8-12 hours for full integration
