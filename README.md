# hello_wagi: Example WAGI program

[WAGI (Web Assembly Gateway Interface)](https://github.com/deislabs/wagi) is an adaptation of CGI for WebAssembly WASI modules.

This repo contains a Rust implementation of a simple "Hello World" WAGI server.

## Building

You can test the output of this command using `cargo run`.

However, when building it for WAGI, you need to compile this as Web Assembly with WASI support:

```
$ cargo build --target wasm32-wasi --release
```

(We recommend using `--release` because it considerably reduces the size.)

If you use `make`, you can `make build` instead of typing in the cargo command above.

The resulting binary will be written to `target/wasm32-wasi/release/hello_wagi.wasm`.

## Using This With WAGI

WAGI is a [simple web server for Wasm](https://github.com/deislabs/wagi).

This module can be executed from within WAGI. See the `example-wagi.toml` file to see how.