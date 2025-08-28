#!/usr/bin/env bash
set -e

TARGETS=(
    "x86_64-unknown-linux-gnu:hnt-linux-x86_64"
    "aarch64-unknown-linux-gnu:hnt-linux-arm64"
    # "x86_64-apple-darwin:hnt-darwin-x86_64"
    # "aarch64-apple-darwin:hnt-darwin-arm64"
    "x86_64-pc-windows-gnu:hnt-windows-x86_64.exe"
)

mkdir -p releases

for t in "${TARGETS[@]}"; do
    TARGET=${t%%:*}
    NAME=${t##*:}
    echo "Building $NAME ..."
    cargo build --release --target $TARGET

    BINARY_FILE="hnt"
    [[ $TARGET == *windows* ]] && BINARY_FILE="hnt.exe"

    cp target/$TARGET/release/$BINARY_FILE releases/$NAME
done
echo "✅ Succesfully built the releases for 5 binary"
