#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Setting up Homebrew Tap Repository   ${NC}"
echo -e "${BLUE}========================================${NC}"

# Check for GitHub CLI
if ! command -v gh &> /dev/null; then
    echo -e "${RED}GitHub CLI (gh) is required but not installed.${NC}"
    echo -e "${YELLOW}Please install it first:${NC}"
    echo -e "  brew install gh"
    echo -e "  gh auth login"
    exit 1
fi

# Check if logged in to GitHub
if ! gh auth status &> /dev/null; then
    echo -e "${RED}You are not logged in to GitHub.${NC}"
    echo -e "${YELLOW}Please login first:${NC}"
    echo -e "  gh auth login"
    exit 1
fi

# Get GitHub username
GITHUB_USERNAME=$(gh api user | jq -r .login)
if [ -z "$GITHUB_USERNAME" ]; then
    echo -e "${RED}Failed to get GitHub username.${NC}"
    exit 1
fi

# Repository name
REPO_NAME="homebrew-serve"
REPO_DESCRIPTION="Homebrew Tap for the Serve file server"

echo -e "${BLUE}Creating Homebrew tap repository for ${GITHUB_USERNAME}/serve...${NC}"

# Create or clone repository
if ! gh repo view "${GITHUB_USERNAME}/${REPO_NAME}" &> /dev/null; then
    echo -e "${YELLOW}Repository doesn't exist, creating it...${NC}"
    gh repo create "${REPO_NAME}" --public --description "${REPO_DESCRIPTION}" --clone
    cd "${REPO_NAME}"
else
    echo -e "${YELLOW}Repository exists, cloning it...${NC}"
    gh repo clone "${GITHUB_USERNAME}/${REPO_NAME}"
    cd "${REPO_NAME}"
fi

# Create directory structure
mkdir -p Formula

# Copy README from parent project if it exists
if [ -f "../homebrew-tap-readme.md" ]; then
    echo -e "${BLUE}Using custom README template${NC}"
    cat "../homebrew-tap-readme.md" | sed "s/USERNAME/${GITHUB_USERNAME}/g" > README.md
else
    # Create a basic README
    cat > README.md << EOF
# Homebrew Tap for Serve

This repository is a [Homebrew Tap](https://docs.brew.sh/Taps) for the [Serve](https://github.com/${GITHUB_USERNAME}/serve) file server.

## Installation

### Add the tap

\`\`\`bash
brew tap ${GITHUB_USERNAME}/serve
\`\`\`

### Install serve

\`\`\`bash
brew install serve
\`\`\`
EOF
fi

# Create a Formula directory placeholder
cat > Formula/.gitkeep << EOF
# This file ensures the Formula directory is tracked by Git
# It will be replaced by the actual formula file during GitHub Actions release
EOF

# Git setup
git add README.md Formula/.gitkeep
git config --global user.name "$(git config user.name || echo 'GitHub Actions')"
git config --global user.email "$(git config user.email || echo 'actions@github.com')"

# Check if we need to commit (could be a re-run of the script)
if ! git diff --cached --quiet; then
    git commit -m "Initial setup of Homebrew tap"
    git push
    echo -e "${GREEN}Successfully created and initialized ${REPO_NAME}!${NC}"
else
    echo -e "${YELLOW}No changes to commit.${NC}"
fi

echo -e "${GREEN}=================================${NC}"
echo -e "${GREEN}  Homebrew Tap Repository Ready  ${NC}"
echo -e "${GREEN}=================================${NC}"
echo ""
echo -e "${BLUE}To use this tap, users will run:${NC}"
echo -e "  brew tap ${GITHUB_USERNAME}/serve"
echo -e "  brew install serve"
echo ""
echo -e "${YELLOW}Note: You'll need to create a GitHub personal access token with 'repo' scope${NC}"
echo -e "${YELLOW}and add it as a secret named HOMEBREW_GITHUB_TOKEN in your serve repository.${NC}"
