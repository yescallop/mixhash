# mixhash

A non-cryptographic hash function that mixes the data only.

`mixhash` is generally faster than SipHash for trivial types like integers. That said,
it is unclear what use case `mixhash` is more suited for. Do keep in mind that:

> If you want it, benchmark it.

Reference: <http://zimbry.blogspot.com/2011/09/better-bit-mixing-improving-on.html>

## Examples

```rust
use mixhash::Mix;
use std::collections::HashSet;

let set: HashSet<u32, _> = HashSet::with_hasher(Mix);
```
