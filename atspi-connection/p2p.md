# P2P introduction

Peer-to-Peer (P2P) support in [atspi](../atspi) enables direct connections to applications, bypassing the central accessibility bus when possible. This can significantly reduce method-call latency and improve performance, especially on systems with many accessible applications.

## `p2p_tree` example

```shell
cargo run --release --example p2p_tree
```

The example walks all nodes in the accessibility tree and queries each node for its role. The example registers the amount of time the operation took.

It does so in three ways:
The `p2p_tree` example demonstrates three ways to traverse the accessibility tree and measure performance:

1. **Bus:** Queries all nodes of all applications over the accessibility bus.
2. **P2P Sequential:** Queries each application's nodes over a P2P connection (if available), one after another.
3. **P2P Parallel:** Queries each application's nodes over P2P connections (if available) concurrently, leveraging parallelism.

Because P2P connections are all separate connections, this modality lends itself well for requests to be sent concurrently with `futures::future::FutureUnordered`.

| D-Bus operation              | Node count | Time (ms) |
|------------------------------|------------|-----------|
| Building tree (bus)          | 6386       | 1291.77   |
| Building tree (P2P)          | 6386       | 688.18    |
| Building tree (P2P parallel) | 6386       | 554.23    |
|------------------------------|------------|-----------|
| Speedup (p2p vs bus)         |            | 1.88x     |
| Speedup (p2p-par vs bus)     |            | 2.33x     |

* Tested on Intel Core Ultra 7 155H*

The actual latency improvement depends on your hardware. Faster, more IO-bound systems tend to benefit more from P2P, as they spend less time waiting for the bus and more time processing requests in parallel.

## Design

When the `"p2p"` feature (enabled by default) is active, P2P connections are cached up front and updated continuously by listening for `NameOwnerChanged` events. Each bus name is queried for P2P support once.

The `AccessibilityConnection` type is extended via the `P2P` trait, which provides:

* `object_as_accessible`: Returns an `AccessibleProxy` for a given `ObjectRef`.
* `bus_name_as_root_accessible`: Returns a root object `AccessibleProxy` for a given `BusName`.
* `find_peer`: Returns the `Peer` corresponding to a `BusName`.
