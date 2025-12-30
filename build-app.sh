#!/bin/bash

set -euo pipefail

BUILD_MODE="debug"
BUILD_FLAG=""

while [[ $# -gt 0 ]]; do
	case $1 in
	-r | --release)
		BUILD_MODE="release"
		BUILD_FLAG="--release"
		shift
		;;
	-d | --debug)
		BUILD_MODE="debug"
		BUILD_FLAG=""
		shift
		;;
	-h | --help)
		echo "Usage: $0 [OPTIONS]"
		echo "  -d, --debug     Build in debug mode (default)"
		echo "  -r, --release   Build in release mode"
		exit 0
		;;
	*)
		echo "Error: Unknown option $1"
		exit 1
		;;
	esac
done

echo "Build type is ${BUILD_MODE}"

echo ""
echo "Running clippy ..."
cargo clippy ${BUILD_FLAG} --all-targets --all-features -- -D warnings

echo ""
echo "Running tests ..."
cargo test ${BUILD_FLAG} --all-features

echo ""
echo "Building ..."
cargo build ${BUILD_FLAG} --jobs $(nproc)
