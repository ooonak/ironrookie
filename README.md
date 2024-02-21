# ironrookie
[![Rust](https://github.com/ooonak/ironrookie/actions/workflows/rust.yml/badge.svg)](https://github.com/ooonak/ironrookie/actions/workflows/rust.yml)

Play &amp; learn

## Project setup
Please use pre-commit hook for formatting.

```bash
$ git config --local core.hooksPath .githooks/
```

## Coverage
```bash
# https://doc.rust-lang.org/rustc/instrument-coverage.html
# https://blog.balthazar-rouberol.com/measuring-the-coverage-of-a-rust-program-in-github-actions

# Install grcov

$ cargo install rustfilt

$ cargo clean
$ RUSTFLAGS="-C instrument-coverage"
$ cargo build
$ LLVM_PROFILE_FILE="default_%m_%p.profraw"
$ cargo test --tests
$ grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
$ firefox ./target/debug/coverage/index.html
$ rm *.profraw
```

