# Prototype cho ADR-001

Chứng minh: một interpreter viết bằng Rust, biên dịch sang WASM, chạy được trong
host JS với kích thước rất nhỏ.

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
node test.mjs
```

Kết quả đo trên macOS, Rust 1.97.1, Node 26.7:
build 3.76s · `wasm_probe.wasm` 16 KB · 7/7 test pass.
