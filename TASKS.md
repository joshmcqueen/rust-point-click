# Prototype checklist

- [x] Inspect folder and Apple development tools; no existing files or repository instructions found.
- [x] Parallel agents: room art and pure movement/mapping, integrated by primary agent.
- [x] Install project-local official Rust toolchain (Rust was absent).
- [x] Implement room, character, click-to-walk, resizing and letterboxing.
- [x] Implement shader compositing for ambient darkness and local warm/cool light over room and character.
- [x] Add movable light, visible controls and documentation.
- [x] Formatting, compilation, four movement/mapping tests and strict Clippy checks pass.
- [x] Build local unsigned macOS app and repeatable app packaging helper.
- [x] Attempt runtime validation via command launch and native app launch.
- [x] User desktop check: app opens, room renders, movable light works, and character responds to torch while walking (October 18, 2026).
- [x] User desktop check: live resizing looks good (October 18, 2026).
- [ ] Explicit manual checks: L lighting-comparison toggle and clicks in letterbox bars are ignored.
- [x] User Windows check: project builds, launches, and renders; screenshot shows hard circular lighting cutoffs (October 18, 2026).
- [x] Apply shader precision fix: normalize light offsets before length and use high-precision coordinates/arithmetic.
- [x] User Windows visual recheck: patched torch and movable light gradients are smooth and match macOS (October 18, 2026).
- [x] User desktop check: resizing works on Windows and Linux (October 18, 2026).
- [ ] Complete remaining Windows control checks and Linux checks beyond resizing.

Scope: one room, unobstructed movement, procedural temporary art, verb panel, demonstration inventory and a key/gate interaction. No dialogue, collision/pathfinding, normal maps or dynamic occlusion shadows.

## Verb and inventory interface

- [x] Add a teal nine-verb panel and a two-row inventory with arrows and mouse-wheel scrolling.
- [x] Select verbs/items and preview action sentences over gate, torch, and inscription hotspots.
- [x] Walk to hotspots before acting; clicking empty room cancels the pending action.
- [x] Add a small brass-key/gate interaction and persistent open/closed state within the session.
- [ ] User visual/play check: panel readability at 1280×720, inventory scrolling, hover sentences, and resized input mapping.

Inventory starts with temporary demonstration items to exercise scrolling; collecting items and save/load are still future work.
- [x] Interface integration: formatting, six unit tests, strict Clippy, native build, and macOS app rebuild pass. User confirmed the corrected interface looks good on macOS (October 18, 2026); detailed interaction checks remain pending.
- [x] Fix screen-overlay Y orientation after user screenshot showed inverted interface at the top. Screen camera now compensates for Macroquad's screen/render-target Y convention; six tests, formatting, Clippy and app rebuild pass. User confirmed the orientation fix works and the interface looks good (October 18, 2026).

## Walk to interact

- [x] Give gate, torch and inscription fixed approach points on the floor.
- [x] Snapshot selected action and item; execute exactly once at the object's approach point.
- [x] Show the queued action while walking; new hotspot clicks replace it and empty-room clicks cancel it.
- [x] Cover delayed execution, replacement, cancellation, and already-at-object behavior with tests.
- [ ] User runtime check: approach all three objects, replace/cancel actions mid-walk, and use the key then open the gate.

- [x] Nine unit tests, formatting, strict Clippy and macOS app rebuild pass; user reviewed the walk-to-interact update and confirmed it looks good (October 18, 2026).
