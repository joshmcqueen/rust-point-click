# Project direction

## Purpose

Build a portable Rust point-and-click adventure inspired by LucasArts
games from the 1980s and 1990s.

This is primarily a learning and creative project: learn Rust through
small, understandable implementations while making a fun game using
custom AI-generated artwork and pixel art.

Develop on macOS. Target macOS, Windows, and Linux. A separate Windows
PC with an RTX 5090 will likely be used to generate game assets.

## Visual direction

- Modern-resolution pixel art with a retro aesthetic.
- Do not constrain the game to a low resolution such as 320×180.
- Initial scene design coordinates: 1920×1080.
- Support a resizable window, including comfortable use at 1280×720.
- Initially preserve the 16:9 composition with letterboxing.
- Ultrawide support may eventually reveal additional scenery.
- Preserve intentional pixel edges and consistent asset pixel density.
- Atmospheric lighting is part of the intended look: characters should
  respond to nearby torches, lights, and shadowed areas.
- Start with simple local lighting. Normal maps and dynamic occlusion
  shadows are optional later enhancements.

## Technology and code

- Start with Rust and Macroquad.
- Macroquad is provisional: evaluate it through a playable visual
  prototype before committing to more elaborate systems.
- Bevy was considered; Macroquad was favored for its simpler learning
  curve. Maintenance and platform reliability remain considerations.
- Prefer approachable Rust, ordinary structs/enums, and modest modules.
- Avoid speculative frameworks and premature abstraction.
- Keep game state explicit and separate from rendering.
- Begin with room content in Rust; move repeated content into data files
  when actual needs become clear.
- Explain important implementation choices so the user can learn Rust.

## Initial prototype

Build one room containing:
- Temporary artwork and a character.
- Click-to-walk movement on a simple unobstructed floor.
- Frame-rate-independent movement with predictable stopping.
- Correct mouse mapping during resizing and letterboxing.
- Ambient darkness and a warm torch affecting room and character.
- A movable light to evaluate the lighting approach.
- Visible, concise controls.

Do not add physics, advanced pathfinding, dynamic shadows, or a full
adventure framework to this first prototype.

## Milestones

1. Playable visual prototype: room, walking, resizing, lighting.
2. Hotspots, hover feedback, and object descriptions.
3. Picking up objects and inventory selection.
4. One complete puzzle, such as using a key to open a door.
5. Connected rooms with persistent world state.
6. One NPC conversation and save/load.
7. A small, complete three-screen demo.

Suggested demo scope: one objective, one NPC, three or four collectible
objects, and two or three connected puzzles; approximately 5–10 minutes.

After the demo, explore AI-assisted planning of an entire act, including
puzzles, characters, narration, and dialogue. Voice synthesis via an
open-source tool or an API such as ElevenLabs is a later consideration.

## Working approach

- Deliver small, playable milestones and evaluate them before expanding.
- Temporary assets are welcome; establish visual constraints before
  producing substantial finished artwork.
- Use agents/subagents for useful independent implementation tasks,
  with clear ownership and integration.
- Maintain the current checklist in TASKS.md.
- Maintain launch commands, controls, and architecture notes in README.md.
- Test portability early, including on the user's Windows PC.
- Distinguish compilation and automated tests from actual runtime checks.
- Never claim a platform or visual behavior was tested without evidence.