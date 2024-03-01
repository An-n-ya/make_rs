#!/usr/bin/zsh
cargo rustc -- -Awarnings
pushd
cd /home/annya/playground/make_rs/tests/basic
../../target/debug/make_rs clean
popd