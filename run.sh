#!/usr/bin/bash
RUST_BACKTRACE=1 cargo run --bin d$1 $(dirname $0)/inputs/d$1
