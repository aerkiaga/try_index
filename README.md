# try_index
A boilerplate library that provides traits `TryIndex` and `TryIndexMut`.

Unlike the ones in [try_traits](https://docs.rs/try-traits/latest/try_traits/),
this gives explicit implementations for all standard library collections,
which can actually fail. In exchange, no blanket implementation is provided.

```rust
use try_index::*;

let foo = vec![4, 3, 6, 2];
assert_eq!(foo.try_index(2), Some(&6));
assert_eq!(foo.try_index(4), None);
assert_eq!(foo.try_index(1..=2), Some(&[3, 6][..]));
```

This crate has a default `std` feature that may be disabled for `no_std` support,
as well as an `alloc` feature that enables `no_std` `alloc` container types.
See documentation for more information.

## License
Copyright 2026 Developers of the try_index project

try_index is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See files `LICENSE-APACHE` and `LICENSE-MIT`, and `COPYRIGHT` for details.
