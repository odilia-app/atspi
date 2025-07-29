# P2P introduction

Peer-to-Peer (P2P) support in [atspi](../atspi) enables direct connections to applications, bypassing the central accessibility bus when possible. This can significantly reduce method-call latency and improve performance, especially on systems with many accessible applications that support it.

We aim to integrate peer-to-peer communication in `atspi`'s API with the least friction for users, all behind a "p2p" feature.

The `Application` interface offers `GetApplicationBusAddress` which returns a bus address for direct communication.

## `AccessibilityConnection`

To avoid querying each object whether their bus name supports P2P, `AccessibilityConnection` keeps a list of all accessible applications on the bus that support P2P and continuously updates it.
`AccessibilityConnection` is always in scope when performing operations on the accessibility bus, therefore it is the perfect place to keep the list of peers.

In practice, if you want to perform a method call on an `ObjectRef`, just get the `AccessibleProxy` for that object with `object_as_accessible`:

```rust
let obj_ap = a11y.object_as_accessible(obj).await?;
let name = obj_ap.name().await?;
```

## feature "p2p"

For `atspi` users who do not perform method calls or query properties, P2P is gated behind the "p2p" feature. Those users will need to opt out of default features.

The feature "p2p" is enabled by default and if enabled, the `AccessibilityConnection` gains a list of applications that support P2P communication on initialization.
Initialization of an `AccessibilityConnection` will also spawn a task to continuously listen for new applications entering or leaving the bus. It does so by listening for the `NameOwnerChanged` event emitted by the D-Bus daemon.

If users opt out, no list of `Peer`s will be kept and atspi will not listen for updates.

## traits `P2P` and `Peer`

As stated before, a list of `Peer`s is kept by the `AccessibilityConnection`.

The `Peer` struct can be considered a handle to individual peers that do support P2P and allows:

- getting an `AccessibleProxy` for a given path
- getting an `AccessibleProxy` for the root object
- getting a `ProxyExt::Proxies` object to conveniently get any proxy that object supports

The P2P trait offers the higher level API and is implemented for `AccessibilityConnection` and allows:

- getting a peer by bus name
- getting an `AccessibleProxy` for the root object by bus name - may or may not support P2P
- getting an `AccessibleProxy` for any `ObjectRef` - may or may not support P2P

## `p2p_tree` example

You are invited to try and run the following. This assumes your PWD is the repository.

```shell
cargo run --release --example p2p_tree
```

The example walks all nodes in the accessibility tree and queries each node for its role.
The example registers the time it took to construct the tree.

The `p2p_tree` example constructs the tree using three methods:

1. **Bus:** Queries all nodes of all applications over the accessibility bus (baseline).
2. **P2P Sequential:** Queries each application's nodes over a P2P connection (if available), one after another.
3. **P2P Parallel:** Distributes queries to all applications (over P2P connections if available), parallel in the sense that queried applications get to work in parallel.

```shell
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/examples/p2p_tree`
Here's that data presented as a nicely aligned Markdown table:

| D-Bus operation                | Node count | Time (ms) |
| :----------------------------- | :--------- | :-------- |
| Building tree (bus)            | 10445      | 2144.92   |
| Building tree (P2P)            | 10446      | 1077.20   |
| Building tree (P2P "parallel") | 10446      | 847.70    |
| ---                            | ---        | ---       |
| Speedup (p2p vs bus)           | 1.99x      |           |
| Speedup (p2p-par vs bus)       | 2.53x      |           |

```

The results depend on the ratio of applications that do and do not support P2P on the bus.
Performance also depends on your hardware.
The above ran on an Intel Core Ultra 155H.
