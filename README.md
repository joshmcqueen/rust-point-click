# The Ember Archive

A small Rust/Macroquad point-and-click learning prototype. One procedural ruined chamber, an explorer, click-to-walk, ambient darkness, a warm flickering torch and a movable cool light. Art is temporary geometry; no external assets are needed.

## Launch on this Mac

From Terminal:

```sh
cd /Users/joshmcqueen/Documents/rust-point-click
./run.sh
```

Rust was absent, so an official stable Rust toolchain was installed in `.local/` inside this project. The launcher selects it automatically without changing your shell configuration. Keep `.local/` if using this launcher without a system Rust installation. Dependencies are locked in `Cargo.lock`.

A local unsigned development app is also built at `target/Ember Archive.app`; open it from Finder. Rebuild it after source changes with `./build-mac.sh`.

On another machine with Rust installed, run `cargo run` in the project. A standard Rust installation is available from https://rust-lang.org/install.html.

## Controls

- Left click: walk toward that point. Destinations are clamped to the rectangular floor.
- Hold right mouse: place the cool light beneath the pointer.
- L: toggle between lighting and full-bright colors for comparison.
- R: reset the cool light.
- Escape: quit.

The window starts at 1280×720 and can be resized. The room is drawn at **1920×1080**, with nearest-neighbor filtering and centered 16:9 letterboxing. Mouse coordinates use the inverse of that same transform; clicks in the bars are ignored. At non-integer display scales some geometric pixels will occupy unequal physical pixel widths.

## Small architecture

- `src/main.rs`: window, input, frame loop, render target, lighting shader and controls.
- `src/world.rs`: scene coordinates, viewport mapping, floor bounds and movement, independent of graphics. Four unit tests cover mapping, bars, movement timing and exact stopping.
- `src/scene.rs`: room, explorer and torch drawn with Macroquad primitives.

Movement advances by speed × elapsed seconds, normalizes the travel direction and snaps precisely to the target when the remaining distance is less than a step. New clicks replace the destination. There is no collision or pathfinding.

Room and character first share an unlit render texture. A fragment shader multiplies their colors by cool ambient light plus two radial lights with squared falloff. The torch flickers slightly. This is actual color compositing, rather than a transparent orange circle over the scene. The architecture includes painted shadow shapes; these are static art. Light passes through architecture: no occlusion, cast shadows, normal maps or physically based lighting. The HUD is drawn after lighting for readability.

The shader follows Macroquad's material API: https://docs.rs/macroquad/0.4.16/macroquad/material/index.html.

## Validation

On the available Apple Silicon Mac: `cargo fmt --check`, `cargo test` (4 passed), `cargo clippy -- -D warnings` and `cargo build` passed. Apple Command Line Tools are installed. Rust 1.99.1, Macroquad 0.4.16.

**User desktop check passed on macOS (October 18, 2026).** The user opened the app and confirmed the room renders successfully, the movable light works, and the character visibly responds to the torch while walking past it. Live resizing, letterboxing and the L comparison toggle still need an explicit manual check; coordinate mapping is covered by automated tests.

Earlier agent launch attempts aborted during Miniquad's macOS initialization or timed out. The user reported having recently reinstalled macOS and needing to grant OS permissions, which may explain those failures; the cause was not independently established. Their successful desktop launch resolves the initial rendering blocker.

Windows and Linux are intended targets through Macroquad, but neither was built or run here. Use a native Rust toolchain and any platform development libraries required by Macroquad. The `.app` helper is macOS-only and currently builds for this Mac's architecture; it is not a signed or universal distribution.

To repeat developer checks with the project-local toolchain:

```sh
export CARGO_HOME="$PWD/.local/cargo"
export RUSTUP_HOME="$PWD/.local/rustup"
export PATH="$CARGO_HOME/bin:$PATH"
cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo build
```

## Next milestones

1. Explicitly check live resizing, letterboxing and the lighting toggle; tune movement and lighting as needed.
2. Add a named hotspot and examine interaction.
3. Add a small inventory and one puzzle.
4. Connect rooms, then add conversation and save/load.
5. Build a coherent three-screen demo before expanding content.
