#!/bin/bash
set -exo pipefail

echo "starting build for TARGET $TARGET"

export CRATE_NAME=secureircd

SUFFIX=""

echo "$TARGET" | grep -E '^x86_64-pc-windows-gnu$' >/dev/null && SUFFIX=".exe"

# build binary
cross build --target $TARGET --release

# to check how they are built
file "target/$TARGET/release/secureircd$SUFFIX"

# if this commit has a tag, upload artifact to release
strip "target/$TARGET/release/secureircd$SUFFIX" || true # if strip fails, it's fine
mkdir -p release
cp "target/$TARGET/release/secureircd$SUFFIX" "release/secureircd-$TARGET$SUFFIX"

echo 'build success!'
exit 0
