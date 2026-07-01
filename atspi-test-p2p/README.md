# test-atspi-p2p

The P2P feature of [`atspi`](<http://github.com/odilia-app/atspi.git>) spawns a task that listens for [`NameOwnerChanged`](<https://dbus.freedesktop.org/doc/dbus-java/api/index.html?org/freedesktop/DBus.NameOwnerChanged.html>) events.

If feature 'p2p' is selected, this `DBus` signal is used to update the `Peers` list on the `AccessibilityConnection`.
This list of available P2P-capable, accessible applications is kept to provide users a P2P connection / proxy of the  
application when they need it.

## What it does

1. Subscribes to tracing messages
2. Shows the applications in the `Peers` list
3. Launches a P2P capable application and verifies its appearance in the `Peers` list,
   indicating the `NamoOwnerChanged` signal was received and handled as intended.
4. Terminates the application and verifies its removal from the `Peers` list,
   indicating the `NamoOwnerChanged` signal was received and handled as intended.

## Limitations

Currently it does not verify the entry of a `WellKnownName` or the transfer of ownership of a `WellKnownName`.

To verify this behavior, we need an example of such an application.
Suggestions are welcome.

## Example output

```shell
/test-atspi-p2p (main)> cargo run
    Finished `dev` profile [unoptimized] target(s) in 0.03s
     Running `target/debug/test-atspi-p2p`
2025-07-18T15:10:45.534154Z  INFO main ThreadId(01) CI(p2p): Set session accessibility to true
2025-07-18T15:10:45.538490Z  INFO main ThreadId(01) CI(p2p): Create accessibility connection
2025-07-18T15:10:45.540512Z  INFO main ThreadId(01) new: Connecting to a11y bus
2025-07-18T15:10:45.541409Z  INFO main ThreadId(01) new: Connected to a11y bus name=":1.39"
2025-07-18T15:10:45.568757Z  INFO main ThreadId(01) CI(p2p): Launching child process "mate-calc"
2025-07-18T15:10:45.688332Z  INFO tokio-runtime-worker ThreadId(19) Inserted unique name: :1.40 into the peer list.
2025-07-18T15:10:46.606777Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 24)
Peer: ":1.29", human readable name: "code-insiders"
Peer: ":1.32", human readable name: "element-desktop"
Peer: ":1.40", human readable name: "mate-calc"

2025-07-18T15:10:46.606816Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer insertion assertion passed
2025-07-18T15:10:47.608781Z  INFO                 main ThreadId(01) CI(p2p): Terminating "mate-calc"
2025-07-18T15:10:47.613761Z  INFO tokio-runtime-worker ThreadId(19) Peer with unique name: :1.40 left the bus - removed from peer list.
2025-07-18T15:10:48.627097Z  INFO                 main ThreadId(01) CI(p2p): Printing peers...
Peer: ... (total: 23)
Peer: ":1.28", human readable name: "Firefox"
Peer: ":1.29", human readable name: "code-insiders"
Peer: ":1.32", human readable name: "element-desktop"

2025-07-18T15:10:48.627137Z  INFO                 main ThreadId(01) CI(p2p): ✅ Peer removal assertion passed
2025-07-18T15:10:48.627141Z  INFO                 main ThreadId(01) CI(p2p): ✅ All assertions passed, exiting

```
