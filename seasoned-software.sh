#!/bin/bash

set -euo pipefail

# Print environment for debuggin.
# env

# Cargo-fuzz needs nightly rust, so switch this project to nightly
rustup override set nightly

# Build and register all tests known to Cargo-fuzz.
for t in $(cargo fuzz list|sed 's@\x1b[^m]*m@@g'); do
	echo "Building test: $t"
	cargo fuzz run $t --release -- -help=1
	exe="$(find fuzz/target -iname $t -executable)"

	echo "Registering: $exe"
	upload-binary "$exe"
done
