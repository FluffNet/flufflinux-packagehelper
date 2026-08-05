# Fluff Linux Package Helper

Fluff Linux Package Helper explains why Debian and RPM packages cannot be
installed directly on Fluff Linux and points users to Discover or `pacman`.

The helper:

- identifies packages by content with `file`, never by filename or extension;
- displays its own in-process Qt 6 dialog without KDialog;
- follows the active KDE/Qt color, widget, font, and icon themes;
- uses the `dialog-information` icon from the KDE icon theme;
- uses the desktop entry's `package` icon for the application window;
- stays on top as a fixed-size, movable dialog without minimize or maximize;
- measures and wraps localized text before sizing so translated content is not
  cut off;
- supports right-to-left dialog layout and bidirectional filenames;
- selects its language from `LC_ALL`, `LANGUAGE`, `LC_MESSAGES`, or `LANG`.

Translations are included for Arabic, Basque, Bulgarian, Catalan, Chinese
(Simplified and Traditional), Czech, Danish, Dutch, English, Finnish, French,
German, Greek, Hebrew, Hungarian, Indonesian, Italian, Japanese, Korean,
Norwegian Bokmål, Persian, Polish, Portuguese, Brazilian Portuguese, Romanian,
Russian, Slovak, Spanish, Swedish, Turkish, Ukrainian, and Vietnamese.

## Compile

Compilation requires Rust, `base-devel`, `pkgconf`, `qt6-base`, and `file`.
The project has no Cargo dependencies: its Qt bridge is compiled with the
system C++ compiler and linked directly against the system Qt 6 libraries.

### Fakeroot build

When compiling inside the fakeroot used by the Fluff Linux packaging tools:

```sh
cargo build --release
```

The packaging tools are responsible for staging the compiled binary and
`flufflinux-packagehelper.desktop`.

### Installation build

To compile a release build before installation:

```sh
cargo build --release
```

The resulting executable is
`target/release/flufflinux-packagehelper`. The runtime dependencies are Qt 6
Widgets and `file`.
