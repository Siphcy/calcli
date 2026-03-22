#!/bin/bash
# Update version across all files in the project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: ./update-version.sh <new-version>"
    echo "Example: ./update-version.sh 0.1.3"
    exit 1
fi

NEW_VERSION="$1"

# Validate version format (basic check)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version should be in format: X.Y.Z (e.g., 0.1.3)"
    exit 1
fi

echo -e "${YELLOW}Updating version to $NEW_VERSION...${NC}"
echo ""

# Update Cargo.toml
if [ -f "Cargo.toml" ]; then
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
    echo -e "${GREEN}✓${NC} Updated Cargo.toml"
else
    echo -e "${RED}✗${NC} Cargo.toml not found"
fi

# Update flake.nix
if [ -f "flake.nix" ]; then
    sed -i "s/version = \".*\";/version = \"$NEW_VERSION\";/" flake.nix
    echo -e "${GREEN}✓${NC} Updated flake.nix"
else
    echo -e "${YELLOW}⚠${NC} flake.nix not found (skipping)"
fi

# Update PKGBUILD
if [ -f "PKGBUILD" ]; then
    sed -i "s/^pkgver=.*/pkgver=$NEW_VERSION/" PKGBUILD
    sed -i "s/^pkgrel=.*/pkgrel=1/" PKGBUILD  # Reset pkgrel to 1
    echo -e "${GREEN}✓${NC} Updated PKGBUILD"
else
    echo -e "${YELLOW}⚠${NC} PKGBUILD not found (skipping)"
fi

# Update README.md badge
if [ -f "README.md" ]; then
    sed -i "s/version-[0-9]\+\.[0-9]\+\.[0-9]\+-blue/version-$NEW_VERSION-blue/" README.md
    echo -e "${GREEN}✓${NC} Updated README.md badge"
else
    echo -e "${YELLOW}⚠${NC} README.md not found (skipping)"
fi

echo ""
echo -e "${GREEN}Version updated successfully!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Update Cargo.lock:"
echo -e "   ${GREEN}cargo build${NC}"
echo ""
echo "2. Update PKGBUILD sha256sum after creating the release:"
echo -e "   ${GREEN}wget https://github.com/Siphcy/calcli/archive/v$NEW_VERSION.tar.gz${NC}"
echo -e "   ${GREEN}sha256sum v$NEW_VERSION.tar.gz${NC}"
echo -e "   ${GREEN}# Update PKGBUILD with the hash${NC}"
echo ""
echo "3. Commit changes:"
echo -e "   ${GREEN}git add -A${NC}"
echo -e "   ${GREEN}git commit -m \"Bump version to $NEW_VERSION\"${NC}"
echo ""
echo "4. Create and push tag:"
echo -e "   ${GREEN}git tag v$NEW_VERSION${NC}"
echo -e "   ${GREEN}git push origin main${NC}"
echo -e "   ${GREEN}git push origin v$NEW_VERSION${NC}"
echo ""
echo "5. Wait for GitHub Actions to build binaries"
echo ""
echo "6. Update AUR (after getting sha256sum):"
echo -e "   ${GREEN}makepkg --printsrcinfo > .SRCINFO${NC}"
echo -e "   ${GREEN}# Push to AUR repo${NC}"
echo ""
