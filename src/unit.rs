//! The SI units.




//=============================================================================
// Crates


use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

use crate::Latex;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum UnitError {
	#[error( "Not all units represent the same physical quantity: {}", .0.iter().map( |x| x.to_string() ).collect::<Vec<String>>().join( ", " ) )]
	UnitMismatch( Vec<Unit> )
}




//=============================================================================
// Enums


pub(super) enum Dimension {
	Current,
	LuminousIntensity,
	Temperature,
	Mass,
	Length,
	Amount,
	Time,
}

impl Dimension {
	/// Returns the available units for this `Dimension` and the factor to the base unit.
	pub fn units( &self ) -> HashMap<Unit, f64> {
		match self {
			Self::Current => HashMap::from( [
				( Unit::Ampere, 1.0 ),
			] ),
			Self::LuminousIntensity => HashMap::from( [
				( Unit::Candela, 1.0 ),
			] ),
			Self::Temperature => HashMap::from( [
				( Unit::Kelvin, 1.0 ),
			] ),
			Self::Mass => HashMap::from( [
				( Unit::Gram, 1e-3 ),
				( Unit::Kilogram, 1.0 ),
				( Unit::Tonne, 1e3 ),
			] ),
			Self::Length => HashMap::from( [
				( Unit::Meter, 1.0 ),
			] ),
			Self::Amount => HashMap::from( [
				( Unit::Mole, 1.0 ),
			] ),
			Self::Time => HashMap::from( [
				( Unit::Second, 1.0 ),
			] ),
		}
	}
}

impl From<Unit> for Dimension {
	/// Returns the `Dimension` that is measured by `item`.
	fn from( item: Unit ) -> Self {
		item.dimension()
	}
}


/// Represents the different SI units.
#[derive( Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug )]
pub enum Unit {
	// Base units
	Ampere,
	Candela,
	Kelvin,
	Kilogram,
	Meter,
	Mole,
	Second,
	// Additional mass units
	Gram,
	Tonne,
}

impl Unit {
	/// Returns the `Dimension` that is measured by `self`.
	pub(super) fn dimension( &self ) -> Dimension {
		match self {
			// Base units
			Self::Ampere =>    Dimension::Current,
			Self::Candela =>   Dimension::LuminousIntensity,
			Self::Kelvin =>    Dimension::Temperature,
			Self::Kilogram | Self::Gram | Self::Tonne =>  Dimension::Mass,
			Self::Meter =>     Dimension::Length,
			Self::Mole =>      Dimension::Amount,
			Self::Second =>    Dimension::Time,
		}
	}

	/// Returns the factor between the unit and the base unit for the same physical quantity.
	pub(super) fn factor( &self ) -> f64 {
		match self {
			// Base units
			Self::Ampere |
				Self::Candela |
				Self::Kelvin |
				Self::Kilogram |
				Self::Meter |
				Self::Mole |
				Self::Second => 1.0,
			Self::Gram => 1e-3,
			Self::Tonne => 1e3,
		}
	}

	/// Returns the base unit of the unit.
	pub(super) fn base( &self ) -> Self {
		match self {
			// Base units
			Self::Ampere =>    Self::Ampere,
			Self::Candela =>   Self::Candela,
			Self::Kelvin =>    Self::Kelvin,
			Self::Kilogram =>  Self::Kilogram,
			Self::Meter =>     Self::Meter,
			Self::Mole =>      Self::Mole,
			Self::Second =>    Self::Second,
			//
			Self::Gram | Self::Tonne => Self::Kilogram,
		}
	}
}

impl fmt::Display for Unit {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			// Base units
			Self::Ampere =>    write!( f, "A" ),
			Self::Candela =>   write!( f, "cd" ),
			Self::Kelvin =>    write!( f, "K" ),
			Self::Kilogram =>  write!( f, "kg" ),
			Self::Meter =>     write!( f, "m" ),
			Self::Mole =>      write!( f, "mol" ),
			Self::Second =>    write!( f, "s" ),
			// Additional mass units
			Self::Gram =>      write!( f, "g" ),
			Self::Tonne =>     write!( f, "t" ),
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
			// Base units
			Self::Ampere =>    format!( r"\ampere" ),
			Self::Candela =>   format!( r"\candela" ),
			Self::Kelvin =>    format!( r"\kelvin" ),
			Self::Kilogram =>  format!( r"\kilogram" ),
			Self::Meter =>     format!( r"\meter" ),
			Self::Mole =>      format!( r"\mol" ),
			Self::Second =>    format!( r"\second" ),
			// Additional mass units
			Self::Gram =>      format!( r"\gram" ),
			Self::Tonne =>     format!( r"\tonne" ),
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn unit_factor_to_base() {
		assert_eq!( Unit::Ampere.factor(), 1.0 );
		assert_eq!( Unit::Kilogram.factor(), 1.0 );
		assert_eq!( Unit::Tonne.factor(), 1e3 );
	}

	#[test]
	fn unit_base() {
		assert_eq!( Unit::Ampere.base(), Unit::Ampere );
		assert_eq!( Unit::Kilogram.base(), Unit::Kilogram );
		assert_eq!( Unit::Tonne.base(), Unit::Kilogram );
	}

	#[test]
	fn print_prefix() {
		assert_eq!( Unit::Ampere.to_string(), "A".to_string() );
		assert_eq!( Unit::Candela.to_string(), "cd".to_string() );
	}
}
