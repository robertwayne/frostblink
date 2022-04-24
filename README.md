# Frostblink

Frostblink is a cross-platform tool to run alongside Path of Exile that allows users to define hotkeys, fast logout (TCP disconnect), price checking, and quick access to the trade site, craftofexile, and poe.db based on items or their base types.

If you are a Windows-user, there are many alternatives available to you, like Lutbot and Awakened POE. I made this because, on Linux, there were always issues with the available options.

*Disclaimer: While I am building this with cross-platform libraries, Linux is my daily driver. Features will be developed for Linux initially, then ported to Windows as I get time and can test it.*

## Usage

Don't use it yet.

## GNOME x11 System Stutter

Due to an issue in one of the underlying libraries in GNOME, you may need to patch your mutter version in order to avoid a system stutter every time you send virtual input. You can use my [libmutter-fix](https://github.com/robertwayne/libmutter-fix) script to attempt to patch & recompile libmutter, or patch it manually with [this guide](https://gitlab.gnome.org/GNOME/gnome-shell/-/issues/1858#note_818548).

## Contributing

I'm open to any suggestions, bug reports, or contributions. If you wish to contribute, it is preferable to open an issue first (unless it is small), as many things will be changing very quickly.

Prerequesties:

- Rust 1.60 (nightly)

*By contributing, you agree that any code submitted by you shall be dual-licensed under MIT and Apache-2.0.*

## License

Frostblink source code is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
