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

**User desktop check passed on macOS (October 18, 2026).** The user opened the app and confirmed the room renders successfully, the movable light works, and the character visibly responds to the torch while walking past it. The user also confirmed live resizing looks good. The L comparison toggle and ignoring clicks in letterbox bars still need explicit manual checks; coordinate mapping is covered by automated tests.

Earlier agent launch attempts aborted during Miniquad's macOS initialization or timed out. The user reported having recently reinstalled macOS and needing to grant OS permissions, which may explain those failures; the cause was not independently established. Their successful desktop launch resolves the initial rendering blocker.

The user built and ran the project on Windows (October 18, 2026). Their screenshot revealed hard circular cutoffs around both lights. A shader fix now normalizes light offsets before calculating length and requests high precision; this avoids large squared pixel distances that can overflow reduced-precision arithmetic. The user confirmed the patched torch and movable light gradients are smooth and visually match macOS (October 18, 2026). The user also confirmed resizing works on Windows and Linux (October 18, 2026); other Linux behavior has not yet been explicitly verified. Use a native Rust toolchain and any platform development libraries required by Macroquad. The `.app` helper is macOS-only and currently builds for this Mac's architecture; it is not a signed or universal distribution.

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

1. Explicitly check clicks in letterbox bars and the lighting toggle; run the Windows portability check below.
2. Add a named hotspot and examine interaction.
3. Add a small inventory and one puzzle.
4. Connect rooms, then add conversation and save/load.
5. Build a coherent three-screen demo before expanding content.

## Build and run on Windows

Use native Windows PowerShell or Windows Terminal for this portability check.

1. Install Rust using https://rust-lang.org/tools/install. Accept the default stable MSVC toolchain and install the Visual Studio C++ prerequisites when prompted. If installing Build Tools manually, select **Desktop development with C++**, including the MSVC x64/x86 tools and a Windows SDK. Restart the terminal afterward.
2. Copy `Cargo.toml`, `Cargo.lock`, and the entire `src` folder into a folder such as `Documents\rust-point-click` on the PC. README.md and TASKS.md are useful to copy too. Do not transfer `.local`, `target`, or the macOS `.app`; Windows needs its own toolchain and build. No external artwork is needed.
3. In PowerShell, run:

```powershell
cd "$HOME\Documents\rust-point-click"
rustc --version
cargo --version
cargo test --locked
cargo run --locked
```

The first build downloads dependencies and needs an internet connection. `cargo run` builds a native Windows executable and opens the game.

For an optimized executable:

```powershell
cargo build --release --locked
.\target\release\rust-point-click.exe
```

Check walking/stopping, resizing into wide and tall shapes, ignoring clicks in black bars, warm torch lighting on the character, right-mouse movable light, L comparison, R reset, and Escape. Record build results separately from visual checks in TASKS.md. Windows build, launch, patched lighting, and resizing are user-confirmed. Linux resizing is also user-confirmed. Remaining Windows controls and Linux behavior beyond resizing still need explicit checks.

If Cargo is not recognized, reopen the terminal after installing Rust. If the build cannot find `link.exe`, install or repair the Visual Studio C++ tools and Windows SDK. For other failures, capture the full terminal error.

## Adventure interface

The bottom quarter now holds nine selectable verbs and a four-column, two-row
inventory inspired by classic verb interfaces. Click a verb, then a room hotspot.
Click an inventory item to select it for Use; click it again to deselect. Select
Look at before an item to build an examine sentence. Scroll the inventory using
the mouse wheel over the panel or the up/down buttons. Temporary starting items
exercise scrolling; there is no pickup system yet.

Hover over the archive gate, torch, or wall inscription to preview the action
near the pointer. Clicking walks the explorer to the object before responding;
clicking empty room cancels the pending interaction. Try Use + brass key + archive
gate, then Open + archive gate. The gate can also be closed. Other actions give
short feedback. Interface clicks never move the character or the cool light.

`src/interface.rs` owns panel selection, inventory scrolling, hit testing and
unlit UI drawing. `src/interaction.rs` owns hotspot detection and room responses.
The room render occupies the top three quarters; its input uses the matching
vertical transform. This first layout compresses the temporary room vertically
to retain all existing scenery and floor destinations. The outer canvas remains
1920×1080 with 16:9 letterboxing. Interface runtime checks are pending.

Walk-to-interact now uses fixed floor approach points for each object, rather than
where within the hotspot you clicked. A "Walking to" sentence shows the queued
action. New hotspot clicks replace it; empty-room clicks cancel it. Changing the
verb or inventory selection while walking affects the next action, while the
queued action keeps its original selection. Actions execute once upon arrival,
or immediately if already at the approach point. This uses ordinary structs in
`src/interaction.rs`; movement remains in `src/world.rs` with no pathfinding.
