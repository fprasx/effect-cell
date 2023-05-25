# `EffectCell`

A container that runs an effect every time its data is mutated.
In essence, a slimed down implementation of the Observer pattern using Rust's `Fn` trait.

```rust
use effect_cell::EffectCell;

fn main() {
    let mut effect_cell = EffectCell::new(0);
    effect_cell.bind(|data| {println!("{data}");});
    effect_cell.update(1);
    // Prints "1"
}
```

## Operator Passthrough

The `XAssign` traits have been setup so that they can modify the internal data
without the need for a call through `update_lambda`.
They will always call effects.

```rust
use effect_cell::EffectCell;

fn main() {
    let mut effect_cell = EffectCell::new(0);
    effect_cell.bind(|data| {println!("{data}");});
    effect_cell += 1;
    // Prints "1"
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](https://github.com/fprasx/peapod/blob/main/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](https://github.com/fprasx/peapod/blob/main/LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
