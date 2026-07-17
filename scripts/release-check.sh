#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
npm pack --dry-run --prefix packaging/npm >/dev/null
python -m build packaging/pip
