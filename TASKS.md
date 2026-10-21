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

Current scope: two connected rooms, a chamber chest obstacle, embedded Castle/Garden backgrounds, an animated Jenn character sprite, verb panel, inventory collection, and a complete garden-key/gate puzzle. No dialogue, save/load, general navigation framework, normal maps, or dynamic occlusion shadows.

## Verb and inventory interface

- [x] Add a teal nine-verb panel and a two-row inventory with arrows and mouse-wheel scrolling.
- [x] Select verbs/items and preview action sentences over gate, torch, and inscription hotspots.
- [x] Walk to hotspots before acting; clicking empty room cancels the pending action.
- [x] Add a small brass-key/gate interaction and persistent open/closed state within the session.
- [ ] User visual/play check: panel readability at 1280×720, inventory scrolling, hover sentences, and resized input mapping.

The earlier interface started with demonstration items. The garden milestone below adds collection and removes the starting key; save/load remains future work.
- [x] Interface integration: formatting, six unit tests, strict Clippy, native build, and macOS app rebuild pass. User confirmed the corrected interface looks good on macOS (October 18, 2026); detailed interaction checks remain pending.
- [x] Fix screen-overlay Y orientation after user screenshot showed inverted interface at the top. Screen camera now compensates for Macroquad's screen/render-target Y convention; six tests, formatting, Clippy and app rebuild pass. User confirmed the orientation fix works and the interface looks good (October 18, 2026).

## Walk to interact

- [x] Give gate, torch and inscription fixed approach points on the floor.
- [x] Snapshot selected action and item; execute exactly once at the object's approach point.
- [x] Show the queued action while walking; new hotspot clicks replace it and empty-room clicks cancel it.
- [x] Cover delayed execution, replacement, cancellation, and already-at-object behavior with tests.
- [ ] User runtime check: approach all three objects, replace/cancel actions mid-walk, and use the key then open the gate.

- [x] Nine unit tests, formatting, strict Clippy and macOS app rebuild pass; user reviewed the walk-to-interact update and confirmed it looks good (October 18, 2026).


## First obstacle: wooden chest

- [x] Draw a temporary wooden chest with brass fittings in the room lighting pass.
- [x] Route around its padded rectangular footprint using a small corner visibility graph.
- [x] Clamp blocked destinations to a safe edge; replace routes on new clicks.
- [x] Preserve movement speed through corners and exact stopping.
- [x] Draw explorer behind/in front of chest based on floor depth.
- [x] Add chest hotspot, approach point and examine/immovable responses.
- [x] Formatting, 14 unit tests, strict Clippy and native macOS app rebuild pass.
- [ ] User runtime check: walk both directions around chest, click behind/in front, redirect mid-walk, and examine chest; verify depth ordering and lighting.
- [ ] Recheck updated obstacle behavior on Windows and Linux.

- [x] User reviewed the chest obstacle update and confirmed it looks good on macOS (October 18, 2026). Detailed route/depth checks and other-platform checks remain pending.


## Garden and complete demo

- [x] Separate typed identities/session state, shared interactions, and room content/artwork.
- [x] Add sunny garden with animated fountain, stonework, greenery, and right passage.
- [x] Add chamber left passage and directional exit cursor/label independent of selected verb/item.
- [x] Walk to exits before switching; clear old routes/actions and enter safely inside the destination.
- [x] Parameterize single-obstacle routing for chamber chest and garden fountain; preserve depth order.
- [x] Push loose stone, reveal compartment, collect brass key once, persist state through room changes.
- [x] Keep nine other starting props; key/icon selection uses item identity, not inventory index.
- [x] Use key to unlock gate; Open gate completes demo with one-time dismissible message.
- [x] Preserve per-room lighting, resizing/input mapping, and movable-light controls.
- [x] 24 automated tests pass; formatting, strict Clippy, native build, and macOS app rebuild pass (October 18, 2026).
- [x] Refresh README launch instructions, controls, puzzle walkthrough, architecture, and portability checks.
- [x] macOS app launches outside the sandbox; screenshot confirms the new chamber passage and nine-prop starting inventory render (October 18, 2026).
- [ ] Complete macOS visual playthrough: garden/daylight/fountain animation, both arrow cursors, pickup, gate completion/dismissal, depth order and resized inputs. UI automation could not reliably drive the passage and reported that the user changed the app; full visual behavior is not claimed.
- [ ] Windows/Linux validation of the new rooms, arrows, puzzle, lighting, and obstacle behavior.
- [x] User reviewed the garden/demo update and confirmed it looks good on macOS (October 18, 2026). Detailed playthrough and other-platform checks remain pending.

## Jenn sprite integration

- [x] Replace temporary explorer artwork with the embedded `assets/jenn` PNG and Aseprite JSON Array export.
- [x] Use `idle_right` and `walk_right` tags and exported frame durations; ignore untagged frames.
- [x] Select walking from actual movement, mirror leftward movement, and retain horizontal facing while idle or moving vertically.
- [x] Keep sprite drawing at feet-based movement/interaction positions in the existing room-lighting and obstacle-depth stages.
- [x] Document re-export settings and include `assets` in source-copy build instructions.
- [x] 27 automated tests, formatting, strict Clippy, native build/macOS app rebuild, and whitespace checks pass (October 19, 2026).
- [x] Launch rebuilt macOS app and capture idle transparency/lighting, arrival idle at the gate approach, a rightward walking pose at a new position, a mirrored leftward walking pose, and stopped left-facing idle (October 19, 2026).
- [ ] Detailed macOS visual review: stride/foot sliding, animation timing, ground alignment, displayed size, garden lighting, obstacle depth ordering, and resized sprite presentation.
- [ ] Windows/Linux runtime validation of the sprite integration.

## Front-facing movement

- [x] Embed renamed side/front sheets and validate each sheet's own tags.
- [x] Use front walking for predominantly downward motion; retain front idle on arrival. Use side sprites for other movement until back artwork is available.
- [x] Preserve the user-selected scale of 2.0 and shared feet origin.
- [x] 28 tests, strict Clippy, formatting, native build/macOS app rebuild, and whitespace checks pass (October 19, 2026).
- [x] macOS runtime screenshots confirm front walking toward the camera and front idle after arrival (October 19, 2026).
- [ ] Windows/Linux runtime check of front movement; detailed stride and alignment review.

- [x] Double front-walk animation playback speed; keep movement speed, side playback, and idle timing unchanged.

## Painted background integration

- [x] Embed `assets/backgrounds/castle.png` and `assets/backgrounds/garden.png`, using nearest-neighbor filtering and stretching to the existing room canvas.
- [x] Restore the Castle torch and inscription sign with Macroquad drawing; retain the chest and existing interactions.
- [x] Use the painted Garden fountain as static scenery with no collision or character occlusion; remove the procedural fountain and droplets.
- [x] Retain the marked-stone/key puzzle using a compartment/key overlay and a cropped displaced stone.
- [x] Update current architecture and source-copy instructions for the background assets.
- [x] 29 automated tests, formatting, strict Clippy, native build/macOS app rebuild, and whitespace checks pass (October 20, 2026).
- [x] macOS screenshots confirm both backgrounds, restored torch/sign, passage transition, and marked stone reveal. User tested the update and confirmed it looks good (October 20, 2026).
- [ ] Detailed runtime checks: chest depth, walking over the fountain, key collection, gate completion, and resized input mapping; the user did not enumerate individual checks.
- [ ] Windows/Linux runtime validation of the new backgrounds and puzzle overlays.

The garden milestone above records the earlier procedural fountain implementation. The current background version deliberately allows walking over the painted fountain. Separating it into an animated transparent sprite with depth occlusion is future work.
