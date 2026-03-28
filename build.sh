#!/bin/sh

cargo build --release

sudo cp target/release/togglecomp /usr/local/bin
