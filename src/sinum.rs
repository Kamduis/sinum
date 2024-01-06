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

	/// Returns a `Prefix` from the exponent `item`.
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
	/// Create a new `SiNum` with abosulte value `num`.
	pub fn new( num: f64 ) -> Self {
		Self {
			mantissa: num,
			prefix: Prefix::Nothing,
		}
	}

	/// Creates a new `SiNum` from `self` and applying `prefix`. This will change the numeric value of the number from `self` since the mantissa is staying the same while the prefix is modified.
	pub fn with_prefix( self, prefix: Prefix ) -> Self {
		Self {
			mantissa: self.mantissa,
			prefix,
		}
	}

	/// Creates a new `SiNum` from `self` at the specified `prefix`. This will *not* change the numeric value of the number from `self` since the mantissa is being modified alongside the prefix to reflect the same numeric value as before.
	pub fn to_prefix( self, prefix: Prefix ) -> Self {
		let factor = self.prefix.as_f64() / prefix.as_f64();
		Self {
			mantissa: self.mantissa * factor,
			prefix,
		}
	}

	/// Creates a new `SiNum` from `self` with less than 4 digits in front of the decimal point or less than 3 zeros after the decimal point if the digit in front of the decimal point is also 0.
	///
	/// * 1234 → 1.234 k
	/// * 0.001 → 1.0 m
	pub fn shorten( self ) -> Result<Self, PrefixError> {
		let exps = self.mantissa.log10().floor().div_euclid( 3.0 ) * 3.0;

		if exps > Prefix::MAX_EXP as f64 {
			return Err( PrefixError::ExpInvalid( exps as i32 ) );
		}

		let exp_new = self.prefix.exp() + exps as i8;
		let prefix_new = Prefix::try_from( exp_new )?;

		Ok( self.to_prefix( prefix_new ) )
	}

	/// Returns the mantissa of the `SiNum`.
	pub fn mantissa( &self ) -> f64 {
		self.mantissa
	}

	/// Returns the prefix of the `SiNum`.
	pub fn prefix( &self ) -> Prefix {
		self.prefix
	}

	/// Returns the numeric value of the `SiNum` without any prefix.
	pub fn as_f64( &self ) -> f64 {
		self.mantissa * self.prefix.as_f64()
	}
}

impl From<f64> for SiNum {
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
	fn prefix_factor() {
		assert_eq!( Prefix::Peta.as_f64(), 1e15f64 );
		assert_eq!( Prefix::Femto.as_f64(), 1e-15f64 );
	}

	#[test]
	fn prefix_exp() {
		assert_eq!( Prefix::Peta.exp(), 15i8 );
		assert_eq!( Prefix::Femto.exp(), -15i8 );
	}

	#[test]
	fn from_integer() {
		assert_eq!( Prefix::try_from( -3 ).unwrap(), Prefix::Milli );
		assert_eq!( Prefix::try_from( -2 ).unwrap(), Prefix::Centi );
		assert_eq!( Prefix::try_from( 0 ).unwrap(), Prefix::Nothing );
		assert_eq!( Prefix::try_from( 3 ).unwrap(), Prefix::Kilo );
		assert_eq!( Prefix::try_from( 15 ).unwrap(), Prefix::Peta );
	}

	#[test]
	fn print_prefix() {
		assert_eq!( Prefix::Peta.to_string(), "P".to_string() );
		assert_eq!( Prefix::Femto.to_string(), "f".to_string() );
	}

	#[test]
	fn create_sinum() {
		assert_eq!( SiNum::from( 9999.9 ), SiNum::new( 9999.9 ) );
		assert_eq!( SiNum::from( 99999.9 ), SiNum::new( 99999.9 ) );
	}

	#[test]
	fn sinum_value() {
		assert_eq!( SiNum::new( 9999.9 ).as_f64(), 9999.9f64 );
		assert_eq!( SiNum::new( 99999.9 ).as_f64(), 99999.9f64 );
	}

	#[test]
	fn sinum_with_prefix() {
		// Adding a specific prefix is *modifying* the value by keeping the mantissa the same and switching the prefix.
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).as_f64(), 9999900000.0f64 );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Milli ).as_f64(), 9.9999f64 );
	}

	#[test]
	fn sinum_prefix() {
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).prefix(), Prefix::Mega );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Milli ).prefix(), Prefix::Milli );
	}

	#[test]
	fn sinum_mantissa() {
		// The Mantissa is the number displayed before the prefix.
		assert_eq!( SiNum::new( 9999.9 ).mantissa(), 9999.9 );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).mantissa(), 9999.9 );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Milli ).mantissa(), 9999.9 );
	}

	#[test]
	fn sinum_to_prefix() {
		// Converting a `SiNum` to a new prefix shall not change the value itself, only its representation.
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).as_f64(), 9999900000.0f64 );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).to_prefix( Prefix::Milli ).as_f64(), 9999900000.0f64 );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).to_prefix( Prefix::Milli ).mantissa(), 9999900000000.0f64 );
	}

	#[test]
	fn sinum_shorten() {
		// "shorten" means reducing the number of digits to represent the number.
		assert_eq!( SiNum::new( 1000.0 ).shorten().unwrap(), SiNum::new( 1.0 ).with_prefix( Prefix::Kilo ) );
		assert_eq!( SiNum::new( 0.001 ).shorten().unwrap(), SiNum::new( 1.0 ).with_prefix( Prefix::Milli ) );
		assert_eq!( SiNum::new( 1234.5 ).shorten().unwrap(), SiNum::new( 1.2345 ).with_prefix( Prefix::Kilo ) );
	}

	#[test]
	fn sinum_string() {
		assert_eq!( SiNum::new( 9999.9 ).to_string(), "9999.9".to_string() );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).to_string(), "9999.9 M".to_string() );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Milli ).to_string(), "9999.9 m".to_string() );
		assert_eq!( SiNum::new( 9999.9 ).with_prefix( Prefix::Mega ).to_prefix( Prefix::Milli ).to_string(), "9999900000000 m".to_string() );
	}
}
