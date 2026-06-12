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
[smol](https://crates.io/crates/smol).

## Feature Flags

| Flag | Default | Description |
|------|:-------:|-------------|
| `proxies` | Yes | Re-exports the `atspi-proxies` crate, letting you actively communicate with (query) D-Bus. |
| `connection` | Yes | Re-exports the `atspi-connection` crate, providing convenient abstractions over D-Bus for *receiving*. Note that active queries still require `proxies`. |
| `p2p` | Yes | Extends `AccessibilityConnection` with peer-to-peer capabilities: direct, per-application connections that bypass the bus for *queries*. Events remain bus broadcasts. |
| `wrappers` | Yes | Enables the event wrapper enums (e.g. `Event`, `ObjectEvents`) and the conversions between the generic `Event` type and the user-facing event structs. |
| `tokio` | No | Enables support for the `tokio` runtime. Rather than starting its own runtime, zbus spawns its task on the host runtime. |
| `tracing` | No | Enables support for the `tracing` logger. |
| `x11-legacy` | No | Enables the deprecated X11-era interfaces `DeviceEventController` and `DeviceEventListener`. |

> **Note:** `atspi` is **not** compatible with the [`glommio`](https://crates.io/crates/glommio) runtime, as it uses its own types unrelated to how other runtimes work. (PRs welcome!)


## D-Bus type validation

Atspi is used to send and receive data to and from applications. Sender and recipient need to agree on the shape of the data type for fruitful communication. Our best bet is to keep our types in sync with the protocol descriptions.

We employ [zbus-lockstep](https://github.com/luukvanderduim/zbus-lockstep/) to match types against those defined in the AT-SPI2 protocol descriptions.

Not all types can be validated (easily) with zbus_lockstep because types may not exist
in the protocol descriptions, for example because they are deprecated (but still in use) or we have chosen a different representation.

[A (partial) review of type validation may be found here](type_validation.md)

## Contributing

This repository offers contributors hooks and a commit message template.
We kindly request contributors to set both up locally.

### git hooks
This repository offers basic pre-commit and pre-push scripts in the `.githooks` directory.
This command will configure git to use the hooks from the `.githooks` directory for this repository.

```sh
git config core.hooksPath .githooks
```
### git commit message template

The git commit message template helps contributors follow conventional commits for atspi.
This command will configure git to use the commit message template from the `.gitmessage` file for
this repository.

```sh
git config commit.template .gitmessage
```

## License

The `atspi` library is licensed as [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html) or [MIT](https://mit-license.org/).
