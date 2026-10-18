#!/bin/sh
set -eu
cd "$(dirname "$0")"
if [ -x .local/cargo/bin/cargo ]; then
  export CARGO_HOME="$PWD/.local/cargo"
  export RUSTUP_HOME="$PWD/.local/rustup"
  export PATH="$CARGO_HOME/bin:$PATH"
fi
cargo build
app='target/Ember Archive.app/Contents'
mkdir -p "$app/MacOS"
cp target/debug/rust-point-click "$app/MacOS/ember-archive"
cat > "$app/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>ember-archive</string>
<key>CFBundleIdentifier</key><string>local.rust-point-click.ember-archive</string>
<key>CFBundleName</key><string>Ember Archive</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
printf 'Built target/Ember Archive.app (local unsigned development app)\n'
