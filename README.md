# The Ember Archive

A small Rust/Macroquad point-and-click adventure demo: a ruined chamber and a sunny garden, with procedural pixel-style art, walking around obstacles, room lighting, nine verbs, and a complete key puzzle. No external artwork is required.

## Launch

On this Mac:

```sh
cd /Users/joshmcqueen/Documents/rust-point-click
./run.sh
```

The launcher uses the project-local official Rust toolchain in `.local/` when present, without changing your shell configuration. Dependencies are locked in Cargo.lock. With a standard Rust installation on macOS, Windows, or Linux, use `cargo run --locked`.

Open `target/Ember Archive.app` from Finder for the local unsigned development app. Rebuild it with `./build-mac.sh`. This bundle targets the build Mac's architecture; it is not signed or universal.

## Controls and puzzle

- Left click empty floor to walk. Destinations stay on the floor and routes avoid the room's obstacle.
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
- `src/world.rs` and `src/navigation.rs`: viewport mapping and frame-rate-independent movement using a small visibility graph around one padded rectangular obstacle per room.
- `src/interface.rs`: selection, scrolling, hit testing, and UI drawing. Inventory belongs to game state; icons use item identity rather than slot number.
- `src/scene.rs`: shared explorer drawing. `src/main.rs`: input, frame loop, compositing, cursor, and completion overlay.

Ordinary enums make invalid names impossible; ordinary structs keep the state visible while learning Rust. Room transitions replace the actor's geometry and clear its route and pending action. Each room has one obstacle: the chamber chest or garden fountain. No general adventure framework, physics, or advanced pathfinding is introduced.

The 1920×1080 design canvas uses nearest-neighbor filtering with centered 16:9 letterboxing. The window starts at 1280×720. The bottom quarter holds the UI; the room's 1920×1080 art is vertically compressed into the upper three quarters, with matching inverse mouse mapping. Clicks in bars are ignored. At non-integer scales, pixel widths can vary.

Room and explorer share a render texture. A shader multiplies their colors by room-specific ambient light, the chamber's warm torch, and a movable cool light. The garden uses warm daylight with no torch. Animated fountain droplets use quantized positions. Obstacle drawing orders the explorer by floor depth. Painted shadows are static; light has no dynamic occlusion, cast shadows, or normal maps.

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

## Windows and Linux checks

Install Rust from https://rust-lang.org/tools/install. Windows uses the stable MSVC toolchain and Visual Studio C++ build prerequisites (Desktop development with C++, MSVC tools, and Windows SDK). Linux needs the platform libraries required by Macroquad.

Copy Cargo.toml, Cargo.lock, and the complete `src` directory, including `src/rooms`. Do not copy `.local`, `target`, or the macOS app. Then run:

```sh
cargo test --locked
cargo run --locked
```

For a Windows release build, use `cargo build --release --locked`, then run `target\release\rust-point-click.exe`.

Check both passages and arrows, the complete puzzle, fountain/chest routes and depth ordering, resizing and letterbox clicks, inventory scrolling, warm torch/daylight rendering, right-mouse light, L, R, completion dismissal, and Escape. Record compilation separately from actual visual checks. The new two-room demo still needs Windows/Linux runtime validation.
