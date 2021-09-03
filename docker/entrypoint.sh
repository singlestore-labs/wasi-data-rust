#!/bin/bash

set -e

# Check if running in GitHub vs locally
if [ -n "$GITHUB_ACTIONS" ]
then
  echo "** Running github action script **"
  cargo install cargo-wasi
  cargo wasi build --manifest-path component/Cargo.toml
  cargo run
  echo "** **"
fi