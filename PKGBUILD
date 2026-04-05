# Maintainer: Siphcy archlinux.gloating053@passmail.net
pkgname=calcli
pkgver=1.0.0
pkgrel=1
pkgdesc="A lightweight TUI scientific calculator with Vi-style keybindings"
arch=('x86_64')
url="https://github.com/Siphcy/calcli"
license=('MIT')
depends=()
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('fc0e846441c2007fcd4a5ebf72037d5033f33b5fc5a33e88e91baacfec91540f')  # Run: sha256sum v#.tar.gz and update this

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --release
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
