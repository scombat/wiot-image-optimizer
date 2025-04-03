set shell := ["bash", "-c"]

default:
  just -l

install:
    just setup-hooks
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

setup-hooks:
    chmod +x .git-hooks/*
    git config core.hooksPath .git-hooks

pre-push:
    #!/bin/bash
    echo "🔎 [pre-push] Checking formatting..."
    just format --check

    echo "🧠 [pre-push] Running clippy..."
    just lint -- -D warnings

    echo "🧪 [pre-push] Running tests..."
    just test --all

    echo "✅ [pre-push] All checks passed."

pre-commit:
    #!/bin/bash
    echo "🔧 [pre-commit] Running format..."
    just format

    echo "🧹 [pre-commit] Running lint --fix..."
    just lint --fix --allow-dirty --allow-staged

    echo "✅ [pre-commit] Done."
