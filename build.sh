#!/bin/bash
cargo build --release
cp target/release/libturn_based_fighting_game_lib.so ../tbfg/lib/tbfg.so
