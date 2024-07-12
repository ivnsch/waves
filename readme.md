```sh
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_example \
  --out-dir target \
  --target web target/wasm32-unknown-unknown/debug/linear_alg.wasm
python -m http.server 8888
```
