# Publishing Guide

This document explains how to publish calcli to various package repositories.

## Quick Release Process

1. **Update version** in `Cargo.toml`
2. **Commit changes**: `git commit -am "Bump version to X.Y.Z"`
3. **Create and push tag**:
   ```bash
   git tag vX.Y.Z
   git push origin main
   git push origin vX.Y.Z
   ```
4. **GitHub Actions** will automatically build binaries for all platforms
5. **Update package repositories** (AUR, nixpkgs) - see below

---

## Publishing to AUR (Arch User Repository)

### First Time Setup

1. **Create AUR account**: https://aur.archlinux.org/register
2. **Add SSH key** to your AUR account
3. **Clone AUR repo**:
   ```bash
   git clone ssh://aur@aur.archlinux.org/calcli.git aur-calcli
   ```

### Publishing a Release

1. **Update PKGBUILD**:
   - Update `pkgver=X.Y.Z`
   - Download tarball and update sha256sum:
     ```bash
     wget https://github.com/Siphcy/calcli/archive/vX.Y.Z.tar.gz
     sha256sum vX.Y.Z.tar.gz
     ```
   - Update `sha256sums=('...')` with the hash

2. **Test build**:
   ```bash
   cd aur-calcli
   cp ../PKGBUILD .
   makepkg -si  # Build and install locally to test
   namcap PKGBUILD  # Check for issues
   ```

3. **Generate .SRCINFO**:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

4. **Commit and push**:
   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Update to X.Y.Z"
   git push
   ```

### Users Install Via

```bash
# Using yay
yay -S calcli

# Or manually
git clone https://aur.archlinux.org/calcli.git
cd calcli
makepkg -si
```

---

## Publishing to nixpkgs

### First Time Setup

1. **Fork nixpkgs**: https://github.com/NixOS/nixpkgs
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/nixpkgs.git
   cd nixpkgs
   ```

3. **Add yourself to maintainers** (first time only):

   Edit `maintainers/maintainer-list.nix`:
   ```nix
   your-github-username = {
     email = "your.email@example.com";
     github = "your-github-username";
     githubId = 12345678;  # Get from: curl https://api.github.com/users/YOUR_USERNAME
     name = "Your Name";
   };
   ```

### Creating the Package

1. **Create package file**: `pkgs/by-name/ca/calcli/package.nix`

   ```nix
   { lib
   , rustPlatform
   , fetchFromGitHub
   }:

   rustPlatform.buildRustPackage rec {
     pname = "calcli";
     version = "X.Y.Z";

     src = fetchFromGitHub {
       owner = "Siphcy";
       repo = "calcli";
       rev = "v${version}";
       hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
     };

     cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

     meta = with lib; {
       description = "A lightweight TUI scientific calculator with Vi-style keybindings";
       homepage = "https://github.com/Siphcy/calcli";
       changelog = "https://github.com/Siphcy/calcli/releases/tag/v${version}";
       license = licenses.mit;
       maintainers = with maintainers; [ your-github-username ];
       mainProgram = "calcli";
     };
   }
   ```

2. **Get correct hashes**:
   ```bash
   # Source hash
   nix-prefetch-url --unpack https://github.com/Siphcy/calcli/archive/vX.Y.Z.tar.gz

   # Build to get cargo hash (will fail but show correct hash)
   nix-build -A calcli
   ```

3. **Test the package**:
   ```bash
   nix-build -A calcli
   ./result/bin/calcli
   ```

4. **Submit PR**:
   ```bash
   git checkout -b add-calcli
   git add pkgs/by-name/ca/calcli/package.nix
   git add maintainers/maintainer-list.nix  # If adding yourself
   git commit -m "calcli: init at X.Y.Z"
   git push origin add-calcli
   ```

   Then create PR on GitHub to `NixOS/nixpkgs`

### Users Install Via

```bash
# Once merged into nixpkgs
nix-env -iA nixpkgs.calcli

# Or with nix profile
nix profile install nixpkgs#calcli

# Or add to configuration.nix
environment.systemPackages = [ pkgs.calcli ];
```

---

## Using Nix Flake (Immediate Availability)

The `flake.nix` in the repo allows users to install immediately without waiting for nixpkgs:

### Users can run:

```bash
# Run without installing
nix run github:Siphcy/calcli

# Install to profile
nix profile install github:Siphcy/calcli

# Add to flake.nix
{
  inputs.calcli.url = "github:Siphcy/calcli";
  # ...
}
```

### To test flake locally:

```bash
# Build
nix build

# Run
nix run

# Enter dev shell
nix develop
```

---

## Binary Releases (GitHub)

The `.github/workflows/release.yml` automatically creates releases when you push a tag.

### Process:
1. Tag a release: `git tag vX.Y.Z && git push origin vX.Y.Z`
2. GitHub Actions builds binaries for:
   - Linux x86_64 (gnu and musl)
   - Windows x86_64
   - macOS x86_64 (Intel)
   - macOS aarch64 (Apple Silicon)
3. Binaries appear at: https://github.com/Siphcy/calcli/releases

### Users Install Via:

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/Siphcy/calcli/main/install.sh | sh
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/Siphcy/calcli/main/install.ps1 | iex
```

---

## Publishing to cargo (crates.io)

### One-time setup:
```bash
cargo login
```

### Publish:
```bash
cargo publish
```

### Users install via:
```bash
cargo install calcli
```

---

## Checklist for New Release

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `README.md` badge
- [ ] Update version in `PKGBUILD` (for AUR)
- [ ] Update version in `flake.nix`
- [ ] Commit: `git commit -am "Bump version to X.Y.Z"`
- [ ] Tag: `git tag vX.Y.Z`
- [ ] Push: `git push origin main && git push origin vX.Y.Z`
- [ ] Wait for GitHub Actions to build binaries
- [ ] Update AUR PKGBUILD with new sha256sum
- [ ] Publish to cargo: `cargo publish`
- [ ] Create nixpkgs PR (if not already in nixpkgs)
- [ ] Announce release (optional)

---

## Quick Commands Reference

```bash
# Create release
VERSION="0.1.3"
sed -i "s/version = .*/version = \"$VERSION\"/" Cargo.toml
git commit -am "Bump version to $VERSION"
git tag "v$VERSION"
git push origin main
git push origin "v$VERSION"

# Update AUR
cd aur-calcli
# Update PKGBUILD version and sha256sum
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to $VERSION"
git push

# Publish to cargo
cargo publish
```
