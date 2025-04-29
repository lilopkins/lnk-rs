#!/bin/bash

##### Preparation:
#
# rustup install nightly
# cargo install cargo-fuzz

RUST_BACKTRACE=1 cargo +nightly fuzz run shell_link_header
