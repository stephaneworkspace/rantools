#!/bin/sh

# Build rust
# cargo +ios-arm64-1.60.0 build --target aarch64-apple-ios --release --lib
rm -rf lib
mkdir -p lib/generic
mkdir -p lib/generic_debug
mkdir -p lib/ios
mkdir -p lib/ios_debug
mkdir -p lib/sim
mkdir -p lib/sim_debug

cargo update
cargo test
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
cargo build --target aarch64-apple-ios --release --lib
cargo build --target x86_64-apple-ios --release --lib
cp ./target/aarch64-apple-ios/release/librantools.a ./lib/ios/librantools.a
cp ./target/x86_64-apple-ios/release/librantools.a ./lib/sim/librantools.a
lipo -create ./lib/ios/librantools.a ./lib/sim/librantools.a -output ./lib/generic/librantools.a
#lipo -info ./lib/generic/librantools.a
cp ./target/aarch64-apple-ios/debug/librantools.a ./lib/ios_debug/librantools.a
cp ./target/x86_64-apple-ios/debug/librantools.a ./lib/sim_debug/librantools.a
lipo -create ./lib/ios_debug/librantools.a ./lib/sim_debug/librantools.a -output ./lib/generic_debug/librantools.a
#lipo -info ./lib/generic_debug/librantools.a

