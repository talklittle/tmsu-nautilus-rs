# TMSU Nautilus Extension

GNOME Nautilus extension for file tagging using [TMSU](https://github.com/oniony/TMSU/).

**[Screenshots](SCREENSHOTS.md)**

## Requirements

* Nautilus 3
* Gtk+ 3.18+
* [TMSU](https://github.com/oniony/TMSU/) 0.6.1+
* Rust 1.27+ (if compiling from source)

## Installation

Download the latest release from [Releases](https://github.com/talklittle/tmsu-nautilus-rs/releases).

**From .deb (Ubuntu/Debian):**

    sudo dpkg -i tmsu-nautilus_${VERSION}_${ARCH}.deb

    # Kill and restart Nautilus:
    nautilus -q

**Alternatively, copy .so manually:**

    sudo cp libtmsu-nautilus.so /usr/lib/nautilus/extensions-3.0/libtmsu-nautilus.so

    # Kill and restart Nautilus:
    nautilus -q

## Compiling from source

    sudo apt install libnautilus-extension-dev libgtk-3-0
    cd tmsu-nautilus-rs
    make

    # (Optional) Generate .deb file. Requires https://github.com/mmstick/cargo-deb
    cargo deb --no-build

## Release notes

See [CHANGELOG.md](CHANGELOG.md) for changes between versions.

## License

[GNU General Public License version 3](COPYING.txt)
