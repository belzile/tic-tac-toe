Tic Tac Toe Game coded in Rust with Bevy.

## Running Locally

### Native

```sh
cargo run
```

## Build for the Web

### Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/tic-tac-toe.wasm
npx serve .
```
