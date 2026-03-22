#!/bin/bash
# Update version across all files

set -e

if [ -z "$1" ]; then
    echo "Usage: ./update-version.sh <new-version>"
    echo "Example: ./update-version.sh 0.1.3"
    exit 1
fi

NEW_VERSION="$1"

echo "Updating version to $NEW_VERSION..."

# Update Cargo.toml
sed -i "s/^version = .*/version = \"$NEW_VERSION\"/" Cargo.toml

# Update flake.nix
sed -i "s/version = \".*\";/version = \"$NEW_VERSION\";/" flake.nix

# Update PKGBUILD
sed -i "s/^pkgver=.*/pkgver=$NEW_VERSION/" PKGBUILD

echo "✓ Updated Cargo.toml"
echo "✓ Updated flake.nix"
echo "✓ Updated PKGBUILD"
echo ""
echo "Note: You still need to:"
echo "  1. Update PKGBUILD sha256sum manually after creating the release"
echo "  2. Commit: git commit -am 'Bump version to $NEW_VERSION'"
echo "  3. Tag: git tag v$NEW_VERSION"
echo "  4. Push: git push origin main && git push origin v$NEW_VERSION"
