#!/bin/sh

set -ex

# Nightly is needed for wasm threading
rustup override set nightly
rustup target add wasm32-unknown-unknown
rustup component add rust-src

cargo install wasm-bindgen-cli --vers=0.2.74

# A couple of steps are necessary to get this build working which makes it slightly
# nonstandard compared to most other builds.
#
# * First, the Rust standard library needs to be recompiled with atomics
#   enabled. to do that we use Cargo's unstable `-Zbuild-std` feature.
#
# * Next we need to compile everything with the `atomics` and `bulk-memory`
#   features enabled, ensuring that LLVM will generate atomic instructions,
#   shared memory, passive segments, etc.

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  cargo build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

# Note the usage of `--target no-modules` here which is required for passing
# the memory import to each wasm module.
wasm-bindgen target/wasm32-unknown-unknown/release/particle_life.wasm \
  --out-dir sab \
  --target no-modules

# Compile again without atomics for browsers which don't support them.
cargo build --target wasm32-unknown-unknown --release

wasm-bindgen target/wasm32-unknown-unknown/release/particle_life.wasm \
  --out-dir no-sab \
  --target no-modules

# Don't deploy the entire target folder
cargo clean

wasm-opt -O4 -g sab/particle_life_bg.wasm -o sab/particle_life_bg.wasm
wasm-opt -O4 -g no-sab/particle_life_bg.wasm -o no-sab/particle_life_bg.wasm