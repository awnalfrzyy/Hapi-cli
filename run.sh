#!/usr/bin/env bash
set -euo pipefail

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}==> 1. Build debug${NC}"
cargo build

echo -e "${GREEN}==> 2. Run tests${NC}"
cargo test

echo -e "${GREEN}==> 3. Run linter (Clippy)${NC}"
cargo clippy -- -D warnings

echo -e "${GREEN}==> 4. Run program${NC}"
cargo run "$@"