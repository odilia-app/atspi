# Type validation

For successful communication it is important that data are
understood equally on both ends.

We make a deliberate effort to validate our types in `atspi`.
We leverage [zbus-lockstep](https://github.com/luukvanderduim/zbus-lockstep/) to match types in our crate against those defined in the AT-SPI2 protocol descriptions.

`zbus-lockstep` is used to check if a type's (D-Bus type-system) signature corresponds to its counterpart in the protocol's XML.

## Validation table

| `atspi` type| XML validation | Notes |
|:--|---|---|
| `ObjectRef`| ✓ | Formerly: `Accessible`, used in `Available` and `RemoveAccessible`|
| `CacheItem`| ✓ | See: `AddAccessible` signal of the `Cache` interface.|
| `LegacyCacheItem` | ⚠ | The signature does not appear in current protocol: test hard coded.|
| `Interface` | ⚠ | Unlike the protocol, which uses strings, we store `Interface` as bitflags.|
| `InterfaceSet` | ⚠ | We store `InterfaceSet` as a single `u64`, not as 'array of strings'.|
| `State` | ⚠ | XML: "au", a list of enum variants. Whereas `atspi` stores a `u64` bitflag.|
| `StateSet` | ⚠ | See `State`.|
| `EventBodyOwned` | ✓ | Covers many other events.|
| `EventBodyQt` | ⚠ | Signature does not appear in XML.|
| `EventListeners` | ✓ | Represents signals of `EventListenerRegistered` and `EventListenerDeregistered`|
| `SortOrder` | ✓ | 'sortby' argument pf member `GetMatches`|
| `TreeTraversalType` | ✓ | 'tree' argument of member `GetMatchesTo`|
| `MatchType` | ✓ | Found at index 3, in the 'rule' argument in `GetMatches`|
| `CoordType` | ✓ | Argument of `GetImagePosition` of `org.a11y.atspi.Image`|
| `ClipType` | ✓ | `Text::GetTextAtOffset` argument `type`|
| `Granularity` | ✓ | `Text::GetStringAtOffset` argument `granularity`|
| `Layer` | ✓ | `Component::GetLayer` return type.|
| `ScrollType` | ✓ | `Component::ScrollTo` argument `type`|
| `Politeness` | ✓ | `Event.Object::Announcement` argument `politeness`|
| `ObjectMatchRule` | ⚠ | `Collection::GetMatches` argument 'rule' contains `Vec<Role>` and `StateSet` which are defined as "ai", but `Role` and `StateSet` are not.|
