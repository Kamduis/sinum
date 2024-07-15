<div align="center" class="rustdoc-hidden">
# sinum
</div>

This crate provides the `Num` and `Qty` types that represent numbers and quantities[^1] that can easily be represented by the [International System of Units][] (SI, from French Système International).


## Usage

To use this crate your `Cargo.toml` could look like this (replace `x`, `y` and `z` with the latest version number):

```toml
[dependencies]
sinum = "x.y.z"
```


## Example

### Numbers

A [`Num`] represents a number:
```rust
use sinum::Num;

let num = Num::new( 9.9 );
assert_eq!( num.as_f64(), 9.9 );
assert_eq!( num.to_string(), "9.9" );
```

Assigning the the prefix is straightforward:
```rust
use sinum::{Num,Prefix};

let num_milli = Num::new( 9.9 ).with_prefix( Prefix::Milli );

assert_eq!( num_milli.as_f64(), 0.0099 );
assert_eq!( num_milli.prefix(), Prefix::Milli );
assert_eq!( num_milli.to_string(), "9.9 m" );
```

A [`Num`] prefix can be changed, without changing the value of the number it represents:
```rust
use sinum::{Num,Prefix};

let num = Num::new( 9.9 ).to_prefix( Prefix::Milli );

assert_eq!( num.as_f64(), 9.9 );
assert_eq!( num.prefix(), Prefix::Milli );
assert_eq!( num.to_string(), "9900 m" );
```


### Quantities

To represent quantities the [`Qty`] type is provided. This type is handling the numeric value (including the prefix) with an internal [`Num`] type while being aware of a unit.
```rust
use sinum::{Qty, Unit};

let qty = Qty::new( 9.9.into(), &Unit::Meter );
assert_eq!( qty.as_f64(), 9.9 );
assert_eq!( qty.to_string(), "9.9 m" );
assert_eq!( qty.unit(), &Unit::Meter );
```

A prefix can be added by applying it directly to the [`Num`] part while creating it.
```rust
use sinum::{Num,Qty,Prefix, Unit};

let qty_milli = Qty::new( Num::from( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Meter );

assert_eq!( qty_milli.as_f64(), 0.0099 );
assert_eq!( qty_milli.to_string(), "9.9 mm" );
```

As with [`Num`] the prefix of a [`Qty`] can be changed, without changing the value of the number it represents:
```rust
use sinum::{Num, Qty, Prefix, Unit};

let num = Num::new( 9.9 );
let qty = Qty::new( 9.9.into(), &Unit::Ampere );

assert_eq!( qty.to_string(), "9.9 A" );
assert_eq!( qty.as_f64(), 9.9 );

let qty_milli = qty.to_prefix( Prefix::Milli );

assert_eq!( qty_milli.to_string(), "9900 mA" );
assert_eq!( qty_milli.as_f64(), 9.9 );
```


## Optional Features

* **serde** Enables [`serde`][serde] support.
* **tex** Enables returning [`Prefix`]es and [`Num`]s as strings usable directly by LaTeX (to be used with the `{siunitx}` LaTeX-package).



## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE][] or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT][] or <http://opensource.org/licenses/MIT>)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[^1]: Numbers with a unit of measurement.


[International System of Units]: https://www.bipm.org/documents/20126/41483022/SI-Brochure-9-EN.pdf
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
