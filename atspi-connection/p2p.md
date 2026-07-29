# P2P introduction

Peer-to-Peer (P2P) support in [atspi](../atspi) enables direct connections to applications, bypassing the central accessibility bus when possible. This can significantly reduce method-call latency and improve performance, especially on systems with many accessible applications that support it.

We aim to integrate peer-to-peer communication in `atspi`'s API with the least friction for users, all behind a "p2p" feature.

The `Application` interface offers `GetApplicationBusAddress` which returns a bus address for direct communication.

## `AccessibilityConnection`

`AccessibilityConnection` discovers an application's direct address only when a P2P-aware lookup first targets that application. Ready connections are reused, concurrent lookups for the same unique owner share one attempt, and normal P2P unavailability transparently falls back to the connection's long-lived accessibility bus.

In practice, if you want to perform a method call on an `ObjectRef`, just get the `AccessibleProxy` for that object with `object_as_accessible`:

```rust
let obj_ap = a11y.object_as_accessible(obj).await?;
let name = obj_ap.name().await?;
```

## feature "p2p"

For `atspi` users who do not perform method calls or query properties, P2P is gated behind the "p2p" feature. Those users will need to opt out of default features.

The feature "p2p" is enabled by default. Initialization does not enumerate applications, request direct addresses, or open direct connections. It only starts a `NameOwnerChanged` listener used to invalidate cached identities; ownership signals never trigger discovery themselves.

If users opt out, no peer discovery state is kept and atspi does not start this listener.

## traits `P2P` and `Peer`

Ready peers are kept privately by `AccessibilityConnection`.

The `Peer` struct can be considered a handle to individual peers that do support P2P and allows:

- getting an `AccessibleProxy` for a given path
- getting an `AccessibleProxy` for the root object
- getting a `ProxyExt::Proxies` object to conveniently get any proxy that object supports

The P2P trait offers the higher level API and is implemented for `AccessibilityConnection` and allows:

- asynchronously discovering or getting a peer by bus name
- taking a detached, unique-name-sorted snapshot of currently ready peers
- getting an `AccessibleProxy` for the root object by bus name - may or may not support P2P
- getting an `AccessibleProxy` for any `ObjectRef` - may or may not support P2P

`get_peer` returns `Ok(Some(peer))` when a direct connection is ready. `Ok(None)` means the application definitively does not support P2P, a capability probe or connection attempt failed transiently, a transient failure is still in backoff, or ownership changed during discovery. Name-resolution and other failures that prevent correct shared-bus routing remain errors. Errors returned by `GetApplicationBusAddress` itself are optional-P2P failures and do not prevent shared-bus fallback.

Ready transport is canonical by unique owner and never stores a discovery-order alias. For compatibility, `Peer::well_known_name()` describes the current lookup: unique-name lookups and `peers()` snapshots return `None`, while a well-known lookup returns the alias requested by that call. Two aliases for one owner therefore share one connection while each lookup reports its own alias.

Invalid addresses and direct connection failures use per-owner retry delays of 1, 2, 4, 8, and 16 seconds, then 30 seconds. Unsupported applications remain cached until their owner changes. Negative state is bounded, so eviction can cause a later lookup to retry but never changes fallback correctness.

```rust,no_run
use atspi_connection::{AccessibilityConnection, P2P};
use zbus::names::BusName;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let connection = AccessibilityConnection::new().await?;
let name = BusName::try_from(":1.42")?;

if let Some(peer) = connection.get_peer(&name).await? {
    println!("direct peer: {}", peer.unique_name());
}

// This is an owned snapshot. Iterating or retaining it never locks discovery.
for peer in connection.peers() {
    println!("ready peer: {}", peer.unique_name());
}
# Ok(())
# }
```

## Migrating from eager peer storage

`get_peer` changed from `Option<Peer>` to `AtspiResult<Option<Peer>>` and now performs discovery. Add `.await?` before handling the option:

```text
connection.get_peer(&name)          // old
connection.get_peer(&name).await?   // new
```

`peers` changed from `Arc<Mutex<Vec<Peer>>>` to an owned `Vec<Peer>`. Remove locking and iterate the returned snapshot directly:

```text
connection.peers().lock().unwrap().iter() // old
connection.peers().iter()                 // new
```

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
