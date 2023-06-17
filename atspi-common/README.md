# `atspi-common`

Common data structures for the `atspi` crate.
This crate is meant to only contain the absolute lowest common denominator for libraries to talk to each other about `atspi` information;
therefore, it attempts to be able to compile on basically any architecture (including WASM).

Please use the internal documentation to learn how to use these data structures.

## Compiling for WASM

If you'd like to compile this crate for any architecture that implements `std`, then you have to compile it without any feature flags.
By default, this crate provides a variety of converison methods for foreign error types into its own event type.

## Feature Flags

* `async-std`: build `zbus` with `async-std` support.
* `tokio`: build `zbus` with `tokio` support.

Either of these flags will enable conversion from zbus error types to the `AtspiError` type.

> Why do I need to specify a runtime to have the conversion methods available to me?

A: Because `zbus` requires you to specify at least one runtime, and it would be annoying to compile an entire second runtime if it is not necessary.
