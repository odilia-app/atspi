# AT-SPI for Rust

[![crates.io badge](https://img.shields.io/crates/v/atspi)](https://crates.io/crates/atspi)
[![docs.rs badge](https://docs.rs/atspi/badge.svg)](https://docs.rs/atspi)
[![CI badge](https://github.com/odilia-app/atspi/actions/workflows/ci.yml/badge.svg)](https://github.com/odilia-app/atspi/actions/workflows/ci.yml)
[![Code coverage badge](https://codecov.io/gh/odilia-app/atspi/branch/main/graph/badge.svg?token=MQ1BBEZ3UC)](https://codecov.io/gh/odilia-app/atspi)

Higher level, asynchronous, pure Rust [AT-SPI2](https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/) protocol implementation using
[zbus](https://crates.io/crates/zbus).

Part of the [Odilia screen reader project](https://odilia.app).

## Design

* Fully documented, with `#[deny(missing_docs)]`
	* Or at least, it will be by 1.0
* Fully safe, with `#[deny(unsafe_code)]`
* Fantastic code style with `#[deny(clippy:all, clippy::pedantic, clippy::cargo)]`

This crate makes use of the
[zbus crate](https://crates.io/crates/zbus) for
[dbus communication](https://www.freedesktop.org/wiki/Software/dbus/).
We use the asynchronous zbus API, so to use atspi, you will need to run an async executer like
[tokio](https://crates.io/crates/tokio) or
[async-std](https://crates.io/crates/async-std).
The `async-io` and `tokio` features are exposed and will be passed through to zbus.

## License

The `atspi` library is licensed as [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html) or [MIT](https://mit-license.org/).
