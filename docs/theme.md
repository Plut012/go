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
    manifest.json          # name, author, palette tokens
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
