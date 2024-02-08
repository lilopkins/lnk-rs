#!/bin/bash

RUST_BACKTRACE=1 cargo +nightly fuzz run shell_link_header
