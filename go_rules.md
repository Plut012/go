# Go Rules

Complete ruleset for implementing a web-based Go (Weiqi/Baduk) game.

---

## Game Setup

**Board**
- Standard: 19×19 grid of intersections
- Variants: 13×13, 9×9 for shorter games
- Stones are placed on intersections, not in squares

**Starting State**
- Empty board
- Black plays first
- Each player has unlimited stones

**Turn Order**
- Players alternate placing one stone per turn

---

## Core Mechanics

### Placing Stones
- Stones are placed on vacant intersections
- Once placed, stones never move
- Stones cannot be placed on occupied intersections

### Liberties
- A liberty is an empty intersection adjacent to a stone (horizontal or vertical, NOT diagonal)
- Corner stones have 2 liberties maximum
- Side stones have 3 liberties maximum
- Center stones have 4 liberties maximum
- Connected stones of the same color form a **group** and share liberties

### Groups
- Stones of the same color that are adjacent (horizontally or vertically) form a group
- Groups share all liberties collectively
- A group's liberties are all empty intersections adjacent to any stone in the group

---

## Capture

**Capture Condition**
- A group with zero liberties is captured
- Captured stones are immediately removed from the board
- Captured stones are kept by the capturing player as prisoners

**Capture Sequence**
1. Player places stone
2. Opponent groups with zero liberties are removed first
3. Then check if the placed stone/group has liberties

**Important**: Placement that captures opponent stones is legal even if the placed stone initially appears to have no liberties.

---

## Illegal Moves

### Occupied Intersection
- Cannot place a stone where one already exists

### Suicide Rule
- Cannot place a stone that would result in your own group having zero liberties
- Exception: Legal if the placement simultaneously captures opponent stones

---

## Ko Rule

### Simple Ko (Required)
- A move cannot immediately recreate the board position from the previous turn
- Prevents infinite capture-recapture loops
- Player must make at least one move elsewhere before recapturing

**Example**: If White captures a Black stone, Black cannot immediately recapture to return to the previous position.

---

## Game End

### Passing
- A player may pass instead of placing a stone
- Pass counts as a turn

### Game Termination
- Game ends when both players pass consecutively
- Alternative: Player resignation ends game immediately

---

## Scoring

### Japanese Rules (Territory Scoring)
**Score = Surrounded empty territory - Captured stones**

- Count empty intersections surrounded by your stones
- Subtract the number of your stones captured by opponent
- Player with higher score wins
- Komi: Add 6.5 points to White's score

**Notes:**
- Filling own territory LOSES points
- Dame (neutral points) are worthless
- Seki points are neutral (neither player scores them)

### Dead Stone Agreement
After both players pass, before final scoring:
1. Players identify which stones are **dead** (would inevitably be captured if play resumed)
2. Each player marks stones they believe are dead
3. Both players must agree on all dead stones
4. If disagreement occurs, players resume play to resolve the dispute
5. Once agreement is reached, dead stones are removed and added to the opponent's prisoner count
6. Scoring then proceeds

**Implementation**: Requires a UI for players to mark dead stones and confirm agreement, with option to resume play if consensus cannot be reached.

---

## Special Situations

### Seki (Mutual Life)
- Groups that cannot be captured but lack two eyes
- Neither player can play without being captured themselves
- Scoring: Seki points are neutral (neither player scores them)
- Both groups remain on the board at game end

### Snapback
- Technique where allowing a capture sets up an immediate larger recapture
- Legal and tactical play—not an edge case to handle specially

### Dead Stones
- Stones that cannot avoid capture if opponent plays optimally
- Removed during scoring (after both players pass)
- Implementation: Requires agreement between players OR full game tree analysis

---

## Implementation Priorities

### Core Requirements
1. Board representation (choices: 19×19, 13x13, 9x9 grids)
2. Stone placement validation
3. Liberty counting for groups
4. Capture detection and removal
5. Simple ko rule enforcement
6. Pass mechanism
7. Game end detection (two consecutive passes)
8. Dead stone agreement interface
9. Territory and prisoner counting (Japanese rules)

### Robustness Requirements
1. Validate all moves before applying
2. Handle suicide rule consistently
3. Detect ko violations
4. Prevent occupied intersection placement
5. Track game history for ko detection

### Edge Cases to Test
1. Snapback captures
2. Seki formations
3. Large group captures
4. Ko situations
5. Suicide attempts (legal and illegal)
6. Multi-stone captures in single move

---

## Ruleset Recommendation

For a simple, robust implementation:
- **Scoring**: Japanese (territory scoring)
- **Suicide**: Forbidden, except when capturing
- **Ko**: Simple ko
- **Board**: 19×19 standard, with 13×13 and 9×9 options
- **Komi**: 6.5 points to White

This combination minimizes implementation complexity while maintaining standard competitive play compatibility.

---

## Sources

- [Rules of Go - Wikipedia](https://en.wikipedia.org/wiki/Rules_of_Go)
- [Go Game Rule Sets: Chinese, Japanese, Korean, AGA and others](https://gomagic.org/go-rule-sets/)
- [Go rules – Japanese vs Chinese – Polgote Blog](https://polgote.com/en/blog/go-rules-japanese-vs-chinese/)
- [Comparison of Some Go Rules | British Go Association](https://www.britgo.org/rules/compare.html)
- [AGA Concise Rules of Go](https://www.cs.cmu.edu/~wjh/go/rules/AGA.concise.html)
- [The Chinese Rules of Weiqi (Go) (2002)](https://weiqi.org.sg/wp-content/uploads/2025/09/The-Chinese-Rules-of-Weiqi.pdf)
