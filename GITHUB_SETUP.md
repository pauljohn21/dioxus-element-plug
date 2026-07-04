# GitHub Repository Setup Guide

Follow these steps to upload the dioxus-element-plug project to GitHub.

## Prerequisites

1. **GitHub Account** - Make sure you have a GitHub account
2. **Git CLI** - Already installed and configured
3. **GitHub CLI (optional but recommended)** - `gh` command line tool

## Setup Options

### Option 1: Using GitHub CLI (Recommended)

#### 1. Install GitHub CLI (if not already installed)
```bash
# macOS
brew install gh

# Or download from https://cli.github.com/
```

#### 2. Authenticate with GitHub
```bash
gh auth login
# Follow prompts to login to your GitHub account
```

#### 3. Create the repository
```bash
cd /Users/pauljohn/rust/theme-chalk
gh repo create dioxus-element-plug --public --source=. --remote=origin
```

### Option 2: Manual Setup via GitHub Website

#### 1. Create Repository on GitHub Website
1. Go to [github.com/new](https://github.com/new)
2. Repository name: `dioxus-element-plug`
3. Description: "Dioxus UI components with Element UI theme-chalk styling"
4. Choose Public or Private
5. **Do NOT** initialize with README, .gitignore, or license
6. Click "Create repository"

#### 2. Add Remote and Push
```bash
cd /Users/pauljohn/rust/theme-chalk

# Add the remote repository (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/dioxus-element-plug.git

# Rename branch to main (if needed)
git branch -M main

# Push to GitHub
git push -u origin main
```

### Option 3: Using SSH (Advanced Users)

If you prefer SSH over HTTPS:

#### 1. Setup SSH Keys (if not done)
```bash
# Generate SSH key (if needed)
ssh-keygen -t ed25519 -C "your_email@example.com"

# Add to ssh-agent
ssh-add ~/.ssh/id_ed25519

# Add public key to GitHub account
cat ~/.ssh/id_ed25519.pub
# Copy output and add to GitHub SSH keys in account settings
```

#### 2. Create Repository (same as Option 2)

#### 3. Add SSH Remote and Push
```bash
cd /Users/pauljohn/rust/theme-chalk

# Add SSH remote (replace YOUR_USERNAME)
git remote add origin git@github.com:YOUR_USERNAME/dioxus-element-plug.git

git branch -M main
git push -u origin main
```

## Verification

After setup, verify everything worked:

```bash
git remote -v
# Should show origin pointing to your GitHub repo

# Test clone in a different location
git clone https://github.com/YOUR_USERNAME/dioxus-element-plug.git /tmp/test-clone
echo "Test clone successful!"
```

## Post-Setup Tasks

### 1. Update Repository Settings

Go to your repository on GitHub and configure:

1. **Topics**: Add tags like `dioxus`, `rust`, `ui-components`, `element-ui`, `css`
2. **Description**: Ensure it matches your project
3. **Website**: Add website URL if you have one
4. **Issues**: Enable issues for bug reports
5. **Discussions**: Enable for community discussions
6. **Projects**: Enable if you want to use GitHub Projects

### 2. Setup GitHub Actions (Recommended)

Create `.github/workflows/ci.yml` for continuous integration:

```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - run: cargo test
    - run: cargo check --examples

  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - run: cargo build --release
```

### 3. Create GitHub Pages (Optional)

For documentation website:

1. Go to Repository Settings → Pages
2. Source: `Deploy from a branch`
3. Branch: `gh-pages` (or create one)
4. Folder: `/root`
5. Save

### 4. Protect Main Branch

1. Settings → Branches
2. Add branch protection rule for `main`
3. Enable:
   - Require pull request reviews
   - Require status checks to pass
   - Include administrators

## Troubleshooting

### Permission Denied
```bash
# If you get permission errors:
git remote remove origin
git remote add origin https://YOUR_USERNAME:YOUR_TOKEN@github.com/YOUR_USERNAME/dioxus-element-plug.git
```

### Large Files
```bash
# If you accidentally committed large files:
git filter-branch --tree-filter 'rm -f large-file.zip' HEAD
git push origin main --force
```

### Detached HEAD
```bash
git status  # Check current state
git checkout main  # Get back to main branch
```

## Next Steps

1. **Write Better Documentation**: Expand README with more examples
2. **Add Tests**: Create comprehensive test suite
3. **Setup CI/CD**: GitHub Actions for automated testing
4. **Add Badges**: CI status, code coverage, etc.
5. **Publish to crates.io**: Make available as a Rust package
6. **Create Issues**: Plan future features and improvements

## Project Structure on GitHub

After upload, your repository will contain:

```
dioxus-element-plug/
├── .gitignore              # Git ignore rules
├── .cargo/config.toml      # Cargo configuration
├── Cargo.toml              # Rust project manifest
├── README.md               # Project documentation
├── CHANGELOG.md            # Version history
├── QUICKSTART.md           # Quick start guide
├── Makefile               # Build automation
├── build.rs               # Build script
├── rust-toolchain.toml    # Rust toolchain config
├── src/                   # Rust source code
├── scss/                  # Organized SCSS files
├── examples/              # Example applications
└── (GitHub specific files)
```

Your dioxus-element-plug project is now ready to share with the world! 🚀
