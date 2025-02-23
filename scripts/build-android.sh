#!/bin/bash
set -e

# Install Android target if not installed
rustup target add aarch64-linux-android

# Build for Android
cargo build --target aarch64-linux-android --example $1

# Create Android app structure
mkdir -p android/app
cp target/aarch64-linux-android/debug/examples/$1 android/app/

echo "Android build completed"
