#!/bin/sh
set -eu
cd "$(dirname "$0")"
if [ -x .local/cargo/bin/cargo ]; then
  export CARGO_HOME="$PWD/.local/cargo"
  export RUSTUP_HOME="$PWD/.local/rustup"
  export PATH="$CARGO_HOME/bin:$PATH"
fi
cargo run -- "$@"
