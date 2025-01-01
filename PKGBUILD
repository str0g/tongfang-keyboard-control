# Maintainer: Łukasz Buśko <lukasz.busko@guns4hire.cc>

pkgname=tongfang-keyboard-control-git
pkgver=0.1
pkgrel=1
arch=('x86_64')

depends=('rust'
)
makedepends=('git' 'udev' 'systemd')

source=("git+https://github.com/str0g/tongfang-keyboard-control-git")
sha256sums=('SKIP')

pkgdesc="tongfang keyboard control utility"
license=('MPL')
install=hook.install

build() {
  cargo build --release
}

package() {
  currnet_srcdir=".."
  install -dm755 "$pkgdir/usr/bin"
  install -Dm755 "$currnet_srcdir/target/release/tongfang-keyboard-control" "$pkgdir/usr/bin/"
  #
  install -dm755 "$pkgdir/usr/lib/systemd/system"
  install -Dm644 "$currnet_srcdir/examples/tongfang-keyboard-control.service" "$pkgdir/usr/lib/systemd/system/"
  #
  install -dm755 "$pkgdir/etc/udev/rules.d"
  install -Dm644 "$currnet_srcdir/examples/99-keyboard.rules" "$pkgdir/etc/udev/rules.d/"
}
