# Theme System

## Architecture: Complete Independence

The backend emits pure game state. It has no knowledge of colors, sprites, animations, or sound. It answers only: what is on the board, which groups exist, whose turn it is, and who has won.

The frontend consumes this state and renders it however it pleases.

```
┌─────────────────┐       JSON/WebSocket       ┌─────────────────┐
│                 │  ───────────────────────►  │                 │
│   Rust Backend  │     { board, groups,       │    Frontend     │
│   (game logic)  │       turn, captures }     │    (themes)     │
│                 │  ◄───────────────────────  │                 │
└─────────────────┘       { move: [x,y] }      └─────────────────┘
```

**This means:**
- Themes are fully swappable without touching backend code
- New themes cannot introduce game logic bugs
- The backend can be tested in complete isolation
- Multiple frontends (web, mobile, terminal) can share one backend

A theme is a folder containing assets and styling. The game engine never reads it.

---

## Theme: Grim Toon

**Aesthetic:** Minimalist dark fantasy. Warhammer 40K's brooding atmosphere meets Lord of the Rings' epic scale, filtered through Clash Royale's approachable, cartoonish style.

### Visual Direction

**Tone:** Serious subject matter, playful execution. War is hell, but the soldiers are chunky and expressive.

**Palette:** Muted earth tones, desaturated golds, deep shadows. Occasional warm accent (torch glow, molten metal) to draw the eye.

**Line work:** Bold outlines, slightly exaggerated proportions. Characters are readable at small sizes.

### Stone Evolution

Stones are not stones—they are units. As groups grow, they visually evolve:

| Group Size | Black (Attackers) | White (Defenders) |
|------------|-------------------|-------------------|
| 1 | Lone scout | Peasant militia |
| 2-3 | Patrol | Guard post |
| 4-6 | War band | Fortified camp |
| 7-10 | Battalion + banner | Stone keep |
| 11+ | Legion with siege gear | Citadel with towers |

### Territory

Controlled empty space becomes land: scorched earth for attackers, cultivated fields for defenders. Contested zones show battle scars.

### Captures

When a group is captured, a brief, satisfying animation: troops scatter, walls crumble. Nothing gory—cartoonish defeat, like units poofing in Clash Royale.

### Audio (Future)

Ambient: distant drums, wind, occasional crow.
Placement: metallic clink (attacker), stone thud (defender).
Capture: collective groan, crumbling stone.

---

## Theme File Structure

```
/themes
  /grim-toon
    manifest.json          # name, author, palette tokens, faction identity
    theme.css              # colors, fonts, animations
    /pieces
      black-1.svg          # scout
      black-2.svg          # patrol
      black-5.svg          # war band
      ...
      white-1.svg
      white-territory.svg
    /audio (optional)
      place-black.wav
      capture.wav
```

Themes are loaded at runtime. Switching themes requires zero backend changes.

---

## Future: Army Selection & Faction Identity

### The Vision

Choosing your color should feel like choosing your army. Not "I'm Black" but "I command the Grim Legion." Each theme becomes a faction with identity, aesthetic, and presence.

This transforms Go from abstract strategy to tactical fantasy—while keeping the rules pure.

### From Colors to Factions

**Phase 1: Shared Battlefield (Post-MVP)**

Both players see the same theme. But instead of "Black" and "White," they choose faction roles:

```
┌──────────────────────────────────┐
│   Choose Your Army               │
│                                  │
│  [ Grim Legion ]  [ Keeper's Watch ]
│   (Attackers)      (Defenders)   │
│   Scout → Legion   Militia → Citadel │
└──────────────────────────────────┘
```

The board shows one aesthetic, but each side has thematic identity.

**Phase 2: Asymmetric Armies (Later)**

Each player picks their own theme. Your screen shows your pieces in your chosen aesthetic, opponent pieces in theirs.

```
Player A sees:
  Their pieces: Grim Legion visuals
  Opponent pieces: Celestial Order visuals

Player B sees:
  Their pieces: Celestial Order visuals
  Opponent pieces: Grim Legion visuals
```

"My scorched earth vs your sacred gardens." "My siege engines vs your arcane towers."

The board becomes a clash of worlds.

### Manifest Structure: Faction Identity

Themes define faction metadata in their manifest:

```json
{
  "name": "Grim Toon",
  "author": "You",
  "version": "1.0.0",

  "factions": {
    "black": {
      "name": "The Grim Legion",
      "tagline": "Scorched earth and siege",
      "description": "Relentless attackers who leave ash in their wake.",
      "preview": "pieces/black-5.svg"
    },
    "white": {
      "name": "Keeper's Watch",
      "tagline": "Stone and sanctuary",
      "description": "Steadfast defenders who hold the line.",
      "preview": "pieces/white-5.svg"
    }
  },

  "pieces": {
    "black": {
      "1": "pieces/black-1.svg",
      "2-3": "pieces/black-2.svg",
      "4-6": "pieces/black-5.svg",
      "7-10": "pieces/black-7.svg",
      "11+": "pieces/black-11.svg"
    },
    "white": { ... }
  }
}
```

The frontend reads faction data and displays it. Simple themes (like `default`) omit faction metadata—just "Black" and "White" work fine.

### Why This Works

**It's Optional:** Minimal theme doesn't need factions. Just circles labeled "Black" and "White."

**It's Extensible:** Each new theme defines its own faction identity. No code changes needed.

**It's Tasteful:** Faction names are flavor, not mechanics. The game rules never change. A scout is still one stone. A legion is still 11+ stones. Only the *presentation* evolves.

**It's Clever:** Like Warhammer players choosing their army or Magic players building their deck, choosing your theme becomes an expression of identity. "I play Grim Legion" says something about you. "I play Celestial Order" says something else.

The board remembers. The stones respond. The factions emerge.

### Implementation Path

**MVP:** No themes yet. Just circles. Get the game working.

**Phase 1:** Add `grim-toon` theme. Both players see it. Faction names appear in UI.

**Phase 2:** Add theme selection. Players choose shared battlefield before game starts.

**Phase 3:** Multiple themes exist (`grim-toon`, `celestial-order`, `shadow-clan`, etc.). Players choose from palette.

**Phase 4:** Asymmetric themes. Each player picks their army. Board shows mixed visuals. "My world vs yours."

Each phase adds magic without breaking what came before. The architecture supports the full vision from day one—the data structure is already there, waiting.

---

## Philosophy

Themes are not skins. They are **interpretations** of the same underlying truth.

The backend knows: "There is a group of 7 stones at these coordinates."

The frontend interprets: "The Grim Legion has raised a battalion with a banner."

Both are true. One is mechanical. One is meaningful. The game exists in the space between.
