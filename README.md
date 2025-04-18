# Game map

## Building
If you just want to run this, you can with `cargo run`

If you want to build for the web and test it, theres a little bit more to it.

## Building for the web

### Dependencies
rustup target install wasm32-unknown-unknown
cargo install wasm-bindgen-cli

### Building
First build the wasm file:
`cargo build --release --target wasm32-unknown-unknown`

Then make it work for web and have a js file that launches the wasm:
```
wasm-bindgen --target web \
    --out-dir ./out/ \
    --out-name "nanite-factory-game-map" \
    ./target/wasm32-unknown-unknown/release/nanite-factory-game-map.wasm
```

Finally, generate the assets.json file used for testing(Only needs to be done when assets are changed):

`node ./generate-assets.js`
