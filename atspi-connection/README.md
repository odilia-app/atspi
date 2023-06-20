# `atspi-connection`

This crate uses the the following crates to support its various functions:

* `atspi-common`: to receive events from AT-SPI.
* `atspi-proxies`: to send events and query live information over AT-SPI.

This is essentially a client-facing library where you can use the `AccessibilityConnection` structure to get streams of AT-SPI events coming from a Linux system.
See the examples folder and documentation on how to use this library.
