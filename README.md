# WIOT Image Optimizer
![WIP](https://img.shields.io/badge/status-WIP-yellow?style=flat-square&logo=github)
![License](https://img.shields.io/badge/license-GPLv3-blue?style=flat-square)

**WIOT** (Web Image Optimizer Toolkit) is an open, modular, and extensible image optimization tool focused on web performance.
It is designed to run as a CLI, on servers, or inside cloud functions.

> 🚧 **This project is currently under active development.**
> Expect things to break, evolve, and improve rapidly. Feedback and contributions are welcome!

---

## ✨ Vision

WIOT aims to become a universal toolkit to:

- Optimize and convert images for web delivery (WebP, AVIF, etc.)
- Process images in real-time or in batch
- Support multiple entrypoints: CLI, HTTP, AWS Lambda, GCP, etc.
- Handle multiple sources/destinations: local files, HTTP, S3, etc.
- Offer a clear, composable API for transformations via `ProcessingOptions`

---

## ✅ Current Status

The CLI currently supports:

- ✅ Basic pipeline execution: read → process → write
- ✅ Image loaded into stateful pipeline
- ✅ Modular processing steps (resize, encoding, etc.)
- ✅ Automatic adapter resolution for local paths
- 🧪 Early support for resize options (via ProcessingOptions)

---

## 🧭 Feature Progress

| Area          | Feature                         | Status        |
| ------------- | ------------------------------- | ------------- |
| 🧰 Core        | Image pipeline                 | ✅ Done        |
|               | Load image                      | ✅ Done        |
|               | Process Image                   | ✅ Done        |
|               | Save image                      | ✅ Done        |
|               | Output Format detection         | ✅ Done        |
|               | Automatic format selection      | 🧪 Planned     |
|               | Resize (width & height)         | ✅ Done     |
|               | Resize (DPI)                    | ✅ Done     |
|               | Resize Aspect Ratio Strategy    | 🧪 Planned     |
|               | Resize Cover Strategy           | 🧪 Planned     |
|               | Resize Contain Strategy         | 🧪 Planned     |
|               | Resize Fill Strategy            | 🧪 Planned     |
|               | Quality                         | 🧪 Planned     |
|               | Automatic Quality selection     | 🧪 Planned     |
|               | Crop                            | 🧪 Planned     |
|               | Flip                            | 🧪 Planned     |
|               | Flop                            | 🧪 Planned     |
|               | Blur                            | 🧪 Planned     |
|               | Rotate                          | 🧪 Planned     |
|               | Base64 Encode                   | 💭 Idea        |
|               | Gravity centering               | 💭 Idea        |
|               | IA powered object detection     | 💭 Idea        |
|               | IA powered object focus         | 💭 Idea        |
| 🧩 Adapters    | Local (read/write)              | ✅ Done       |
|                | HTTP (read)                     | 🧪 Planned    |
|                | S3 (read/write)                 | 🧪 Planned    |
|               | (s)FTP (read/write)             | 💭 Idea        |
| 🌐 Entrypoints | CLI                             | 🔧 In progress |
|               | HTTP API                        | 🧪 Planned     |
|               | AWS Lambda                      | 🧪 Planned     |
|               | GCP Function                    | 💭 Idea     |
|               | Azure Function                  | 💭 Idea     |
|               | S3 (read/write)                 | 💭 Idea     |
|               | GUI                             | 💭 Idea     |
|               | CMS Plugins                     | 💭 Idea     |
| 🧪 Testing     | Adapter unit tests              | ✅ Done        |
|               | Pipeline integration tests      | 🧪 Planned     |
|               | E2E tests                       | 🧪 Planned     |
| 🧠 Tooling/UX/DX  | auto-resolver                  | ✅ Done        |
|               | Presets processing              | 🧪 Planned |
|               | Error handling/logs             | 🧪 Planned |
|               | Contributing guide             | 🧪 Planned |
|               | Docker image             | 💭 Idea |
|               | Golden image             | 💭 Idea |
|               | AWS Lambda Zip           | 💭 Idea |
|               | One-click cloud deploy   | 💭 Idea |
|               | srcset / responsive variants    | 💭 Idea        |


## 🚀 Getting Started

```bash
cargo run -p cli -- --input ./image.jpg --output ./output.jpg
```

Supported input/output formats:
- ✅ Local file paths (`./image.jpg`, `file://...`)
- 🔧 HTTP URLs (`https://...`)
- 🔧 S3 paths (`s3://bucket/key.jpg`)

---

## 📦 Installation (coming soon)

- Binary releases for major platforms
- Docker container image
- Cloud function deployment ZIPs

---

## 🧱 Architecture

```
wiot-image-optimizer/
├── core/                  # Business logic: image pipeline, processing options
│   └── src/
│       ├── lib.rs         # Entry point for the pipeline
│       ├── models/        # Option models (resize, format, quality, etc.)
│       └── services/      # Logic to apply those options (resize.rs, encode.rs, etc.)
│
├── adapters/              # FileSource & FileDestination implementations
│   ├── local.rs           # Local filesystem adapter
│   └── (http|s3|...)      # Future adapters
│
├── entrypoints/           # How users interact with WIOT
│   └── cli/               # CLI entrypoint (args parsing, triggering pipeline)
│   └── (http/aws/gcp/...)# Future interfaces
│
├── tests/                 # Integration and E2E tests
│
└── scripts/               # Dev tooling (Justfile, Dockerfile, etc.)
```

---

## 🤝 Contributing

We’re building the foundation — your help is welcome!
If you're interested in adapters, pipeline logic, format support, or CLI design, check out the issues and the [contribution guide](link-to-contribution-guidelines).

---

### 🛠 Project Setup

After cloning the repository, run:

```bash
just install
```

This will:

- Install required tools (`rustfmt`, `cargo-tarpaulin`, etc.)
- Set up Git hooks to enforce formatting, linting, and testing before commit/push
- Prepare your local dev environment

> 💡 If you don’t have [`just`](https://github.com/casey/just) installed yet, do:
> `cargo install just`

---

## 📄 License

Licensed under **GNU GPL v3.0**
See the [LICENSE](LICENSE) file for full details.

## ❤️ Sponsorship & Ethics

WIOT is an open project driven by passion for performance and web tooling.
If you'd like to support its development, for the moment you can become a contributor.
The sponsorship model is not yet implemented.
I’m working on a **shared sponsorship model** to ensure that all contributors are rewarded for their work.
I believe in a **fair and transparent** approach to sponsorship, where all contributors are recognized for their efforts.
I want to ensure that the project is sustainable and that contributors are rewarded for their work.
Your contributions help make this project sustainable — but not just for me.

### 🤝 Shared Sponsorship Model

So, if you want to support WIOT, you can do so by supporting the projects that make it possible.
When the sponsorship model is implemented, all sponsorship income will be **fairly redistributed** to the open source projects WIOT relies on, including (but not limited to):

- [`image`](https://github.com/image-rs/image)
- [`clap`](https://github.com/clap-rs/clap) [♥️ Sponsor](https://opencollective.com/clap)
- [`tokio`](https://github.com/tokio-rs/tokio) [♥️ Sponsor](https://github.com/sponsors/tokio-rs)
- [`anyhow`](https://github.com/dtolnay/anyhow) [♥️ Sponsor](https://github.com/sponsors/dtolnay)
- [`Rust Foundation`](https://rustfoundation.org/) [♥️ Sponsor](https://rustfoundation.org/get-involved/)
- ...and more.

### 🎯 Goal

Make WIOT sustainable.
If WIOT grows, _so should the crates and maintainers that power it._

---
