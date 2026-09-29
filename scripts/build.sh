#!/usr/bin/env sh
# Builds the codegraph binary (release) and copies it into plugin/bin/.
# Usage: sh scripts/build.sh   (Linux, macOS, Git Bash on Windows)
set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
if ! command -v cargo >/dev/null 2>&1 && [ -x "$HOME/.cargo/bin/cargo" ]; then
    PATH="$HOME/.cargo/bin:$PATH"
fi
command -v cargo >/dev/null 2>&1 || { echo "cargo not found: install Rust with rustup (https://rustup.rs)" >&2; exit 1; }

cd "$root"
cargo build --release -p codegraph

# Windows (Git Bash) builds codegraph.exe; hooks and .mcp.json reference
# bin/codegraph and Windows resolves the .exe suffix (README, "Windows").
name=codegraph
[ -f target/release/codegraph.exe ] && name=codegraph.exe

mkdir -p plugin/bin
target="plugin/bin/$name"
if ! cp -f "target/release/$name" "$target" 2>/dev/null; then
    # A running `codegraph mcp` locks the exe on Windows: move it aside first.
    rm -f "$target.old" 2>/dev/null || true
    mv -f "$target" "$target.old"
    cp -f "target/release/$name" "$target"
    echo "warning: $target was in use; the old binary is kept as $target.old" >&2
fi
chmod +x "$target" 2>/dev/null || true

"$target" --version
echo "plugin binary: $root/$target"
