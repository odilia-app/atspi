# `atspi-connection`

This crate uses the the following crates to support its various functions:

* `atspi-common`: to receive events from AT-SPI.
* `atspi-proxies`: to send events and query live information over AT-SPI.

This is essentially a client-facing library where you can use the `AccessibilityConnection` structure to get streams of AT-SPI events coming from a Linux system.
See the examples folder and documentation on how to use this library.

## Feature Flags

* `default`: `wrappers`, `p2p`
* `p2p` : dependencies `async-executor",`dep:async-lock` and enables `zbus/p2p`
* `tracing`: enable support for the `tracing` crate
* `wrappers`: enable support for `atspi-common` wrapper types that categorize events by interface, as well as the all-encompassing `Event` enum that can store any event type.
  * This also enables the `event_stream` function that allows you to receive a stream of `Event`s instead of specific events.

## P2P

Peer-to-Peer (P2P) support in atspi enables direct connections to applications, bypassing the central accessibility bus when possible.

See: [introduction to `atspi` p2p](p2p.md)
