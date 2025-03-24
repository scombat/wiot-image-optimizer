# WIOT Image Optimizer

WIOT (Web Image Optimizer Toolkit) is a versatile image optimization tool designed for the web, compatible with cloud functions and servers.

## Features

- Convert images to optimized formats (webp, avif, etc.).
- Resize and adjust image quality for web use.
- Advanced transformations: blur, cropping, focus on objects, etc.
- Easy deployment on AWS Lambda, GCP Functions, Azure Functions, and more.
- Hot and cold processing: real-time image handling via HTTP or batch processing from storage.

## Installation

1. **Download the binary:**
   - Visit the [releases page](link-to-releases) and download the appropriate binary for your platform.

2. **Deploy as a cloud function:**
   - Use the provided zip files or container images to deploy on your preferred cloud provider.

3. **Run on a server:**
   - Follow the instructions in the [deployment guide](link-to-deployment-guide) to set up on any server.

## Usage

- **HTTP Handler:**
  - Use query parameters to specify transformations and get optimized images on-the-fly.

- **Batch Processing:**
  - Process images stored in S3 or local directories with ease.

## Contributing

We welcome contributions! Please read our [contribution guidelines](link-to-contribution-guidelines) to get started.

## License

This project is licensed under the GNU GPL v3.0. See the [LICENSE](LICENSE) file for details.
