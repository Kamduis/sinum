//! Provides values using the SI prefix and unit system.




//=============================================================================
// Crates


use std::fmt;

use thiserror::Error;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum PrefixError {
	#[error( "There is no Prefix with exponent `{0}`" )]
	TryFromExp( i8 ),

	#[error( "There is no SI prefix for `{0}`" )]
	ExpInvalid( i32 ),
}




//=============================================================================
// Enums


/// Represents the different SI prefixes like kilo, milli, nano etc.
#[derive( Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug )]
pub enum Prefix {
	Femto,
	Pico,
	Nano,
	Micro,
	Milli,
	Centi,
	Deci,
	Nothing,
	Kilo,
	Mega,
	Giga,
	Tera,
	Peta,
}

impl Prefix {
	/// Larges exponent representable by `Self`.
	pub const MAX_EXP: i8 = 15;

	/// Smalles exponent representable by `Self`.
	pub const MIN_EXP: i8 = -15;

	/// Return the factor represented by this prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::Prefix;
	/// assert_eq!( Prefix::Peta.as_f64(), 1e15f64 );
	/// assert_eq!( Prefix::Femto.as_f64(), 1e-15f64 );
	/// ```
	pub fn as_f64( &self ) -> f64 {
		match self {
			Self::Femto => 1e-15,
			Self::Pico =>  1e-12,
			Self::Nano =>  1e-9,
			Self::Micro => 1e-6,
			Self::Milli => 1e-3,
			Self::Centi => 1e-2,
			Self::Deci =>  1e-1,
			Self::Nothing => 1.0,
			Self::Kilo =>  1e3,
			Self::Mega =>  1e6,
			Self::Giga =>  1e9,
			Self::Tera =>  1e12,
			Self::Peta =>  1e15,
		}
	}

	/// Returns the exponent representing this prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::Prefix;
	/// assert_eq!( Prefix::Peta.exp(), 15i8 );
	/// assert_eq!( Prefix::Femto.exp(), -15i8 );
	/// ```
	pub fn exp( &self ) -> i8 {
		match self {
			Self::Femto =>  -15,
			Self::Pico =>   -12,
			Self::Nano =>    -9,
			Self::Micro =>   -6,
			Self::Milli =>   -3,
			Self::Centi =>   -2,
			Self::Deci =>    -1,
			Self::Nothing =>  0,
			Self::Kilo =>     3,
			Self::Mega =>     6,
			Self::Giga =>     9,
			Self::Tera =>    12,
			Self::Peta =>    15,
		}
	}
}

impl TryFrom<i8> for Prefix {
	type Error = PrefixError;

	/// Returns a `Prefix` with the exponent of `item`.
	///
	/// If there is no Prefix representing this exponent a `PrefixError` will be returned.
	///
	/// # Example
	/// ```
	/// # use sinum::Prefix;
	/// assert_eq!( Prefix::try_from( -3 ).unwrap(), Prefix::Milli );
	/// assert_eq!( Prefix::try_from( -2 ).unwrap(), Prefix::Centi );
	/// assert_eq!( Prefix::try_from( 0 ).unwrap(), Prefix::Nothing );
	/// assert_eq!( Prefix::try_from( 3 ).unwrap(), Prefix::Kilo );
	/// assert_eq!( Prefix::try_from( 15 ).unwrap(), Prefix::Peta );
	/// ```
	fn try_from( item: i8 ) -> Result<Self, Self::Error> {
		let res = match item {
			-15 => Self::Femto,
			-12 => Self::Pico,
			-9  => Self::Nano,
			-6  => Self::Micro,
			-3  => Self::Milli,
			-2  => Self::Centi,
			-1  => Self::Deci,
			0   => Self::Nothing,
			3   => Self::Kilo,
			6   => Self::Mega,
			9   => Self::Giga,
			12  => Self::Tera,
			15  => Self::Peta,
			_ => return Err( PrefixError::TryFromExp( item ) ),
		};

		Ok( res )
	}
}

impl fmt::Display for Prefix {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			Self::Femto =>   write!( f, "f" ),
			Self::Pico =>    write!( f, "p" ),
			Self::Nano =>    write!( f, "n" ),
			Self::Micro =>   write!( f, "µ" ),
			Self::Milli =>   write!( f, "m" ),
			Self::Centi =>   write!( f, "c" ),
			Self::Deci =>    write!( f, "d" ),
			Self::Nothing => write!( f, "" ),
			Self::Kilo =>    write!( f, "k" ),
			Self::Mega =>    write!( f, "M" ),
			Self::Giga =>    write!( f, "G" ),
			Self::Tera =>    write!( f, "T" ),
			Self::Peta =>    write!( f, "P" ),
		}
	}
}




//=============================================================================
// Structs


/// Represents a number in combination with a SI prefix.
#[derive( Clone, Copy, PartialOrd, PartialEq, Debug )]
pub struct SiNum {
	mantissa: f64,
	prefix: Prefix
}

impl SiNum {
	/// Create a new `SiNum` representing the numeric value `num` without any prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::SiNum;
	/// assert_eq!( SiNum::new( 9999.9 ).as_f64(), 9999.9 );
	/// assert_eq!( SiNum::new( 99999.9 ).as_f64(), 99999.9 );
	/// ```
	pub fn new( num: f64 ) -> Self {
		Self {
			mantissa: num,
			prefix: Prefix::Nothing,
		}
	}

	/// Creates a new `SiNum` from `self` and applying `prefix`.
	///
	/// *Note:* The numeric value of the new `SiNum` will be different from `self` (aside from using the same `Prefix`) since the mantissa is staying the same while the `Prefix` is modified.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, Prefix};
	/// assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).as_f64(), 9_999_900_000.0 );
	/// assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Milli ).as_f64(), 9.9999 );
	/// ```
	pub fn with_prefix( self, prefix: Prefix ) -> Self {
		Self {
			mantissa: self.mantissa,
			prefix,
		}
	}

	/// Creates a new `SiNum` from `self` at the specified `prefix`.
	///
	/// The numeric value of the new `SiNum` will be identical to `self` (apart from possible floating point rounding errors) since the mantissa is being modified alongside the prefix to reflect the same numeric value as before.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, Prefix};
	/// let num = SiNum::new( 9999.9 );
	///
	/// assert_eq!( num.as_f64(), 9_999.9 );
	/// assert_eq!( num.to_prefix( Prefix::Milli ).as_f64(), 9999.9 );
	/// assert_eq!( num.to_prefix( Prefix::Milli ).mantissa(), 9_999_900.0 );
	///
	/// assert_eq!( num.to_prefix( Prefix::Kilo ).as_f64(), 9999.9 );
	/// assert_eq!( num.to_prefix( Prefix::Kilo ).mantissa(), 9.9999 );
	/// ```
	pub fn to_prefix( self, prefix: Prefix ) -> Self {
		let factor = self.prefix.as_f64() / prefix.as_f64();
		Self {
			mantissa: self.mantissa * factor,
			prefix,
		}
	}

	/// Creates a new `SiNum` from `self` with a reduced numbers of digits of the mantissa (see `mantissa()`) required to represent the number:
	///
	/// * No more than 3 digits in front of the decimal point.
	/// 	(1234 → 1.234 k)
	///
	/// * No zero in front of the decimal point.
	/// 	(0.001 → 1.0 m)
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, Prefix};
	/// assert_eq!(
	/// 	SiNum::new( 1000.0 ).shorten().unwrap(),
	/// 	SiNum::new( 1.0 ).with_prefix( Prefix::Kilo )
	/// );
	/// assert_eq!(
	/// 	SiNum::new( 0.001 ).shorten().unwrap(),
	/// 	SiNum::new( 1.0 ).with_prefix( Prefix::Milli )
	/// );
	/// assert_eq!(
	/// 	SiNum::new( 1234.5 ).shorten().unwrap(),
	/// 	SiNum::new( 1.2345 ).with_prefix( Prefix::Kilo )
	/// );
	/// ```
	pub fn shorten( self ) -> Result<Self, PrefixError> {
		let exps = self.mantissa.log10().floor().div_euclid( 3.0 ) * 3.0;

		if exps > Prefix::MAX_EXP as f64 {
			return Err( PrefixError::ExpInvalid( exps as i32 ) );
		}

		let exp_new = self.prefix.exp() + exps as i8;
		let prefix_new = Prefix::try_from( exp_new )?;

		Ok( self.to_prefix( prefix_new ) )
	}

	/// Returns the mantissa of the `SiNum`. The Mantissa is the number displayed before the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, Prefix};
	/// let num = SiNum::new( 9999.9 );
	///
	/// assert_eq!( num.mantissa(), 9999.9 );
	/// assert_eq!( num.with_prefix( Prefix::Mega ).mantissa(), 9999.9 );
	/// assert_eq!( num.with_prefix( Prefix::Milli ).mantissa(), 9999.9 );
	/// ```
	pub fn mantissa( &self ) -> f64 {
		self.mantissa
	}

	/// Returns the `Prefix` of the `SiNum`.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, Prefix};
	/// let num = SiNum::new( 9999.9 ).with_prefix( Prefix::Mega );
	/// assert_eq!( num.prefix(), Prefix::Mega );
	/// ```
	pub fn prefix( &self ) -> Prefix {
		self.prefix
	}

	/// Returns the numeric value of the `SiNum` without any prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::SiNum;
	/// assert_eq!( SiNum::new( 9999.9 ).as_f64(), 9999.9 );
	/// assert_eq!( SiNum::new( 99999.9 ).as_f64(), 99999.9 );
	/// ```
	pub fn as_f64( &self ) -> f64 {
		self.mantissa * self.prefix.as_f64()
	}
}

impl From<f64> for SiNum {
	/// Creates a new `SiNum` from `item`. This is identical to `SiNum::new()`.
	///
	/// # Example
	/// ```
	/// # use sinum::SiNum;
	/// assert_eq!( SiNum::from( 9999.9 ), SiNum::new( 9999.9 ) );
	/// assert_eq!( SiNum::from( 99999.9 ), SiNum::new( 99999.9 ) );
	/// ```
	fn from( item: f64 ) -> Self {
		Self {
			mantissa: item,
			prefix: Prefix::Nothing,
		}
	}
}

impl fmt::Display for SiNum {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self.prefix {
			Prefix::Nothing => write!( f, "{}", self.mantissa ),
			_ => write!( f, "{} {}", self.mantissa, self.prefix )
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn print_prefix() {
		assert_eq!( Prefix::Peta.to_string(), "P".to_string() );
		assert_eq!( Prefix::Femto.to_string(), "f".to_string() );
	}

	#[test]
	fn sinum_string() {
		assert_eq!( SiNum::new( 9999.9 ).to_string(), "9999.9".to_string() );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).to_string(), "9999.9 M".to_string() );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Milli ).to_string(), "9999.9 m".to_string() );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).to_prefix( Prefix::Milli ).to_string(), "9999900000000 m".to_string() );
	}
}
