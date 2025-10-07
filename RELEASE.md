# Release Process

This document describes how to release a new version of Serve to GitHub Releases and Homebrew.

## Prerequisites

Before you can release a new version, you need:

1. **GitHub Personal Access Token**: Create a token with the `repo` scope at [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens).

2. **Add Token to Repository Secrets**: Go to your repository settings, then "Secrets and variables" > "Actions". Add your token as `HOMEBREW_GITHUB_TOKEN`.

3. **Homebrew Tap Repository**: You need a GitHub repository named `homebrew-serve` to host the Homebrew formula.

## Setting Up Homebrew Tap

We provide a helper script to set up the Homebrew tap repository:

```bash
# Make sure the script is executable
chmod +x scripts/setup-homebrew-tap.sh

# Run the setup script
./scripts/setup-homebrew-tap.sh
```

This script will:
- Check for GitHub CLI installation
- Create or clone the `homebrew-serve` repository
- Set up the necessary directory structure
- Create a basic README

## Release Process

### 1. Update Version

Update the version in `Cargo.toml`:

```toml
[package]
name = "serve"
version = "0.2.0"  # <-- Update this
```

### 2. Create a Git Tag

```bash
# Commit your changes first
git add Cargo.toml
git commit -m "Bump version to 0.2.0"

# Create and push a tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 3. GitHub Actions Workflow

Once you push a tag starting with `v`, the GitHub Actions workflow will automatically:

1. Create a new GitHub Release
2. Build binaries for:
   - Linux (x86_64, ARM64)
   - macOS (x86_64, ARM64)
   - Windows (x86_64)
3. Upload the binaries to the GitHub Release
4. Generate a Homebrew formula and update your tap repository

### 4. Verify the Release

- Check the GitHub Actions workflow to make sure all steps completed successfully
- Verify the GitHub Release has all expected assets
- Check the Homebrew formula in your tap repository
- Test installation via Homebrew:
  ```bash
  brew tap yourusername/serve
  brew install serve
  ```

## Troubleshooting

### GitHub Actions Workflow Failed

1. Check the workflow logs for errors
2. Most common issues:
   - Missing `HOMEBREW_GITHUB_TOKEN` secret
   - Insufficient permissions for the token
   - Homebrew tap repository doesn't exist

### Homebrew Formula Issues

If the formula is incorrect or not working:

1. Check the SHA256 checksums in the formula
2. Verify the download URLs are correct
3. Make sure the binary is being installed correctly

## Manual Release Process (if GitHub Actions fails)

If you need to release manually:

1. Build binaries for each platform:
   ```bash
   cargo build --release
   ```

2. Create a GitHub Release through the web interface

3. Upload the binaries manually

4. Create a Homebrew formula manually:
   ```ruby
   class Serve < Formula
     desc "Simple file server with a modern UI"
     homepage "https://github.com/yourusername/serve"
     version "0.2.0"

     on_macos do
       on_arm do
         url "https://github.com/yourusername/serve/releases/download/v0.2.0/serve-darwin-arm64"
         sha256 "YOUR_SHA256_HERE"
       end

       on_intel do
         url "https://github.com/yourusername/serve/releases/download/v0.2.0/serve-darwin-amd64"
         sha256 "YOUR_SHA256_HERE"
       end
     end

     def install
       bin.install "serve-darwin-arm64" => "serve"
     end
   end
   ```
