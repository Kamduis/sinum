//! Provides quantities representing numbers combined with the SI prefix and unit system.




//=============================================================================
// Crates


use std::fmt;

use crate::{SiNum, Prefix, Unit};




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
	fn sinum_string() {
		assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).to_string(), "9.9 A".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Meter ).to_string(), "9.9 km".to_string() );
	}
}
