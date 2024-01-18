# this helps trunk not override dependencies that are used when running the game locally
# please run this in the bevy directory
CARGO_TARGET_DIR=../target/target2 trunk serve --features wasm