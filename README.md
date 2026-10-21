# The Ember Archive

A small Rust/Macroquad point-and-click adventure demo: a ruined chamber and a sunny garden, with painted Castle/Garden backgrounds, an animated Jenn sprite, walking around the chamber chest, room lighting, nine verbs, and a complete key puzzle. Background and character artwork are embedded in the executable.

## Launch

On this Mac:

```sh
cd /Users/joshmcqueen/Documents/rust-point-click
./run.sh
```

The launcher uses the project-local official Rust toolchain in `.local/` when present, without changing your shell configuration. Dependencies are locked in Cargo.lock. With a standard Rust installation on macOS, Windows, or Linux, use `cargo run --locked`.

Open `target/Ember Archive.app` from Finder for the local unsigned development app. Rebuild it with `./build-mac.sh`. This bundle targets the build Mac's architecture; it is not signed or universal.

## Controls and puzzle

- Left click empty floor to walk. Destinations stay on the floor; chamber routes avoid the chest. The garden fountain is part of the painted background, so the character can walk over it.
- Select a verb, then click an object. The explorer walks to its approach point before acting.
- Hover over the chamber's left passage or garden's right passage for a directional arrow and destination label. Click to walk there and change rooms, regardless of the selected verb/item.
- New object or exit clicks replace queued actions; clicking empty floor cancels them. Changing UI selection leaves the queued action unchanged.
- Click an inventory item to select/deselect it. Choose **Use** to use it on an object. Inventory selection preserves **Look at** and **Give** when those verbs are active. Scroll with the wheel over the panel or its arrow buttons.
- Hold right mouse over the room to move the cool light. **L** toggles full-bright comparison; **R** resets the light, not the puzzle.
- **Escape** quits, or dismisses the completion message when it is showing. Click or Enter also dismisses that message.

To finish: go left into the garden, **Push** the offset stone in the left garden wall, then **Pick up** the revealed brass key. Return through the right passage. Scroll the inventory down to find the key, select **Use** and the key, then click the archive gate. Finally select **Open** and click the gate. A completion message appears once; dismiss it to keep exploring.

The nine original inventory props remain; the brass key must be collected. Compartment, inventory, unlocked/open gate, and completion state persist across room visits for this session. Restarting resets the demo. Save/load and NPCs are future work.

## Small architecture

- `src/model.rs`: typed room/object/item/verb identities and explicit session state. Display names are independent of identity.
- `src/rooms.rs` and `src/rooms/`: chamber and garden artwork, hotspots, approach/entry/exit positions, geometry, and lighting settings.
- `src/interaction.rs`: queued object actions/exits and puzzle rules. It snapshots intent and changes state only on arrival.
- `src/world.rs` and `src/navigation.rs`: viewport mapping and frame-rate-independent movement using a small visibility graph around the chamber chest’s padded rectangular footprint; the garden has no blocking obstacle.
- `src/interface.rs`: selection, scrolling, hit testing, and UI drawing. Inventory belongs to game state; icons use item identity rather than slot number.
- `src/scene.rs`: Aseprite metadata, character animation playback, and shared explorer drawing. `src/main.rs`: input, frame loop, compositing, cursor, and completion overlay.

Ordinary enums make invalid names impossible; ordinary structs keep the state visible while learning Rust. Room transitions replace the actor's geometry and clear its route and pending action. The chamber chest is an obstacle with depth ordering. The painted garden fountain has no collision or depth occlusion in this version. No general adventure framework, physics, or advanced pathfinding is introduced.

The 1920×1080 design canvas uses nearest-neighbor filtering with centered 16:9 letterboxing. The window starts at 1280×720. The bottom quarter holds the UI; the room's 1920×1080 art is vertically compressed into the upper three quarters, with matching inverse mouse mapping. Clicks in bars are ignored. At non-integer scales, pixel widths can vary.

Room and explorer share a render texture. A shader multiplies their colors by room-specific ambient light, the chamber's warm torch, and a movable cool light. The garden uses warm daylight with no torch. The code-drawn chest orders the explorer by floor depth; the painted fountain stays behind the explorer. Painted shadows are static; light has no dynamic occlusion, cast shadows, or normal maps.

## Background artwork

`assets/backgrounds/castle.png` (1632×964) and `assets/backgrounds/garden.png` (1923×817) are embedded at compile time and drawn with nearest-neighbor filtering. Each image stretches to the existing 1920×1080 room canvas before the room is compressed above the interface; their original aspect ratios are not preserved. Mouse mapping and the 16:9 window composition remain the same.

The Castle image supplies the walls, archive gate, and left passage. Macroquad draws the torch, inscription sign, and chest over it, retaining their interactions and lighting. The Garden image supplies the fountain, greenery, stonework, and right passage. Its marked stone still supports the key puzzle: pushing it reveals a code-drawn compartment and key, and a crop of the background supplies the displaced stone. The fountain is static background art; walking over it is intentional for this first version. A separate animated fountain sprite with character occlusion is future work.

## Character artwork

Jenn's PNG sprite sheet and Aseprite JSON Array export live in `assets/jenn`. Both files are embedded at compile time; the built game needs no loose artwork files. The `idle_right` and `walk_right` tags select the standing and walking clips, and exported frame durations control playback independently of movement speed. Untagged frames are ignored. Actual movement chooses walking versus standing and horizontal facing; left-facing movement mirrors the right-facing artwork. Downward movement uses `walk_front` from the front sheet when the vertical distance moved exceeds the horizontal distance moved; stopping keeps `idle_front`. Front walking plays at 2× the exported timing (100 ms frames display for 50 ms); side walking and idle retain their exported timing. Other movement uses the side sheet, mirroring leftward movement. Upward movement keeps using side artwork until a back-facing sheet exists.

To replace or revise the artwork, export the PNG and JSON Array together, retaining each sheet's tag names (`idle_right`/`walk_right` for side, `idle_front`/`walk_front` for front), forward playback, and untrimmed 256×256 frame canvases. Keep the character aligned to the same ground line across frames. The initial drawing uses a shared feet anchor at canvas position (128, 250) and a uniform scale of 2.0 before the room's vertical compression. These drawing settings can be tuned after visual review without changing movement or interaction positions. Rebuild after changing either export. There are no new controls.

## Developer checks

```sh
export CARGO_HOME="$PWD/.local/cargo"
export RUSTUP_HOME="$PWD/.local/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
cargo fmt --check
cargo test --locked
cargo clippy --locked -- -D warnings
./build-mac.sh
```

On October 18, 2026, the two-room implementation passed 24 unit tests, formatting, strict Clippy, and the native macOS build/app rebuild. Tests cover puzzle prerequisites and one-time completion, collection without duplication, room persistence, queued actions/exits, room geometry, obstacle routing, movement timing, inventory selection, and resized input mapping. See TASKS.md for visual runtime results and remaining checks.

Earlier versions were user-confirmed to render and resize on macOS and Windows, with Windows light gradients verified after a shader precision fix. Linux resizing was user-confirmed. These earlier checks do not validate the new garden or puzzle on those platforms.

On October 19, 2026, the Jenn integration passed 27 unit tests, formatting, strict Clippy, the native macOS build/app rebuild, and whitespace checks. The rebuilt app was launched on macOS; screenshots confirm idle transparency/lighting, arrival idle at the gate approach, rightward walking, mirrored leftward walking, and stopped left-facing idle. Detailed stride/foot alignment, garden lighting, obstacle depth, resizing, and Windows/Linux sprite runtime checks remain pending.

## Windows and Linux checks

Install Rust from https://rust-lang.org/tools/install. Windows uses the stable MSVC toolchain and Visual Studio C++ build prerequisites (Desktop development with C++, MSVC tools, and Windows SDK). Linux needs the platform libraries required by Macroquad.

Copy Cargo.toml, Cargo.lock, the complete `src` directory including `src/rooms`, and the complete `assets` directory including `assets/jenn` and `assets/backgrounds`. Do not copy `.local`, `target`, or the macOS app. Then run:

```sh
cargo test --locked
cargo run --locked
```

For a Windows release build, use `cargo build --release --locked`, then run `target\release\rust-point-click.exe`.

Check both passages and arrows, the complete puzzle, chest routes and depth ordering, walking over the painted fountain, resizing and letterbox clicks, inventory scrolling, warm torch/daylight rendering, right-mouse light, L, R, completion dismissal, and Escape. Also check Jenn's standing/walking transitions, left/right mirroring, ground alignment, and lighting in both rooms. Record compilation separately from actual visual checks. The new two-room demo and sprite integration still need Windows/Linux runtime validation.

Front-facing movement update (October 19, 2026): 28 tests, formatting, strict Clippy, native build/app rebuild, and whitespace checks pass. macOS runtime screenshots confirm front walking and front idle on arrival. Windows/Linux runtime validation remains pending.

Background replacement (October 20, 2026): 29 tests, formatting, strict Clippy, native build/macOS app rebuild, and whitespace checks pass. macOS screenshots confirm both backgrounds, restored torch/sign, passage transition, and marked stone reveal. The user tested the update and confirmed it looks good. Detailed puzzle/depth/resizing checks and Windows/Linux runtime validation remain pending.
