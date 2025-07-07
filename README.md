# WIOT Image Optimizer
![WIP](https://img.shields.io/badge/status-WIP-yellow?style=flat-square&logo=github)
![License](https://img.shields.io/badge/license-GPLv3-blue?style=flat-square)



**WIOT** (Web Image Optimizer Toolkit) is an open, modular, and extensible image optimization tool focused on web performance.
It is designed to run as a CLI, on servers, or inside cloud functions.

> 🚧 **This project is currently under active development.**
> Expect things to break, evolve, and improve rapidly. Feedback and contributions are welcome!

---
[![Bugs](https://sonarcloud.io/api/project_badges/measure?project=scombat_wiot-image-optimizer&metric=bugs)](https://sonarcloud.io/summary/new_code?id=scombat_wiot-image-optimizer)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=scombat_wiot-image-optimizer&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=scombat_wiot-image-optimizer)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=scombat_wiot-image-optimizer&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=scombat_wiot-image-optimizer)
[![Maintainability Rating](https://sonarcloud.io/api/project_badges/measure?project=scombat_wiot-image-optimizer&metric=sqale_rating)](https://sonarcloud.io/summary/new_code?id=scombat_wiot-image-optimizer)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=scombat_wiot-image-optimizer&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=scombat_wiot-image-optimizer)

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
- ✅ Early support for resize options (via ProcessingOptions)


## 🧩 Feature Progress

#### Entrypoints Support

| Feature                                 | Core 🧠 | CLI ✅ | API ❌ | Web UI ❌ |
| --------------------------------------- | ------ | ----- | ----- | -------- |
| Resize                                  | ✅      | ✅     | ❌     | ❌        |
| ├─ Width / Height                       | ✅     | ✅     | ❌     | ❌        |
| ├─ DPI                                  | ✅     | ✅     | ❌     | ❌        |
| ├─ Maintain Aspect Ratio                | ✅     | ✅     | ❌     | ❌        |
| ├─ Cover Strategy                       | ✅     | ✅     | ❌     | ❌        |
| ├─ Stretch Strategy                     | ✅     | ✅     | ❌     | ❌        |
| ├─ Contain Strategy                     | ✅     | ✅     | ❌     | ❌        |
| └─ Fill Strategy                        | ✅     | ✅     | ❌     | ❌        |
| Quality Optimization                    | ✅      | ✅     | ❌     | ❌        |
| Format Conversion                       | ✅      | ✅     | ❌     | ❌        |
| Encode/Store                            | ✅      | ✅     | ❌     | ❌        |
| Auto-select Best Format (smallest file)  | 🚧      | ❌     | ❌     | ❌        |
| Crop                                    | 🔜      | ❌     | ❌     | ❌        |
| ├─ Width / Height                       | ✅     | ✅     | ❌     | ❌        |
| ├─ Gravity (position)                   | ✅     | ✅     | ❌     | ❌        |
| └─  AI Object gravity selection         | 🔜     | ❌     | ❌     | ❌        |
| Flip (horizontal/vertical)              | 🔜      | ❌     | ❌     | ❌        |
| Rotate                                  | 🔜      | ❌     | ❌     | ❌        |
| Blur                                    | 🔜      | ❌     | ❌     | ❌        |
| Grayscale / Color Effects               | 🔜      | ❌     | ❌     | ❌        |
| Sharpen                                 | 🔜      | ❌     | ❌     | ❌        |
| Watermark / Overlay                     | 🔜      | ❌     | ❌     | ❌        |
| Metadata Handling                       | 🔜      | ❌     | ❌     | ❌        |

#### Format Support

| Format   | Encode | Decode | Notes                  |
| -------- | ------ | ------ | ---------------------- |
| JPEG     | ✅      | ✅      | Native quality support |
| PNG      | ✅      | ✅      | Custom quality mapping |
| WebP     | ✅      | ✅      | Requires `webp` crate  |
| AVIF     | ❌      | ❌      | Planned                |
| GIF      | ❌      | ❌      | Planned                |
| HDR      | ❌      | ❌      | Planned                |
| BMP      | ❌      | ❌      | Planned                |
| TIFF     | ❌      | ❌      | Planned                |
| ICO      | ❌      | ❌      | Planned                |
| DSS      | ❌      | ❌      | Optional, low priority |
| Farbfeld | ❌      | ❌      | Optional, low priority |
| EXR      | ❌      | ❌      | Optional, low priority |
| PNM      | ❌      | ❌      | Optional, low priority |
| QOI      | ❌      | ❌      | Optional, low priority |
| TGA      | ❌      | ❌      | Optional, low priority |

#### 💡 Ideas & Extras

| Feature / Idea                  | Status | Notes                                            |
| ------------------------------- | ------ | ------------------------------------------------ |
| Preset Processing               | 🧠      | Define named transformation profiles             |
| Error Handling / Logs           | 🧠      | Structured error types and better CLI output     |
| Contributing Guide              | 🛠️      | Add docs and contribution rules to the repo      |
| Docker Image                    | 🧪      | For local usage or CI/CD pipelines               |
| Golden Image Testing            | 🔜      | Compare output with golden snapshots             |
| AWS Lambda Zip                  | 🔜      | Export zip with binary + dependencies            |
| One-click Cloud Deploy          | 🔜      | AWS / GCP / Azure deployment shortcuts           |
| GCP Function                    | ❌      | Bundle for Google Cloud Function                 |
| Azure Function                  | ❌      | Same as GCP, with bindings                       |
| S3 Support (Read / Write)       | 🔜      | Through `FileAdapter` abstraction                |
| GUI                             | 🔜      | Lightweight Web UI for upload / preview / output |
| CMS Plugins (WordPress, Ghost…) | ❌      | External plugins to use this as a backend        |
| `srcset` / Responsive Variants  | 🔜      | Multi-resolution generation (for HTML img)       |
| Base64 Encode Output            | ❌      | Useful for CSS background images or inline HTML  |

**Legend**:
- 🧠 Idea / planned feature
- 🛠️ In progress
- 🔜 Coming soon
- 🧪 Experimental / needs validation
- ❌ Not planned (yet)
- ✅ Done / implemented


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

- Install required tools (`rustfmt`, `cargo-tarpaulin`, `dav1d` etc.)
- Set up Git hooks to enforce formatting, linting, and testing before commit/push
- Prepare your local dev environment

> 💡 If you don’t have [`just`](https://github.com/casey/just) installed yet, do:
> `cargo install just`

#### System requirements (build)

This project requires the following libraries to be installed on your environment:
- [dav1d](https://github.com/videolan/dav1d)

> 💡 If the install script did not work for the [`dav1d`](https://github.com/videolan/dav1d) library, you can try the following:
> ##### macOS (Homebrew)
> ```sh
> brew install dav1d
> ```
>
> ##### Ubuntu/Debian
> ```sh
> sudo apt-get update
> sudo apt-get install libdav1d-dev pkg-config
> ```
>
> ##### Fedora
> ```sh
> sudo dnf install dav1d-devel pkgconf-pkg-config
> ```

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
