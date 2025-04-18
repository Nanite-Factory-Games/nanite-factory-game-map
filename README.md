# Game map

## Building
If you just want to run this, you can with `cargo run`

If you want to build for the web and test it, theres a little bit more to it.

## Building for the web

### Dependencies
rustup target install wasm32-unknown-unknown
cargo install wasm-pack

Make sure old versions of wasm-bindgen and wasm-opt are removed. If you're having trouble, try:
```
sudo apt remove --purge binaryen
sudo apt autoremove
```

For the optimizations in .cargo/config.toml it may be necessary to add the rust source and use nightly, via
`rustup component add rust-src --toolchain nightly`
IF THIS IS GIVING YOU PROBLEMS you can disable this portion by renaming .cargo's config.toml to _config.toml (please don't delete I worked hard)
(Note, config.toml is NOT Cargo.toml)

### Building
wasm-pack build --target web --release --verbose --out-dir ./out/

Finally, generate the assets.json file used for testing(Only needs to be done when assets are changed):

`node ./generate-assets.js`
