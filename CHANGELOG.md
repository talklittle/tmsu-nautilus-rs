# Changelog

## 0.5.5 (2021-08-27)

* Automate GitHub release using GitHub Actions.

## 0.5.4 (2020-04-30)

* Updated nautilus-extension to 0.5.0.
* Updated gtk-rs dependencies.

## 0.5.3 (2019-04-18)

* Updated nautilus-extension to 0.4.0.

## 0.5.2 (2019-02-10)

* Fixed Debian .deb package dependencies.

## 0.5.1 (2019-02-10)

* Updated dependencies.
* Built with Rust 1.32 (not a strict requirement).

## 0.5.0 (2018-07-28)

* Fixed crash on Properties page when opening remote files.
* Require Rust 1.27.

## 0.4.2 (2018-04-03)

* Fixed crash on Properties page.

## 0.4.1 (2018-03-24)

* Handle filenames with colon (`:`).

## 0.4.0 (2018-02-19)

* Fixed compilation on Rust 1.24.
* Require GTK+ 3.18+.
* Updated gtk-rs dependencies for various fixes.

## 0.3.0 (2016-11-21)

* Added dialog to list, edit, and remove tags.
* Properties tab.
* Refactored code to remove almost all unsafe code.

## 0.2.1 (2016-09-17)

* Info provider: Use Mutex and fix Arc misuse that was possibly leading to segfault.

## 0.2.0 (2016-09-17)

* Ported to Rust. Previous [Python version](https://github.com/talklittle/tmsu-nautilus-python).

## 0.1.0 (2016-08-13)

* First source code release of TMSU Nautilus extension.
* Right-click on file to add tags.
* Display tags in file browser column.
