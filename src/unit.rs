//! The SI units.




//=============================================================================
// Crates


use std::fmt;

use thiserror::Error;

use crate::Latex;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum UnitError {
}




//=============================================================================
// Enums


/// Represents the different SI units.
#[derive( Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug )]
pub enum Unit {
	// Base units
	Ampere,
	Candela,
	Kelvin,
	Kilogram,
	Meter,
	Mole,
	Second,
}

impl fmt::Display for Unit {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			Self::Ampere =>    write!( f, "A" ),
			Self::Candela =>   write!( f, "cd" ),
			Self::Kelvin =>    write!( f, "K" ),
			Self::Kilogram =>  write!( f, "kg" ),
			Self::Meter =>     write!( f, "m" ),
			Self::Mole =>      write!( f, "mol" ),
			Self::Second =>    write!( f, "s" ),
		}
	}
}

#[cfg( feature = "tex" )]
impl Latex for Unit {
	/// Return a string that represents this `Prefix` as LaTeX command (requiring the usage of the `{siunitx}` package in LaTeX).
	///
	/// **Note** Requires the **`tex`** feature.
	///
	/// # Example
	/// ```
	/// # use sinum::Latex;
	/// # use sinum::Unit;
	/// assert_eq!( Unit::Meter.to_latex(), r"\meter".to_string() );
	/// assert_eq!( Unit::Second.to_latex(), r"\second".to_string() );
	/// ```
	fn to_latex( &self ) -> String {
		match self {
			Self::Ampere =>    format!( r"\ampere" ),
			Self::Candela =>   format!( r"\candela" ),
			Self::Kelvin =>    format!( r"\kelvin" ),
			Self::Kilogram =>  format!( r"\kilogram" ),
			Self::Meter =>     format!( r"\meter" ),
			Self::Mole =>      format!( r"\mol" ),
			Self::Second =>    format!( r"\second" ),
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
		assert_eq!( Unit::Ampere.to_string(), "A".to_string() );
		assert_eq!( Unit::Candela.to_string(), "cd".to_string() );
	}
}
