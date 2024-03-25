//! The SI units.




//=============================================================================
// Crates


use std::collections::BTreeSet;
use std::fmt;

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[cfg( feature = "tex" )]
use crate::Latex;
#[cfg( feature = "tex" )]
use crate::Options;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum UnitError {
	#[error( "Not all units represent the same physical quantity: {}", .0.iter().map( |x| x.to_string() ).collect::<Vec<String>>().join( ", " ) )]
	UnitMismatch( Vec<Unit> )
}




//=============================================================================
// Enums


#[derive( PartialEq, Eq, Debug )]
pub(super) enum PhysicalQuantity {
	Custom,
	Current,
	LuminousIntensity,
	Temperature,
	Mass,
	Length,
	Amount,
	Time,
}

impl PhysicalQuantity {
	/// Returns the available units for this `PhysicalQuantity` and the factor to the base unit.
	pub(super) fn units( &self ) -> BTreeSet<Unit> {
		match self {
			Self::Custom => BTreeSet::new(),
			Self::Current => BTreeSet::from( [
				Unit::Ampere,
			] ),
			Self::LuminousIntensity => BTreeSet::from( [
				Unit::Candela,
			] ),
			Self::Temperature => BTreeSet::from( [
				Unit::Kelvin,
			] ),
			Self::Mass => BTreeSet::from( [
				Unit::Gram,
				Unit::Kilogram,
				Unit::Tonne,
			] ),
			Self::Length => BTreeSet::from( [
				Unit::Meter,
			] ),
			Self::Amount => BTreeSet::from( [
				Unit::Mole,
			] ),
			Self::Time => BTreeSet::from( [
				Unit::Second,
			] ),
		}
	}
}

impl From<Unit> for PhysicalQuantity {
	/// Returns the `PhysicalQuantity` that is measured by `item`.
	fn from( item: Unit ) -> Self {
		item.phys()
	}
}


/// Represents the different SI units.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug )]
pub enum Unit {
	Custom( String ),
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
	/// Returns the `PhysicalQuantity` that is measured by `self`.
	pub(super) fn phys( &self ) -> PhysicalQuantity {
		match self {
			Self::Custom( _ ) => PhysicalQuantity::Custom,
			// Base units
			Self::Ampere =>    PhysicalQuantity::Current,
			Self::Candela =>   PhysicalQuantity::LuminousIntensity,
			Self::Kelvin =>    PhysicalQuantity::Temperature,
			Self::Kilogram | Self::Gram | Self::Tonne =>  PhysicalQuantity::Mass,
			Self::Meter =>     PhysicalQuantity::Length,
			Self::Mole =>      PhysicalQuantity::Amount,
			Self::Second =>    PhysicalQuantity::Time,
		}
	}

	/// Returns the factor between the unit and the base unit for the same physical quantity.
	pub(super) fn factor( &self ) -> f64 {
		match self {
			Self::Custom( _ ) => 1.0,
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
			Self::Custom( x ) => Self::Custom( x.clone() ),
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
			Self::Custom( x ) => write!( f, "{}", x ),
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
	/// # use sinum::{Unit, Options};
	/// assert_eq!( Unit::Meter.to_latex( &Options::none() ), r"\meter".to_string() );
	/// assert_eq!( Unit::Second.to_latex( &Options::new() ), r"\second".to_string() );
	/// ```
	fn to_latex( &self, _options: &Options ) -> String {
		match self {
			Self::Custom( x ) => x.clone(),
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
