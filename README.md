# `EffectCell`

A container that runs an effect every time its data is mutated.
The effect is run with the old data.

You can mutate an `EffectCell` using the `update` method or the ~fancy~
`cell <<= val` syntax.

`EffectCell` does is also `#![no_std]`!

```rust
use effect_cell::EffectCell;

fn main() {
    let mut counter = 0;
    let mut printer = EffectCell::new(0, |_| counter += 1);
    printer <<= 2; // counter increments
    printer <<= 4; // counter increments
    assert_eq!(counter, 2)
}
```

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](https://github.com/fprasx/peapod/blob/main/LICENSE-APACHE)
    or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](https://github.com/fprasx/peapod/blob/main/LICENSE-MIT) or
    http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
