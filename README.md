

rustup target install wasm32-unknown-unknown
cargo install wasm-bindgen-cli

cargo install wasm-server-runner

cargo run --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --target web \
    --out-dir ./out/ \
    --out-name "nanite-factory-game-map" \
    ./target/wasm32-unknown-unknown/release/nanite-factory-game-map.wasm