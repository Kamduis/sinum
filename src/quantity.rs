//! Provides quantities representing numbers combined with the SI prefix and unit system.




//=============================================================================
// Crates


use std::fmt;

use crate::unit::UnitError;
use crate::{SiNum, Prefix, Unit, Dimension};




//=============================================================================
// Structs


/// Represents a number in combination with a SI prefix.
#[derive( Clone, Copy, PartialEq, Debug )]
pub struct SiQty {
	number: SiNum,
	unit: Unit,
}

impl SiQty {
	/// Create a new `SiQty` representing a numeric value and a unit.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, Unit};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).as_f64(), 9.9 );
	/// assert_eq!( SiQty::new( 99.9.into(), Unit::Kelvin ).as_f64(), 99.9 );
	/// ```
	pub fn new( number: SiNum, unit: Unit ) -> Self {
		Self {
			number,
			unit,
		}
	}

	/// Returns the numeric value of the `SiQty` without any prefix or unit.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, Unit};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).as_f64(), 9.9 );
	/// assert_eq!( SiQty::new( 99.9.into(), Unit::Kelvin ).as_f64(), 99.9 );
	/// ```
	pub fn as_f64( &self ) -> f64 {
		self.number.as_f64()
	}

	/// Returns the numeric `SiNum` of the `SiQty`.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, SiQty, Unit};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).number(), SiNum::new( 9.9 ) );
	/// ```
	pub fn number( &self ) -> SiNum {
		self.number
	}

	/// Returns the unit of the `SiQty`.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, Unit};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).unit(), Unit::Ampere );
	/// ```
	pub fn unit( &self ) -> Unit {
		self.unit
	}

	/// Returns the dimension that is represented by the `SiQty`.
	fn dimension( &self ) -> Dimension {
		self.unit.dimension()
	}

	/// Returns a new `SiQty` from `self` with the new `unit`.
	///
	/// If `unit` does not represent the same dimension as the original unit, this function returns an `UnitError`.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, Unit};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Kilogram ).to_unit( Unit::Tonne ).unwrap(), SiQty::new( 0.0099.into(), Unit::Tonne ) );
	/// assert!( SiQty::new( 9.9.into(), Unit::Kilogram ).to_unit( Unit::Second ).is_err() );
	/// ```
	pub fn to_unit( &self, unit: Unit ) -> Result<Self, UnitError> {
		let units = self.dimension().units();
		let Some( factor_new ) = units.get( &unit ) else {
			return Err( UnitError::UnitMismatch( vec![ self.unit(), unit ] ) );
		};

		let factor_old = units.get( &self.unit() ).clone()
			.expect( "This unit is not assigned to a dimension, which it really should be!" );

		let factor = factor_old / factor_new;

		let num_new = self.number() * factor;

		Ok( Self::new( num_new, unit ) )
	}
}

impl fmt::Display for SiQty {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self.number.prefix() {
			Prefix::Nothing => write!( f, "{} {}", self.number, self.unit ),
			_ => write!( f, "{}{}", self.number, self.unit ),
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	use crate::Prefix;

	#[test]
	fn siqty_string() {
		assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).to_string(), "9.9 A".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Meter ).to_string(), "9.9 km".to_string() );
		assert_eq!( SiQty::new( 9.9.into(), Unit::Kelvin ).to_string(), "9.9 K".to_string() );
	}
}
