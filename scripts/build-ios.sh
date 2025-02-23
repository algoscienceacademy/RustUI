#!/bin/bash
set -e

# Install iOS target if not installed
rustup target add aarch64-apple-ios

# Build for iOS
cargo build --target aarch64-apple-ios --example $1

# Create iOS app structure
mkdir -p ios/App
cp target/aarch64-apple-ios/debug/examples/$1 ios/App/

echo "iOS build completed"
