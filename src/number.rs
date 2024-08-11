//! Provides values using the SI prefix system.




//=============================================================================
// Crates


use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, MulAssign, Div, Neg};
use std::fmt;

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};

use crate::PrefixError;
use crate::{Prefix, Qty, Unit};




//=============================================================================
// Structs


/// Represents a number in combination with a SI prefix.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Copy, Debug )]
pub struct Num {
	mantissa: f64,
	prefix: Prefix
}

impl Num {
	/// Create a new `Num` representing the numeric value `num` without any prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::Num;
	/// assert_eq!( Num::new( 9999.9 ).as_f64(), 9999.9 );
	/// assert_eq!( Num::new( 99999.9 ).as_f64(), 99999.9 );
	/// ```
	pub fn new( num: f64 ) -> Self {
		Self {
			mantissa: num,
			prefix: Prefix::Nothing,
		}
	}

	/// Creates a new `Num` from `self` and applying `prefix`.
	///
	/// *Note:* The numeric value of the new `Num` will be different from `self` (aside from using the same `Prefix`) since the mantissa is staying the same while the `Prefix` is modified.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Mega ).as_f64(), 9_999_900_000.0 );
	/// assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Milli ).as_f64(), 9.9999 );
	/// ```
	pub fn with_prefix( self, prefix: Prefix ) -> Self {
		Self {
			mantissa: self.mantissa,
			prefix,
		}
	}

	/// Creates a new `Qty` from `self` by applying `unit`.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Qty, Unit};
	/// assert_eq!( Num::new( 9.9 ).with_unit( &Unit::Second ), Qty::new( 9.9.into(), &Unit::Second ) );
	/// ```
	pub fn with_unit( self, unit: &Unit ) -> Qty {
		Qty::new( self, unit )
	}

	/// Creates a new `Num` from `self` at the specified `prefix`.
	///
	/// The numeric value of the new `Num` will be identical to `self` (apart from possible floating point rounding errors) since the mantissa is being modified alongside the prefix to reflect the same numeric value as before.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let num = Num::new( 9999.9 );
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

	/// Creates a new `Num` from `self` with a reduced numbers of digits of the mantissa (see `mantissa()`) required to represent the number:
	///
	/// * No more than 3 digits in front of the decimal point.
	///     (1234 → 1.234 k)
	///
	/// * No zero in front of the decimal point.
	///     (0.001 → 1.0 m)
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// assert_eq!(
	///     Num::new( 1000.0 ).shortened().unwrap(),
	///     Num::new( 1.0 ).with_prefix( Prefix::Kilo )
	/// );
	/// assert_eq!(
	///     Num::new( 0.001 ).shortened().unwrap(),
	///     Num::new( 1.0 ).with_prefix( Prefix::Milli )
	/// );
	/// assert_eq!(
	///     Num::new( 1234.5 ).shortened().unwrap(),
	///     Num::new( 1.2345 ).with_prefix( Prefix::Kilo )
	/// );
	/// assert_eq!(
	///     Num::new( 0.0 ).with_prefix( Prefix::Mega ).shortened().unwrap(),
	///     Num::new( 0.0 )
	/// );
	/// ```
	pub fn shortened( self ) -> Result<Self, PrefixError> {
		if self.mantissa == 0.0 {
			return Ok( Self::new( 0.0 ) );
		}

		let exps = self.mantissa.log10().floor().div_euclid( 3.0 ) * 3.0;

		if exps > Prefix::MAX_EXP as f64 {
			return Err( PrefixError::ExpInvalid( exps as i32 ) );
		}

		let exp_new = self.prefix.exp() + exps as i8;
		let prefix_new = Prefix::try_from( exp_new )?;

		Ok( self.to_prefix( prefix_new ) )
	}

	/// Returns the mantissa of the `Num`. The Mantissa is the number displayed before the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let num = Num::new( 9999.9 );
	///
	/// assert_eq!( num.mantissa(), 9999.9 );
	/// assert_eq!( num.with_prefix( Prefix::Mega ).mantissa(), 9999.9 );
	/// assert_eq!( num.with_prefix( Prefix::Milli ).mantissa(), 9999.9 );
	/// ```
	pub fn mantissa( &self ) -> f64 {
		self.mantissa
	}

	/// Returns the `Prefix` of the `Num`.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let num = Num::new( 9999.9 ).with_prefix( Prefix::Mega );
	/// assert_eq!( num.prefix(), Prefix::Mega );
	/// ```
	pub fn prefix( &self ) -> Prefix {
		self.prefix
	}

	/// Returns the numeric value of the `Num` without any prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::Num;
	/// assert_eq!( Num::new( 9999.9 ).as_f64(), 9999.9 );
	/// assert_eq!( Num::new( 99999.9 ).as_f64(), 99999.9 );
	/// ```
	pub fn as_f64( &self ) -> f64 {
		self.mantissa * self.prefix.as_f64()
	}

	/// Computes the absolute value of `self`.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let x = Num::new( 3.5 );
	/// let y = Num::new( -3.5 );
	///
	/// let abs_difference_x = ( x.abs() - x ).abs();
	/// let abs_difference_y = ( y.abs() - ( -y ) ).abs();
	///
	/// assert!( abs_difference_x < 1e-10 );
	/// assert!( abs_difference_y < 1e-10 );
	/// ```
	pub fn abs( self ) -> Self {
		let val = self.as_f64().abs();
		Self::new( val ).to_prefix( self.prefix() )
	}

	/// Raises the number to an integer power.
	///
	/// Using this function is generally faster than using `powf`. It might have a different sequence of rounding operations than `powf`, so the results are not guaranteed to agree.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let x = Num::new( 2.0 );
	/// let abs_diff = ( x.powi( 2 ) - ( x * x ) ).abs();
	///
	/// assert!( abs_diff < 1e-10 );
	/// ```
	pub fn powi( self, n: i32 ) -> Self {
		let val = self.as_f64().powi( n );
		Self::new( val ).to_prefix( self.prefix() )
	}

	/// Raises the number to a floating point power.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let x = Num::new( 2.0 );
	/// let abs_diff = ( x.powf( 2.0 ) - ( x * x ) ).abs();
	///
	/// assert!( abs_diff < 1e-10 );
	/// ```
	pub fn powf( self, n: f64 ) -> Self {
		let val = self.as_f64().powf( n );
		Self::new( val ).to_prefix( self.prefix() )
	}

	/// Returns a string representation of the number with engineering notation.
	/// Engineering notation is similar to scientific notation (using exponents of ten) but the exponents are always a multiple of 3.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let x = Num::new( 2.0 ).with_prefix( Prefix::Milli );
	///
	/// assert_eq!( x.to_string_eng(), "2×10^-3" );
	/// ```
	pub fn to_string_eng( &self ) -> String {
		match self.prefix {
			Prefix::Nothing => self.mantissa.to_string(),
			_ => format!( "{}×10^{}", self.mantissa, self.prefix.exp() )
		}
	}
}

impl PartialEq for Num {
	/// Compares `Num`s for equality. Since a `Num` always represents a floating point number all of the pityfalls of comparing those apply.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// assert!( Num::new( 1.1 ) == Num::new( 1.1 ) );
	/// assert!( Num::new( 2.0 ).with_prefix( Prefix::Kilo ) == Num::new( 2e6 ).with_prefix( Prefix::Milli ) );
	/// ```
	fn eq( &self, other: &Self ) -> bool {
		self.as_f64().eq( &other.as_f64() )
	}
}

impl PartialEq<f64> for Num {
	/// Compares a `Num` and a `f64` for equality.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// assert!( Num::new( 1.1 ) == 1.1 );
	/// assert!( Num::new( 2.0 ).with_prefix( Prefix::Kilo ) == 2e3 );
	/// ```
	fn eq( &self, other: &f64 ) -> bool {
		self.as_f64().eq( &other )
	}
}

impl PartialOrd for Num {
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

impl PartialOrd<f64> for Num {
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

impl Add for Num {
	type Output = Self;

	/// The addition operator `+`. The resulting `Num` will keep the higher prefix of the two parts.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) + Num::new( 0.1 );
	///
	/// assert_eq!( calc_a, Num::new( 1.1 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) + Num::new( 4.0 );
	///
	/// assert_eq!( calc_b, Num::new( 2.004 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	///
	/// **Note** Since the numbers added together can vary widely in magnitude, common floating point errors may show up:
	/// ```
	/// # use sinum::{Num, Prefix};
	/// assert_eq!(
	///     Num::new( 1.0 ).with_prefix( Prefix::Mega ) + Num::new( 1.0 ).with_prefix( Prefix::Micro ),
	///     Num::new( 1.0000000000009999 ).with_prefix( Prefix::Mega )
	/// );
	/// ```
	fn add( self, other: Self ) -> Self::Output {
		let val = self.as_f64() + other.as_f64();
		let pref = self.prefix().max( other.prefix() );

		Self::new( val ).to_prefix( pref )
	}
}

impl Add<f64> for Num {
	type Output = Self;

	/// The addition operator `+`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) + 0.1;
	///
	/// assert_eq!( calc_a, Num::new( 1.1 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) + 4.0;
	///
	/// assert_eq!( calc_b, Num::new( 2.004 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn add( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() + other;

		Self::new( val ).to_prefix( self.prefix() )
	}
}

impl Sub for Num {
	type Output = Self;

	/// The subtraction operator `-`. The resulting `Num` will keep the higher prefix of the two parts.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) - Num::new( 0.1 );
	///
	/// assert_eq!( calc_a, Num::new( 0.9 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) - Num::new( 4.0 );
	///
	/// assert_eq!( calc_b, Num::new( 1.996 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn sub( self, other: Self ) -> Self::Output {
		let val = self.as_f64() - other.as_f64();
		let pref = self.prefix().max( other.prefix() );

		Self::new( val ).to_prefix( pref )
	}
}

impl Sub<f64> for Num {
	type Output = Self;

	/// The subtraction operator `-`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) - 0.1;
	///
	/// assert_eq!( calc_a, Num::new( 0.9 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) - 4.0;
	///
	/// assert_eq!( calc_b, Num::new( 1.996 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn sub( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() - other;

		Self::new( val ).to_prefix( self.prefix() )
	}
}

impl Mul for Num {
	type Output = Self;

	/// The multiplication operator `*`. The resulting `Num` will keep the higher prefix of the two parts.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) * Num::new( 0.1 );
	///
	/// assert_eq!( calc_a, Num::new( 0.1 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) * Num::new( 4.0 );
	///
	/// assert_eq!( calc_b, Num::new( 8.0 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn mul( self, other: Self ) -> Self::Output {
		let val = self.as_f64() * other.as_f64();
		let pref = self.prefix().max( other.prefix() );

		Self::new( val ).to_prefix( pref )
	}
}

impl Mul<f64> for Num {
	type Output = Self;

	/// The multiplication operator `*`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) * 0.1;
	///
	/// assert_eq!( calc_a, Num::new( 0.1 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) * 4.0;
	///
	/// assert_eq!( calc_b, Num::new( 8.0 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn mul( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() * other;

		Self::new( val ).to_prefix( self.prefix() )
	}
}

impl MulAssign<f64> for Num {
	/// The multiplication assignment operator `*=`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let mut calc_a = Num::new( 1.0 );
	/// calc_a *= 0.1;
	///
	/// assert_eq!( calc_a, Num::new( 0.1 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let mut calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo );
	/// calc_b *= 4.0;
	///
	/// assert_eq!( calc_b, Num::new( 8.0 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn mul_assign( &mut self, rhs: f64 ) {
		self.mantissa *= rhs;
	}
}

impl Div for Num {
	type Output = Self;

	/// The multiplication operator `/`. The resulting `Num` will keep the higher prefix of the two parts.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) / Num::new( 0.1 );
	///
	/// assert_eq!( calc_a, Num::new( 10.0 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) / Num::new( 4.0 );
	///
	/// assert_eq!( calc_b, Num::new( 0.5 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn div( self, other: Self ) -> Self::Output {
		let val = self.as_f64() / other.as_f64();
		let pref = self.prefix().max( other.prefix() );

		Self::new( val ).to_prefix( pref )
	}
}

impl Div<f64> for Num {
	type Output = Self;

	/// The multiplication operator `/`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Prefix};
	/// let calc_a = Num::new( 1.0 ) / 0.1;
	///
	/// assert_eq!( calc_a, Num::new( 10.0 ) );
	/// assert_eq!( calc_a.prefix(), Prefix::Nothing );
	///
	/// let calc_b = Num::new( 2.0 ).with_prefix( Prefix::Kilo ) / 4.0;
	///
	/// assert_eq!( calc_b, Num::new( 0.5 ).with_prefix( Prefix::Kilo ) );
	/// assert_eq!( calc_b.prefix(), Prefix::Kilo );
	/// ```
	fn div( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() / other;

		Self::new( val ).to_prefix( self.prefix() )
	}
}

impl Neg for Num {
	type Output = Self;

	fn neg( self ) -> Self::Output {
		let val = -self.as_f64();

		Self::new( val ).to_prefix( self.prefix() )
	}
}

impl From<f64> for Num {
	/// Creates a new `Num` from `item`. This is identical to `Num::new()`.
	///
	/// # Example
	/// ```
	/// # use sinum::Num;
	/// assert_eq!( Num::from( 9999.9 ), Num::new( 9999.9 ) );
	/// assert_eq!( Num::from( 99999.9 ), Num::new( 99999.9 ) );
	/// ```
	fn from( item: f64 ) -> Self {
		Self {
			mantissa: item,
			prefix: Prefix::Nothing,
		}
	}
}

impl fmt::Display for Num {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		// Avoiding print output like "0.100000000012".
		let mantissa_rounded = ( self.mantissa * 1e6 ).round() / 1e6;

		match self.prefix {
			Prefix::Nothing => write!( f, "{}", mantissa_rounded ),
			_ => write!( f, "{} {}", mantissa_rounded, self.prefix.to_string_sym() )
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn sinum_string() {
		assert_eq!( Num::new( 9999.9 ).to_string(), "9999.9".to_string() );
		assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Mega ).to_string(), "9999.9 M".to_string() );
		assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Milli ).to_string(), "9999.9 m".to_string() );
		assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Mega ).to_prefix( Prefix::Milli ).to_string(), "9999900000000 m".to_string() );
	}

	#[test]
	fn sinum_string_engineering() {
		assert_eq!( Num::new( 9999.9 ).to_string_eng(), "9999.9".to_string() );
		assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Mega ).to_string_eng(), "9999.9×10^6".to_string() );
		assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Milli ).to_string_eng(), "9999.9×10^-3".to_string() );
		assert_eq!( Num::new( 9999.9 ).with_prefix( Prefix::Mega ).to_prefix( Prefix::Milli ).to_string_eng(), "9999900000000×10^-3".to_string() );
	}
}
