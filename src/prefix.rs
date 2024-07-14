//! The SI unit prefixes.




//=============================================================================
// Crates


use std::fmt;
use std::str::FromStr;

use thiserror::Error;

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};

#[cfg( feature = "tex" )]
use crate::Latex;
#[cfg( feature = "tex" )]
use crate::TexOptions;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum PrefixError {
	#[error( "There is no Prefix with exponent `{0}`" )]
	TryFromExp( i8 ),

	#[error( "Cannot convert to Prefix from: `{0}`" )]
	TryFromStr( String ),

	#[error( "There is no SI prefix for `{0}`" )]
	ExpInvalid( i32 ),
}




//=============================================================================
// Enums


/// Represents the different SI prefixes like kilo, milli, nano etc.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug )]
pub enum Prefix {
	Yocto,
	Zepto,
	Atto,
	Femto,
	Pico,
	Nano,
	Micro,
	Milli,
	Centi,
	Deci,
	Nothing,
	Deca,
	Hecto,
	Kilo,
	Mega,
	Giga,
	Tera,
	Peta,
	Exa,
	Zetta,
	Yotta,
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
			Self::Yocto => 1e-24,
			Self::Zepto => 1e-21,
			Self::Atto =>  1e-18,
			Self::Femto => 1e-15,
			Self::Pico =>  1e-12,
			Self::Nano =>  1e-9,
			Self::Micro => 1e-6,
			Self::Milli => 1e-3,
			Self::Centi => 1e-2,
			Self::Deci =>  1e-1,
			Self::Nothing => 1.0,
			Self::Deca =>  1e1,
			Self::Hecto => 1e2,
			Self::Kilo =>  1e3,
			Self::Mega =>  1e6,
			Self::Giga =>  1e9,
			Self::Tera =>  1e12,
			Self::Peta =>  1e15,
			Self::Exa =>   1e18,
			Self::Zetta => 1e21,
			Self::Yotta => 1e24,
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
			Self::Yocto =>  -24,
			Self::Zepto =>  -21,
			Self::Atto =>   -18,
			Self::Femto =>  -15,
			Self::Pico =>   -12,
			Self::Nano =>    -9,
			Self::Micro =>   -6,
			Self::Milli =>   -3,
			Self::Centi =>   -2,
			Self::Deci =>    -1,
			Self::Nothing =>  0,
			Self::Deca =>     1,
			Self::Hecto =>    2,
			Self::Kilo =>     3,
			Self::Mega =>     6,
			Self::Giga =>     9,
			Self::Tera =>    12,
			Self::Peta =>    15,
			Self::Exa =>     18,
			Self::Zetta =>   21,
			Self::Yotta =>   24,
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
			-24 => Self::Yocto,
			-21 => Self::Zepto,
			-18 => Self::Atto,
			-15 => Self::Femto,
			-12 => Self::Pico,
			-9  => Self::Nano,
			-6  => Self::Micro,
			-3  => Self::Milli,
			-2  => Self::Centi,
			-1  => Self::Deci,
			0   => Self::Nothing,
			1   => Self::Deca,
			2   => Self::Hecto,
			3   => Self::Kilo,
			6   => Self::Mega,
			9   => Self::Giga,
			12  => Self::Tera,
			15  => Self::Peta,
			18  => Self::Exa,
			21  => Self::Zetta,
			24  => Self::Yotta,
			_ => return Err( PrefixError::TryFromExp( item ) ),
		};

		Ok( res )
	}
}

impl FromStr for Prefix {
	type Err = PrefixError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let result = match s.to_lowercase().as_str() {
			"yocto"   => Self::Yocto,
			"zepto"   => Self::Zepto,
			"atto"    => Self::Atto,
			"femto"   => Self::Femto,
			"pico"    => Self::Pico,
			"nano"    => Self::Nano,
			"micro"   => Self::Micro,
			"milli"   => Self::Milli,
			"centi"   => Self::Centi,
			"deci"    => Self::Deci,
			"nothing" => Self::Nothing,
			"deca"    => Self::Deca,
			"hecto"   => Self::Hecto,
			"kilo"    => Self::Kilo,
			"mega"    => Self::Mega,
			"giga"    => Self::Giga,
			"tera"    => Self::Tera,
			"peta"    => Self::Peta,
			"exa"     => Self::Exa,
			"zetta"   => Self::Zetta,
			"yotta"   => Self::Yotta,
			_ => return Err( PrefixError::TryFromStr( s.to_string() ) ),
		};

		Ok( result )
	}
}

impl fmt::Display for Prefix {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			Self::Yocto =>   write!( f, "y" ),
			Self::Zepto =>   write!( f, "z" ),
			Self::Atto =>    write!( f, "a" ),
			Self::Femto =>   write!( f, "f" ),
			Self::Pico =>    write!( f, "p" ),
			Self::Nano =>    write!( f, "n" ),
			Self::Micro =>   write!( f, "µ" ),
			Self::Milli =>   write!( f, "m" ),
			Self::Centi =>   write!( f, "c" ),
			Self::Deci =>    write!( f, "d" ),
			Self::Nothing => write!( f, "" ),
			Self::Deca =>    write!( f, "da" ),
			Self::Hecto =>   write!( f, "h" ),
			Self::Kilo =>    write!( f, "k" ),
			Self::Mega =>    write!( f, "M" ),
			Self::Giga =>    write!( f, "G" ),
			Self::Tera =>    write!( f, "T" ),
			Self::Peta =>    write!( f, "P" ),
			Self::Exa =>     write!( f, "E" ),
			Self::Zetta =>   write!( f, "Z" ),
			Self::Yotta =>   write!( f, "Y" ),
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
	/// # use sinum::{Prefix, TexOptions};
	/// assert_eq!( Prefix::Femto.to_latex( &TexOptions::none() ), r"\femto".to_string() );
	/// assert_eq!( Prefix::Nothing.to_latex( &TexOptions::none() ), "".to_string() );
	/// assert_eq!( Prefix::Giga.to_latex( &TexOptions::none() ), r"\giga".to_string() );
	/// ```
	fn to_latex( &self, _options: &TexOptions ) -> String {
		match self {
			Self::Yocto =>   format!( r"\yocto" ),
			Self::Zepto =>   format!( r"\zepto" ),
			Self::Atto =>    format!( r"\atto" ),
			Self::Femto =>   format!( r"\femto" ),
			Self::Pico =>    format!( r"\pico" ),
			Self::Nano =>    format!( r"\nano" ),
			Self::Micro =>   format!( r"\micro" ),
			Self::Milli =>   format!( r"\milli" ),
			Self::Centi =>   format!( r"\centi" ),
			Self::Deci =>    format!( r"\deca" ),
			Self::Nothing => format!( "" ),
			Self::Deca =>    format!( r"\deca" ),
			Self::Hecto =>   format!( r"\hecto" ),
			Self::Kilo =>    format!( r"\kilo" ),
			Self::Mega =>    format!( r"\mega" ),
			Self::Giga =>    format!( r"\giga" ),
			Self::Tera =>    format!( r"\tera" ),
			Self::Peta =>    format!( r"\peta" ),
			Self::Exa =>     format!( r"\exa" ),
			Self::Zetta =>   format!( r"\zetta" ),
			Self::Yotta =>   format!( r"\yotta" ),
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
