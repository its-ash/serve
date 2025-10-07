# Serve - A Simple File Server in Rust

A lightweight, modern file server written in Rust that serves the current directory and its contents with a clean Bootstrap 5 UI.

## Features

- 📂 Browse and navigate through nested directories
- 📄 View and download files
- 🎨 Clean, responsive UI using Bootstrap 5
- 🚀 Fast and lightweight (built in Rust)
- 🔧 Configurable bind address and port
- 📊 File size and modification time information

## Screenshots

(Add screenshots here)

## Installation

### Using Homebrew (macOS and Linux)

The easiest way to install Serve is via Homebrew:

```bash
# Add the tap
brew tap yourusername/serve

# Install serve
brew install serve
```

### Pre-built Binaries

You can download pre-built binaries for Windows, macOS, and Linux from the [Releases page](https://github.com/yourusername/serve/releases).

### Building from Source

#### Prerequisites

- Rust and Cargo (https://rustup.rs/)

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/serve.git
   cd serve
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/serve`

## Usage

```
serve [OPTIONS]

Options:
  -b, --bind <BIND>  IP address to bind to [default: 127.0.0.1]
  -p, --port <PORT>  Port to listen on [default: 8080]
  -h, --help         Print help
  -V, --version      Print version
```

## Examples

Serve the current directory on the default port (8080) and bind to localhost:

```bash
serve
```

Serve on a specific port:

```bash
serve --port 3000
```

Bind to all interfaces (make the server accessible from other devices on the network):

```bash
serve --bind 0.0.0.0
```

Combine options:

```bash
serve --bind 0.0.0.0 --port 5000
```

## How It Works

Serve uses the Actix Web framework to create a high-performance web server that:

1. Serves static files directly from the filesystem
2. Generates directory listings with a modern UI
3. Provides breadcrumb navigation for easier directory traversal
4. Shows file metadata (size, modification time)

## License

MIT License

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.

## Releasing

This project uses GitHub Actions to automatically build and release binaries for multiple platforms and publish to Homebrew.

See [RELEASE.md](RELEASE.md) for detailed instructions on releasing a new version.
