# chain-map

The `ChainMap` type groups a chain of `HashMap`s together in precedence
order and provides a single, unified view into the values. The semantics
for keys are the same as for a `HashMap`, however the value associated
with a given key is the value of that key in the highest-precedence map
that contains the key.

### Rust Version

This version of chain-map requires Rust 1.31 or later.

## Precedence

Maps added to the `ChainMap` earlier have precedence over those added
later. So the first map added to the chain will have the highest
precedence, while the most recent map added will have the lowest.

## Performance

Each read of the `ChainMap` will read the chain of maps in order, so each
operation will complete in worst-case O(N), with `N` the number of maps in
the chain. As a result, this should only be used for cases where the number
of reads is low compared to the number of elements in each map.

## Examples

```rust
use std::collections::HashMap;
use chain_map::ChainMap;

let mut first_map = HashMap::new();
first_map.insert("first", 10);

let mut second_map = HashMap::new();
second_map.insert("first", 20);
second_map.insert("second", 20);

let mut third_map = HashMap::new();
third_map.insert("first", 30);
third_map.insert("second", 30);
third_map.insert("third", 30);

let mut chain: ChainMap<_, _> =
    vec![first_map, second_map, third_map].into_iter().collect();
assert_eq!(chain.get("first"), Some(&10));
assert_eq!(chain["second"], 20);
assert!(chain.contains_key("third"));
assert!(!chain.contains_key("fourth"));
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
