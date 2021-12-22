#!/usr/bin/env bash

set -euo pipefail

cargo build --release
./target/release/game_of_pawns $@