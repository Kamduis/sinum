//! The SI unit prefixes.




//=============================================================================
// Crates


use std::fmt;

use thiserror::Error;

use crate::Latex;




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

#[cfg( feature = "tex" )]
impl Latex for Prefix {
	/// Return a string that represents this `Prefix` as LaTeX command (requiring the usage of the `{siunitx}` package in LaTeX).
	///
	/// **Note** Requires the **`tex`** feature.
	///
	/// # Example
	/// ```
	/// # use sinum::Latex;
	/// # use sinum::Prefix;
	/// assert_eq!( Prefix::Femto.to_latex(), r"\femto".to_string() );
	/// assert_eq!( Prefix::Nothing.to_latex(), "".to_string() );
	/// assert_eq!( Prefix::Giga.to_latex(), r"\giga".to_string() );
	/// ```
	fn to_latex( &self ) -> String {
		match self {
			Self::Femto =>   format!( r"\femto" ),
			Self::Pico =>    format!( r"\pico" ),
			Self::Nano =>    format!( r"\nano" ),
			Self::Micro =>   format!( r"\micro" ),
			Self::Milli =>   format!( r"\milli" ),
			Self::Centi =>   format!( r"\centi" ),
			Self::Deci =>    format!( r"\deca" ),
			Self::Nothing => format!( "" ),
			Self::Kilo =>    format!( r"\kilo" ),
			Self::Mega =>    format!( r"\mega" ),
			Self::Giga =>    format!( r"\giga" ),
			Self::Tera =>    format!( r"\tera" ),
			Self::Peta =>    format!( r"\peta" ),
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
}
