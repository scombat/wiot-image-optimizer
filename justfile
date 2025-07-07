set shell := ["bash", "-c"]

default:
  just -l

install:
    just setup-hooks
    rustup component add rustfmt
    cargo install cargo-tarpaulin
    just install-dav1d

install-dav1d:
    #!/bin/bash
    set -e
    if command -v brew >/dev/null 2>&1; then
      brew install dav1d
    elif command -v apt-get >/dev/null 2>&1; then
      sudo apt-get update
      sudo apt-get install -y libdav1d-dev pkg-config
    elif command -v dnf >/dev/null 2>&1; then
      sudo dnf install -y dav1d-devel pkgconf-pkg-config
    else
      echo "Please install dav1d and pkg-config manually."
      exit 1
    fi

lint *OPTS="-- -D warnings":
    cargo clippy --workspace --all-targets --all-features {{OPTS}}

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
    just format ""

    echo "🧹 [pre-commit] Running lint --fix..."
    just lint --fix --allow-dirty --allow-staged

    echo "✅ [pre-commit] Done."

# Usage: just review-pr github_user branch_name
review-pr username branch:
    git remote add {{username}} https://github.com/{{username}}/wiot-image-optimizer.git || true
    git fetch {{username}} {{branch}}
    git checkout -b review/{{username}}-{{branch}} {{username}}/{{branch}}
