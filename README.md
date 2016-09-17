# TMSU Nautilus Extension

GNOME Nautilus extension for file tagging using [TMSU](https://github.com/oniony/TMSU/).

## Requirements

* Nautilus 3
* Gtk+ 3
* [TMSU](https://github.com/oniony/TMSU/) 0.6.1+
* Rust 1.11.0+ (if compiling from source)

## Installation

Download the latest release from [Releases](https://github.com/talklittle/tmsu-nautilus-rs/releases).

Copy (or symlink) `libtmsu-nautilus.so` into the extensions directory:

    sudo cp libtmsu-nautilus.so /usr/lib/nautilus/extensions-3.0/libtmsu-nautilus.so

Kill and restart Nautilus:

    nautilus -q

## Compiling from source

    sudo apt install libnautilus-extension-dev
    cd tmsu-nautilus-rs
    cargo build --release
    ls -l target/release/libtmsu_nautilus.so

## Release notes

See [CHANGELOG.md](CHANGELOG.md) for changes between versions.

## License

[GNU General Public License version 3](COPYING.txt)