set shell := ["bash", "-c"]

default:
  just -l

install:
  rustup component add rustfmt
  cargo install cargo-tarpaulin

lint *OPTS="-- -D warnings":
  cargo clippy {{OPTS}}

check:
  cargo check

format *OPTS="--check":
  cargo fmt {{OPTS}}

test *OPTS="--verbose":
  cargo test {{OPTS}}

coverage:
  cargo tarpaulin --verbose

build:
  cargo build --release --all-targets --all-features