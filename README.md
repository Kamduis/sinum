# sinum

This Rust crate provides the `SiNum` type that represents numbers that can easily be represented by the International System of Units (SI, from French Système International).


## Example

To use this crate your `Cargo.toml` could look like this:

```toml
[dependencies]
sinum = "0.1.0"
```

Create a prefix-aware number:

```rust,no_run
use sinum::{SiNum, Prefix};

let num = SiNum::new( 9.9 ).with_prefix( Prefix::Milli );

assert_eq!( num.as_f64(), 0.0099 );
assert_eq!( num.prefix(), Prefix::Milli );
assert_eq!( format!( "{}", num ), "9.9 m" );
```

A `SiNum` prefix can be changed, without changing the value of the number it represents:
```
use sinum::{SiNum,Prefix};

let num = SiNum::new( 9.9 ).to_prefix( Prefix::Milli );

assert_eq!( num.as_f64(), 9.9 );
assert_eq!( num.prefix(), Prefix::Milli );
assert_eq!( format!( "{}", num ), "9900 m" );
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
