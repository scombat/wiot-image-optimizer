# Contributing to WIOT

First of all, thank you for your interest in contributing to **WIOT**!

This project aims to provide a robust and extensible image processing pipeline written in idiomatic Rust. Whether you're fixing a typo, improving documentation, refactoring code, or implementing a new encoder or feature — your contribution is welcome.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How to Contribute](#how-to-contribute)
3. [Environment Setup](#environment-setup)
4. [Rust Best Practices](#rust-best-practices)
5. [Commit Messages](#commit-messages)
6. [Testing and Coverage](#testing-and-coverage)
7. [Pull Request Process](#pull-request-process)
8. [Style Guide](#style-guide)
9. [Common Pitfalls](#common-pitfalls)
10. [Feature Flags and Extensibility](#feature-flags-and-extensibility)

---

## Code of Conduct

Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md). Be kind, respectful, and constructive. We foster an inclusive and welcoming environment.

---

## How to Contribute

- Open an issue first for new features or large changes.
- Fork the repository and create a new branch from `develop`.
- Follow the [Rust Best Practices](#rust-best-practices) and [Style Guide](#style-guide).
- Write tests when applicable.
- Submit a focused pull request targeting the `develop` branch.

---

## Environment Setup

### Required Tools

- [Rust toolchain (stable)](https://rustup.rs/)
- [`just`](https://github.com/casey/just) — command runner for dev tasks
- `rustfmt` — for code formatting
- [`cargo-tarpaulin`](https://crates.io/crates/cargo-tarpaulin) — for code coverage
- [`dav1d`](https://code.videolan.org/videolan/dav1d) — system dependency (AV1 decoding)

> A `just install` recipe is provided to install all dependencies and configure Git hooks automatically.

### Getting Started

```bash
git clone https://github.com/your-org/wiot.git
cd wiot
just install
just check
````

---

## Rust Best Practices

WIOT uses idiomatic, modern Rust. Contributors should:

✅ Prefer `Result<T, E>` over panics
✅ Use `?` to propagate errors cleanly
✅ Derive `Debug`, `Clone`, `PartialEq`, etc., when appropriate
✅ Document all public items using `///` doc-comments
✅ Keep modules small and focused
✅ Separate concerns into dedicated files and structs
✅ Favor composition over inheritance-like trait hierarchies

Avoid:

❌ `unwrap()` or `expect()` outside of tests or bootstrapping code
❌ Premature abstraction with complex traits
❌ Mixing unrelated logic in a single module
❌ Overengineering generic interfaces

---

## Commit Messages

Use clear, conventional commit messages. Avoid backticks. Follow this structure:

```
type(scope): short summary

Optional longer body explaining the change.
Use imperative mood ("add", not "adds").
Reference issues/PRs where relevant.
```

Examples:

* `feat(core): add AVIF encoder`
* `fix(cli): handle missing options error gracefully`
* `refactor: isolate quality parsing logic`

---

## Testing and Coverage

* Run `just test` before submitting any changes.
* Cover both success paths and edge cases.
* Avoid redundant or trivial tests.
* Run coverage analysis with:

```bash
just coverage
```

This runs `cargo tarpaulin` and generates a report in `target/tarpaulin`.

---

## Pull Request Process

* Target **`develop`** (never `main`).
* Keep PRs focused and scoped.
* Provide context in the description (`Fixes #42`, screenshots, benchmarks).
* Draft PRs are welcome.
* CI must pass before merging.

Maintainers may:

* Request code cleanup or test coverage
* Ask for architectural clarification
* Refactor code for alignment with project structure

---

## Style Guide

* Use `snake_case` for variables and functions.
* Use `PascalCase` for types and enums.
* Group imports: standard → external → internal.
* Document all public types, functions, and modules.
* Prefer `mod.rs` only when necessary; prefer flat `mod_name.rs` files.

---

## Common Pitfalls

* Forgetting to update public documentation
* Using `unwrap()` in production code
* Adding unused imports or dead code
* Skipping test coverage for new features
* Not running `rustfmt` or linting before PR

---

## Feature Flags and Extensibility

WIOT is modular and designed for extensibility.

When adding a new codec or format:

1. Implement the `ImageEncoder` trait (see existing formats).
2. Gate optional features with `#[cfg(feature = "...")]`.
3. Register the codec in `CodecResolver`.
4. Add relevant documentation and tests.

Use features sparingly and clearly document their purpose in `Cargo.toml`.

---

## Thank You

Thanks again for contributing to WIOT! Your efforts help build a fast, clean, and extensible image processing toolkit in Rust. ❤️

```

---

Souhaites-tu que je le pousse dans un fichier prêt à être commité ou intégré dans ton repo actuel ?
```
