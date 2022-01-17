# Rust Practice

Miscellaneous practice exercises in Rust.

## Usage

Since I'll inevitably forget once I step away from Rust for a bit, here are some basic instructions for building, running, testing, benchmarking, etc.
More info is available with `cargo -h` or `cargo [CMD] -h`.

### Check Compilation

```shell
cargo check
```

### Run Tests

```shell
cargo test
```

### Run Examples

```shell
cargo run --example [EXAMPLE-NAME]
```

### Run Benchmarks

This cannot be run with stable Rust. Install nightly rust with `rustup install nightly`.

```shell
cargo +nightly bench
```
