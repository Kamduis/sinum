//! Provides quantities representing numbers combined with the SI prefix and unit system.




//=============================================================================
// Crates


use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::Latex;
use crate::prefix::PrefixError;
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
		let ( num, uni ) = match unit {
			// The Kilogram as base unit must only be used if the number prefix is `Prefix::Nothing`. If the Prefix is anything else, the unit `Unit::Gram` must be used to correctly display the prefixes like "mg" or "ng".
			Unit::Kilogram if number.prefix() != Prefix::Nothing => {
				let exp_new = number.prefix().exp() + 3;
				let prefix_new = Prefix::try_from( exp_new ).unwrap();
				( number.with_prefix( prefix_new ), Unit::Gram )
			},
			Unit::Gram if number.prefix() == Prefix::Kilo => {
				( number.with_prefix( Prefix::Nothing ), Unit::Kilogram )
			},
			_ => ( number, unit ),
		};

		Self {
			number: num,
			unit: uni,
		}
	}

	/// Creates a new `SiQty` from `self` with a reduced numbers of digits of the mantissa (see `mantissa()`) required to represent the number:
	///
	/// * No more than 3 digits in front of the decimal point.
	/// 	(1234 s → 1.234 ks)
	///
	/// * No zero in front of the decimal point.
	/// 	(0.001 A → 1.0 mA)
	///
	/// This function will only modify the prefix, never the unit itself.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// assert_eq!(
	/// 	SiQty::new( 1000.0.into(), Unit::Ampere ).shorten().unwrap(),
	/// 	SiQty::new( SiNum::new( 1.0.into() ).with_prefix( Prefix::Kilo ), Unit::Ampere )
	/// );
	/// assert_eq!(
	/// 	SiQty::new( 0.001.into(), Unit::Candela ).shorten().unwrap(),
	/// 	SiQty::new( SiNum::new( 1.0 ).with_prefix( Prefix::Milli ), Unit::Candela )
	/// );
	/// ```
	pub fn shorten( self ) -> Result<Self, PrefixError> {
		let num = self.number.shorten()?;

		Ok( Self::new( num, self.unit() ) )
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
		self.number.as_f64() * self.unit.factor()
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

	/// Creates a new `SiQty` from `self` at the specified `prefix`.
	///
	/// The numeric value of the new `SiQty` will be identical to `self` (apart from possible floating point rounding errors) since the mantissa is being modified alongside the prefix to reflect the same numeric value as before.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Prefix, Unit};
	/// let qty = SiQty::new( 2.0.into(), Unit::Meter );
	///
	/// assert_eq!( qty.as_f64(), 2.0 );
	/// assert_eq!( qty.to_prefix( Prefix::Milli ).as_f64(), 2.0 );
	/// assert_eq!( qty.to_prefix( Prefix::Milli ).number().mantissa(), 2_000.0 );
	///
	/// assert_eq!( qty.to_prefix( Prefix::Kilo ).as_f64(), 2.0 );
	/// assert_eq!( qty.to_prefix( Prefix::Kilo ).number().mantissa(), 0.002 );
	/// ```
	pub fn to_prefix( self, prefix: Prefix ) -> Self {
		let number = self.number.to_prefix( prefix );
		Self::new( number, self.unit )
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
			.expect( "This unit is not assigned to a dimension, which it really should be" );

		let factor = factor_old / factor_new;

		let num_new = self.number() * factor;

		Ok( Self::new( num_new, unit ) )
	}

	/// Computes the absolute value of `self` with respect to the base unit. This means 10.0 t are returned as 10e3.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let x = SiQty::new( 3.5.into(), Unit::Ampere );
	/// let y = SiQty::new( SiNum::from( -3.5 ), Unit::Ampere );
	///
	/// let abs_difference_x = ( x.abs() - x ).abs();
	/// let abs_difference_y = ( y.abs() - ( -y ) ).abs();
	///
	/// assert!( abs_difference_x < 1e-10 );
	/// assert!( abs_difference_y < 1e-10 );
	/// ```
	pub fn abs( self ) -> Self {
		let val = self.as_f64().abs();
		Self::new( SiNum::new( val ).to_prefix( self.number.prefix() ), self.unit() )
	}
}

impl PartialEq<f64> for SiQty {
	/// Compares a `SiQty` and a `f64` for equality.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Prefix, Unit};
	/// assert!( SiQty::new( 1.1.into(), Unit::Ampere ) == 1.1 );
	/// assert!( SiQty::new( SiNum::new( 2.0 ).with_prefix( Prefix::Kilo ), Unit::Second ) == 2e3 );
	/// ```
	fn eq( &self, other: &f64 ) -> bool {
		self.as_f64().eq( &other )
	}
}

impl PartialOrd for SiQty {
	fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
		self.as_f64().partial_cmp( &other.as_f64() )
	}

	fn lt( &self, other: &Self ) -> bool {
		self.as_f64() < other.as_f64()
	}

	fn le( &self, other: &Self ) -> bool {
		self.as_f64() <= other.as_f64()
	}

	fn ge( &self, other: &Self ) -> bool {
		self.as_f64() >= other.as_f64()
	}

	fn gt( &self, other: &Self ) -> bool {
		self.as_f64() > other.as_f64()
	}
}

impl PartialOrd<f64> for SiQty {
	fn partial_cmp( &self, other: &f64 ) -> Option<Ordering> {
		self.as_f64().partial_cmp( &other )
	}

	fn lt( &self, other: &f64 ) -> bool {
		self.as_f64() < *other
	}

	fn le( &self, other: &f64 ) -> bool {
		self.as_f64() <= *other
	}

	fn ge( &self, other: &f64 ) -> bool {
		self.as_f64() >= *other
	}

	fn gt( &self, other: &f64 ) -> bool {
		self.as_f64() > *other
	}
}

impl Add for SiQty {
	type Output = Self;

	/// The addition operator `+`. The resulting `SiQty` will keep the prefix and unit of `self`.
	///
	/// **Note:** Adding two `SiQty`s representing different physical quantities results in a **panic**.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) + SiQty::new( 0.1.into(), Unit::Ampere );
	///
	/// assert_eq!( calc_a, SiQty::new( 1.1.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) + SiQty::new( 4.0.into(), Unit::Tonne );
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( 4_000_000_008.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn add( self, other: Self ) -> Self::Output {
		let val = self.as_f64() + other.as_f64();

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Add<f64> for SiQty {
	type Output = Self;

	/// The addition operator `+`. The resulting `SiNum` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) + 0.1;
	///
	/// assert_eq!( calc_a, SiQty::new( 1.1.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 32.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) + 4.0;
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( 4_000_032.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn add( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() + other;

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Sub for SiQty {
	type Output = Self;

	/// The subtraction operator `-`. The resulting `SiQty` will keep the prefix and unit of `self`.
	///
	/// **Note:** Subtracting two `SiQty`s representing different physical quantities results in a **panic**.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) - SiQty::new( 0.1.into(), Unit::Ampere );
	///
	/// assert_eq!( calc_a, SiQty::new( 0.9.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) - SiQty::new( 4.0.into(), Unit::Tonne );
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( -3_999_999_992.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn sub( self, other: Self ) -> Self::Output {
		let val = self.as_f64() - other.as_f64();

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Sub<f64> for SiQty {
	type Output = Self;

	/// The subtraction operator `-`. The resulting `SiNum` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) - 0.1;
	///
	/// assert_eq!( calc_a, SiQty::new( 0.9.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) - 4.0;
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( -3_999_992.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn sub( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() - other;

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Mul for SiQty {
	type Output = Self;

	/// The multiplication operator `*`. The resulting `SiNum` will keep the prefix and unit of `self`.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiNum, Prefix};
	/// let calc_a = SiNum::new( 1.0 ) * SiNum::new( 0.1 );
	///
	/// assert_eq!( calc_a, SiNum::new( 0.1 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = SiNum::new( 2.0 ).with_prefix( Prefix::Kilo ) * SiNum::new( 4.0 );
	///
	/// assert_eq!( calc_b, SiNum::new( 8.0 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn mul( self, other: Self ) -> Self::Output {
		let val = self.as_f64() * other.as_f64();

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Mul<f64> for SiQty {
	type Output = Self;

	/// The multiplication operator `*`. The resulting `SiQty` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) * 0.1;
	///
	/// assert_eq!( calc_a, SiQty::new( 0.1.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) * 4.0;
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( 32.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn mul( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() * other;

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Div for SiQty {
	type Output = Self;

	/// The multiplication operator `/`. The resulting `SiQty` will keep the higher prefix of the two parts.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) / SiQty::new( 0.1.into(), Unit::Ampere );
	///
	/// assert_eq!( calc_a, SiQty::new( 10.0.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) / SiQty::new( 4.0.into(), Unit::Tonne );
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( 2e-3 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn div( self, other: Self ) -> Self::Output {
		let val = self.as_f64() / other.as_f64();

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Div<f64> for SiQty {
	type Output = Self;

	/// The multiplication operator `/`. The resulting `SiQty` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{SiQty, SiNum, Unit, Prefix};
	/// let calc_a = SiQty::new( 1.0.into(), Unit::Ampere ) / 0.1;
	///
	/// assert_eq!( calc_a, SiQty::new( 10.0.into(), Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), Unit::Ampere );
	///
	/// let calc_b = SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) / 4.0;
	///
	/// assert_eq!( calc_b, SiQty::new( SiNum::new( 2.0 ).with_prefix( Prefix::Milli ), Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn div( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() / other;

		Self::new( val.into(), self.unit.base() )
			.to_unit( self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Neg for SiQty {
	type Output = Self;

	fn neg( self ) -> Self::Output {
		let val = -self.as_f64();
		let num = SiNum::new( val ).to_prefix( self.number.prefix() );

		Self::new( num, self.unit.base() ).to_unit( self.unit ).unwrap()
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

#[cfg( feature = "tex" )]
impl Latex for SiQty {
	/// Return a string that represents this `SiQty` as LaTeX command (requiring the usage of the `{siunitx}` package in LaTeX).
	///
	/// **Note** Requires the **`tex`** feature.
	///
	/// # Example
	/// ```
	/// # use sinum::Latex;
	/// # use sinum::{SiQty, Unit, SiNum, Prefix};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).to_latex(), r"\qty{9.9}{\ampere}".to_string() );
	/// assert_eq!(
	/// 	SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Ampere ).to_latex(),
	/// 	r"\qty{9.9}{\milli\ampere}".to_string()
	/// );
	/// ```
	///
	/// # Kilogram
	///
	/// The base unit for mass, the kilogram is a special case, since it already has a prefix (kilo), that has to be taken into account.
	/// ```
	/// # use sinum::Latex;
	/// # use sinum::{SiQty, Unit, SiNum, Prefix};
	/// assert_eq!( SiQty::new( 9.9.into(), Unit::Kilogram ).to_latex(), r"\qty{9.9}{\kilogram}".to_string() );
	/// assert_eq!(
	/// 	SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Kilogram ).to_latex(),
	/// 	r"\qty{9.9}{\mega\gram}".to_string()
	/// );
	/// assert_eq!(
	/// 	SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Kilogram ).to_latex(),
	/// 	r"\qty{9.9}{\gram}".to_string()
	/// );
	/// assert_eq!(
	/// 	SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Micro ), Unit::Kilogram ).to_latex(),
	/// 	r"\qty{9.9}{\milli\gram}".to_string()
	/// );
	/// assert_eq!( SiQty::new(
	/// 	SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Gram ).to_latex(),
	/// 	r"\qty{9.9}{\milli\gram}".to_string()
	/// );
	/// assert_eq!(
	/// 	SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Gram ).to_latex(),
	/// 	r"\qty{9.9}{\kilogram}".to_string()
	/// );
	/// ```
	fn to_latex( &self ) -> String {
		format!(
			r"\qty{{{}}}{{{}{}}}",
			self.number.mantissa(),
			self.number.prefix().to_latex(),
			self.unit.to_latex()
		)
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	use crate::Prefix;

	#[test]
	fn siqty_as_f64() {
		// `as_f64()` returns the value with regard to the base unit.
		assert_eq!( SiQty::new( 9.9.into(), Unit::Tonne ).as_f64(), 9.9e3 );
		assert_eq!( SiQty::new( SiNum::new( 8.0 ).with_prefix( Prefix::Milli ), Unit::Gram ).as_f64(), 8.0e-6 );
	}

	#[test]
	fn siqty_string() {
		assert_eq!( SiQty::new( 9.9.into(), Unit::Ampere ).to_string(), "9.9 A".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Meter ).to_string(), "9.9 km".to_string() );
		assert_eq!( SiQty::new( 9.9.into(), Unit::Kelvin ).to_string(), "9.9 K".to_string() );
	}

	// The weight/mass is a special case.
	#[test]
	fn siqty_kilogram() {
		assert_eq!( SiQty::new( 9.9.into(), Unit::Kilogram ).as_f64(), 9.9 );
		assert_eq!( SiQty::new( 9.9.into(), Unit::Kilogram ).number(), SiNum::new( 9.9 ) );
		assert_eq!( SiQty::new( 9.9.into(), Unit::Kilogram ).to_string(), "9.9 kg".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Kilogram ).to_string(), "9.9 Mg".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Kilogram ).to_string(), "9.9 g".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Micro ), Unit::Kilogram ).to_string(), "9.9 mg".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Gram ).to_string(), "9.9 mg".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Gram ).to_string(), "9.9 kg".to_string() );
	}

	#[cfg( feature = "tex" )]
	#[test]
	fn siqty_latex_kilogram() {
		assert_eq!( SiQty::new( 9.9.into(), Unit::Kilogram ).to_latex(), r"\qty{9.9}{\kilogram}".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Kilogram ).to_latex(), r"\qty{9.9}{\mega\gram}".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Kilogram ).to_latex(), r"\qty{9.9}{\gram}".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Micro ), Unit::Kilogram ).to_latex(), r"\qty{9.9}{\milli\gram}".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Milli ), Unit::Gram ).to_latex(), r"\qty{9.9}{\milli\gram}".to_string() );
		assert_eq!( SiQty::new( SiNum::new( 9.9 ).with_prefix( Prefix::Kilo ), Unit::Gram ).to_latex(), r"\qty{9.9}{\kilogram}".to_string() );
	}
}
