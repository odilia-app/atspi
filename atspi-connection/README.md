# `atspi-connection`

This crate uses the the following crates to support its various functions:

* `atspi-common`: to receive events from AT-SPI.
* `atspi-proxies`: to send events and query live information over AT-SPI.

This is essentially a client-facing library where you can use the `AccessibilityConnection` structure to get streams of AT-SPI events coming from a Linux system.
See the examples folder and documentation on how to use this library.

## Feature Flags

- `default`: `wrappers`
- `wrappers`: enable support for `atspi-common` wrapper types that categorize events by interface, as well as the all-encompassing `Event` enum that can store any event type.
    - This also enables the `event_stream` function that allows you to receive a stream of `Event`s instead of specific events.
- `tracing`: enable support for the `tracing` crate
