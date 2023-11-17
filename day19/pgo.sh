#!/usr/bin/env bash


cargo build --profile release-lto
cargo build --profile release
echo LTO baseline
cargo run --profile release-lto -- --benchmark
echo no LTO baseline
cargo run --profile release -- --benchmark

rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
    cargo run --profile release-lto-pgo -- --benchmark

/home/frankster/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
    cargo build --profile release-lto-pgo
echo PGO run
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
    cargo run --profile release-lto-pgo -- --benchmark
