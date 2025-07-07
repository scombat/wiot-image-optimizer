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
    elif [[ "$OS" == "Windows_NT" ]] || [[ "$(uname -s)" == *"MINGW"* ]] || [[ "$(uname -s)" == *"MSYS"* ]]; then
      if ! command -v vcpkg >/dev/null 2>&1; then
        echo "vcpkg not found, installing..."
        git clone https://github.com/microsoft/vcpkg.git "$HOME/vcpkg"
        "$HOME/vcpkg/bootstrap-vcpkg.sh"
        export PATH="$HOME/vcpkg:$PATH"
      fi
      "$HOME/vcpkg/vcpkg" install dav1d:x64-windows pkgconf
      export VCPKG_ROOT="$HOME/vcpkg"
      export PKG_CONFIG_PATH="$HOME/vcpkg/installed/x64-windows/lib/pkgconfig"
      echo "VCPKG_ROOT set to $VCPKG_ROOT"
      echo "PKG_CONFIG_PATH set to $PKG_CONFIG_PATH"
      echo "SYSTEM_DEPS_DAV1D_SEARCH_NATIVE set to $SYSTEM_DEPS_DAV1D_SEARCH_NATIVE"
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

coverage *OPTS="":
    cargo tarpaulin --verbose {{OPTS}}

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
