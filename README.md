# TMSU Nautilus Extension

GNOME Nautilus extension for file tagging using [TMSU](https://github.com/oniony/TMSU/).

**[Screenshots](SCREENSHOTS.md)**

## Requirements

* Nautilus 3
* Gtk+ 3.20+
* [TMSU](https://github.com/oniony/TMSU/) 0.6.1+
* Rust 1.57+ (if compiling from source)

## Installation

Download the latest release from [Releases](https://github.com/talklittle/tmsu-nautilus-rs/releases).

If your installation is successful, you will see this Nautilus logs when running from the terminal.

```shell
$ nautilus
Initializing TMSU Nautilus 0.6.2
```

**From .deb (Ubuntu/Debian):**

    sudo dpkg -i tmsu-nautilus_${VERSION}_${ARCH}.deb

    # Kill and restart Nautilus:
    nautilus -q

**Alternatively, copy .so manually:**

    # Debian
    sudo cp libtmsu-nautilus.so /usr/lib/nautilus/extensions-3.0/libtmsu-nautilus.so
    # Fedora
    sudo cp libtmsu-nautilus.so /usr/lib64/nautilus/extensions-3.0/libtmsu-nautilus.so

    # Make sure the extension has the correct permission
    sudo chmod u=rwxr,g=xr,o=x libtmsu-nautilus.so
    sudo chmod g+x libtmsu-nautilus.so

    # Kill and restart Nautilus:
    nautilus -q

**Archlinux AUR**

Use the AUR package [nautilus-tmsu-rs-git](https://aur.archlinux.org/packages/nautilus-tmsu-rs-git/).

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
