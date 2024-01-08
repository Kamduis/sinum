//! This crate provides the `SiNum` type that represents numbers that can easily be represented by the International System of Units (SI, from French Système International).
//!
//! It is easy to crate a `SiNum` and have it represent a number:
//! ```
//! # use sinum::SiNum;
//! let num = SiNum::new( 9.9 );
//! assert_eq!( num.as_f64(), 9.9 );
//! assert_eq!( format!( "{}", num ), "9.9" );
//! ```
//!
//! Assigning the the prefix is also straightforward:
//! ```
//! # use sinum::{SiNum,Prefix};
//! let num_milli = SiNum::new( 9.9 ).with_prefix( Prefix::Milli );
//!
//! assert_eq!( num_milli.as_f64(), 0.0099 );
//! assert_eq!( num_milli.prefix(), Prefix::Milli );
//! assert_eq!( format!( "{}", num_milli ), "9.9 m" );
//! ```
//!
//! A `SiNum` prefix can be changed, without changing the value of the number it represents:
//! ```
//! # use sinum::{SiNum,Prefix};
//! let num = SiNum::new( 9.9 ).to_prefix( Prefix::Milli );
//!
//! assert_eq!( num.as_f64(), 9.9 );
//! assert_eq!( num.prefix(), Prefix::Milli );
//! assert_eq!( format!( "{}", num ), "9900 m" );
//! ```
//!
//! # Optional Features
//! * **tex** Enables returning `Prefix`es and `SiNum`s as strings usable directly by LaTeX (requires the `{siunitx}` LaTeX-package.





//=============================================================================
// Modules


mod prefix;
pub use crate::prefix::PrefixError;
pub use crate::prefix::Prefix;

mod number;
pub use crate::number::SiNum;

mod unit;
use crate::unit::Dimension;
pub use crate::unit::Unit;

mod quantity;
pub use crate::quantity::SiQty;




//=============================================================================
// Traits


/// Providing conversion into LaTeX code.
///
/// This Trait is only available, if the **`tex`** feature has been enabled.
#[cfg( feature = "tex" )]
pub trait Latex {
	/// Converts the entity into a LaTeX-string.
	fn to_latex( &self ) -> String;
}
