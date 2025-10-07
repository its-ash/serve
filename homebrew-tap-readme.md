# Homebrew Tap for Serve

This repository is a [Homebrew Tap](https://docs.brew.sh/Taps) for the [Serve](https://github.com/USERNAME/serve) file server.

## What is Serve?

Serve is a lightweight, modern file server written in Rust that serves directory contents with a clean Bootstrap 5 UI. It allows you to:

- 📂 Browse and navigate through nested directories
- 📄 View and download files
- 🎨 Clean, responsive UI using Bootstrap 5
- 🚀 Fast and lightweight (built in Rust)
- 🔧 Configurable bind address and port

## Installation

### Add the tap

```bash
brew tap USERNAME/serve
```

### Install serve

```bash
brew install serve
```

## Usage

```
serve [OPTIONS]

Options:
  -b, --bind <BIND>  IP address to bind to [default: 127.0.0.1]
  -p, --port <PORT>  Port to listen on [default: 8080]
  -h, --help         Print help
  -V, --version      Print version
```

### Examples

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

## Issues

If you encounter any problems with the formula, please file an issue on the [main repository](https://github.com/USERNAME/serve/issues).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
