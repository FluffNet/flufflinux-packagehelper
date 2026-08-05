# Fluff Linux Package Helper

Fluff Linux Package Helper explains why a Debian package cannot be installed
directly on Fluff Linux and points users to Discover or `pacman`.

The helper:

- identifies packages by content with `file`, never by filename or extension;
- displays its own in-process Qt 6 dialog without KDialog;
- follows the active KDE/Qt color, widget, font, and icon themes;
- uses the `dialog-information` icon from the KDE icon theme;
- supports right-to-left dialog layout and bidirectional filenames;
- selects its language from `LC_ALL`, `LANGUAGE`, `LC_MESSAGES`, or `LANG`.

Translations are included for Arabic, Basque, Bulgarian, Catalan, Chinese
(Simplified and Traditional), Czech, Danish, Dutch, English, Finnish, French,
German, Greek, Hebrew, Hungarian, Indonesian, Italian, Japanese, Korean,
Norwegian Bokmål, Persian, Polish, Portuguese, Brazilian Portuguese, Romanian,
Russian, Slovak, Spanish, Swedish, Turkish, Ukrainian, and Vietnamese.

## Build and package

These commands assume they are running in the package's fakeroot with
`base-devel`, Rust, `qt6-base`, and `file` already available. They do not use
`sudo`.

```sh
cargo build --release --locked
install -Dm755 target/release/flufflinux-packagehelper \
  "${pkgdir}/usr/bin/flufflinux-packagehelper"
install -Dm644 flufflinux-packagehelper.desktop \
  "${pkgdir}/usr/share/applications/flufflinux-packagehelper.desktop"
```

The runtime dependencies are Qt 6 Widgets and `file`.
