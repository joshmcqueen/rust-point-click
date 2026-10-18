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
- [ ] Recheck patched lighting on Windows against the smooth macOS gradient; complete remaining Windows controls/resizing checks. Linux remains untested.

Scope: one room, unobstructed movement, procedural temporary art. No inventory, dialogue, puzzle, collision/pathfinding, normal maps or dynamic occlusion shadows.
