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

## Build on Fluff Linux or Arch Linux

Install the Rust toolchain and Qt 6 development files:

```sh
sudo pacman -S --needed base-devel rust qt6-base file
cargo build --release
```

The resulting executable is
`target/release/flufflinux-packagehelper`.
