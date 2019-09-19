#!/usr/bin/env bash

set -ev
pushd examples/program_examples
for d in */ ; do
    pushd "$d"
    cargo build
    if [ "$TRAVIS_RUST_VERSION" = "stable" ]; then
        cargo fmt --all -- --check
    fi
    popd
done
popd
