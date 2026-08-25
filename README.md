# Fluff Linux Package Helper

Fluff Linux Package Helper identifies Debian and RPM package files by content.
It explains why they cannot be installed directly on Fluff Linux.
It uses a native Qt 6 dialog with KDE theming, localization, and RTL support.
It directs users to the App Center or a compatible version online.

## Compile for a fakeroot

```sh
cargo build --release
install -Dm755 target/release/flufflinux-packagehelper /path/to/fakeroot/usr/bin/flufflinux-packagehelper
install -Dm644 flufflinux-packagehelper.desktop /path/to/fakeroot/usr/share/applications/flufflinux-packagehelper.desktop
```

## Compile for installation

```sh
cargo build --release
sudo install -Dm755 target/release/flufflinux-packagehelper /usr/bin/flufflinux-packagehelper
sudo install -Dm644 flufflinux-packagehelper.desktop /usr/share/applications/flufflinux-packagehelper.desktop
```
