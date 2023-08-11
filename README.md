# Flipper Zero My Card
Demo application for Flipper Zero

# Usage

## Initial setup

1. Install [`rustup`](https://rust-lang.github.io/rustup/) by following the instructions on [`rustup.rs`](https://rustup.rs/).
1. Install the nightly build tool-chain to support the[`different-binary-name`](https://doc.rust-lang.org/cargo/reference/unstable.html#different-binary-name) feature:
    ```
    rustup toolchain install nightly
    ```
1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate):
    ```
    cargo install cargo-generate
    ```
1. Use `rustup` to install the `thumbv7em-none-eabihf` target to the nightly build:
    ```
    rustup target add --toolchain nightly thumbv7em-none-eabihf
    ```

## Build with `cargo build`

```
cargo build
```

## Copy the binary to your Flipper Zero

The resulting `.fap` binary can be found in [`target/thumbv7em-none-eabihf/debug`](target/thumbv7em-none-eabihf/debug).
