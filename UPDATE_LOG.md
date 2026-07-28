# Update Log

* `0.10.0`
  * Add new traits matching the traits used to generate `zbus::Proxy{,Blocking}` structs.
  * Add new auto-implementations of the new traits for all `*Proxy` and `*ProxyBlocking` types.
  * Make the `Convertible` trait generic.
  * Make all `*Ext` traits generic.
    * NOTE: No implementations are provided for `AccessibleExtBlocking`.
    * Provide implementations of `*Ext` based on implementing (usually) `*` and `*ExtError` traits.
  * All of this means that the implementations are now generic over any type which implements the appropriate traits.
