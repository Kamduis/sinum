//! Provides quantities representing numbers combined with the SI prefix and unit system.




//=============================================================================
// Crates


use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, MulAssign, Div, Neg};

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};

#[cfg( feature = "tex" )]
use crate::{Latex, LatexSym};
#[cfg( feature = "tex" )]
use crate::TexOptions;

use crate::prefix::PrefixError;
use crate::unit::UnitError;
use crate::{Num, Prefix, Unit, PhysicalQuantity};




//=============================================================================
// Structs


/// Represents a number in combination with a SI prefix.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Debug )]
pub struct Qty {
	number: Num,
	unit: Unit,
}

impl Qty {
	/// Create a new `Qty` representing a numeric value and a unit.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Unit};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).as_f64(), 9.9 );
	/// assert_eq!( Qty::new( 99.9.into(), &Unit::Kelvin ).as_f64(), 99.9 );
	/// ```
	pub fn new( number: Num, unit: &Unit ) -> Self {
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
			_ => ( number, unit.clone() ),
		};

		Self {
			number: num,
			unit: uni,
		}
	}

	/// Creates a new `Qty` from `self` with a reduced numbers of digits of the mantissa (see `mantissa()`) required to represent the number:
	///
	/// * No more than 3 digits in front of the decimal point.
	///     (1234 s → 1.234 ks)
	///
	/// * No zero in front of the decimal point.
	///     (0.001 A → 1.0 mA)
	///
	/// This function will only modify the prefix, never the unit itself. (see `sorten_unit()`).
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// assert_eq!(
	///     Qty::new( 1000.0.into(), &Unit::Ampere ).shortened().unwrap(),
	///     Qty::new( Num::new( 1.0.into() ).with_prefix( Prefix::Kilo ), &Unit::Ampere )
	/// );
	/// assert_eq!(
	///     Qty::new( 0.001.into(), &Unit::Candela ).shortened().unwrap(),
	///     Qty::new( Num::new( 1.0 ).with_prefix( Prefix::Milli ), &Unit::Candela )
	/// );
	/// ```
	pub fn shortened( self ) -> Result<Self, PrefixError> {
		let num = self.number.shortened()?;

		Ok( Self::new( num, self.unit() ) )
	}

	/// Returns the numeric value of the `Qty` without any prefix or unit.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Unit};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).as_f64(), 9.9 );
	/// assert_eq!( Qty::new( 99.9.into(), &Unit::Kelvin ).as_f64(), 99.9 );
	/// ```
	pub fn as_f64( &self ) -> f64 {
		self.number.as_f64() * self.unit.factor()
	}

	/// Returns the numeric `Num` of the `Qty`.
	///
	/// # Example
	/// ```
	/// # use sinum::{Num, Qty, Unit};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).number(), Num::new( 9.9 ) );
	/// ```
	pub fn number( &self ) -> Num {
		self.number
	}

	/// Returns the unit of the `Qty`.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Unit};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).unit(), &Unit::Ampere );
	/// ```
	pub fn unit( &self ) -> &Unit {
		&self.unit
	}

	/// Returns the physical quantity that is represented by the `Qty`.
	fn phys( &self ) -> PhysicalQuantity {
		self.unit.phys()
	}

	/// Creates a new `Qty` from `self` at the specified `prefix`.
	///
	/// The numeric value of the new `Qty` will be identical to `self` (apart from possible floating point rounding errors) since the mantissa is being modified alongside the prefix to reflect the same numeric value as before.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Prefix, Unit};
	/// let qty = Qty::new( 2.0.into(), &Unit::Meter );
	///
	/// assert_eq!( qty.as_f64(), 2.0 );
	/// assert_eq!( qty.clone().to_prefix( Prefix::Milli ).as_f64(), 2.0 );
	/// assert_eq!( qty.clone().to_prefix( Prefix::Milli ).number().mantissa(), 2_000.0 );
	///
	/// assert_eq!( qty.clone().to_prefix( Prefix::Kilo ).as_f64(), 2.0 );
	/// assert_eq!( qty.to_prefix( Prefix::Kilo ).number().mantissa(), 0.002 );
	/// ```
	pub fn to_prefix( self, prefix: Prefix ) -> Self {
		let number = self.number.to_prefix( prefix );
		Self::new( number, &self.unit )
	}

	/// Returns a new `Qty` from `self` with the new `unit`.
	///
	/// If `unit` does not represent the same physical quantity as the original unit, this function returns an `UnitError`.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Unit};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).to_unit( &Unit::Gram ).unwrap(), Qty::new( 9.9e3.into(), &Unit::Gram ) );
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).to_unit( &Unit::Tonne ).unwrap(), Qty::new( 0.0099.into(), &Unit::Tonne ) );
	/// assert!( Qty::new( 9.9.into(), &Unit::Kilogram ).to_unit( &Unit::Second ).is_err() );
	/// ```
	pub fn to_unit( &self, unit: &Unit ) -> Result<Self, UnitError> {
		if self.phys() != unit.phys() {
			return Err( UnitError::UnitMismatch( vec![ self.unit().clone(), unit.clone() ] ) );
		};

		let factor_old = self.unit().factor();
		let factor_new = unit.factor();
		let factor = factor_old / factor_new;
		let num_new = self.number() * factor;

		Ok( Self::new( num_new, unit ) )
	}

	/// Computes the absolute value of `self` with respect to the base unit. This means 10.0 t are returned as 10e3.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let x = Qty::new( 3.5.into(), &Unit::Ampere );
	/// let y = Qty::new( Num::from( -3.5 ), &Unit::Ampere );
	///
	/// let abs_difference_x = ( x.clone().abs() - x ).abs();
	/// let abs_difference_y = ( y.clone().abs() - ( -y ) ).abs();
	///
	/// assert!( abs_difference_x < 1e-10 );
	/// assert!( abs_difference_y < 1e-10 );
	/// ```
	pub fn abs( self ) -> Self {
		let val = self.as_f64().abs();
		Self::new( Num::new( val ).to_prefix( self.number.prefix() ), self.unit() )
	}

	/// Returns a string representation of the quantity with engineering notation.
	/// Engineering notation is similar to scientific notation (using exponents of ten) but the exponents are always a multiple of 3.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let x = Qty::new( Num::new( 2.0 ).with_prefix( Prefix::Milli ), &Unit::Ampere );
	///
	/// assert_eq!( x.to_string_eng(), "2×10^-3 A" );
	/// ```
	pub fn to_string_eng( &self ) -> String {
		format!( "{} {}", self.number.to_string_eng(), self.unit.to_string_sym() )
	}

	/// Returns a LaTeX string representation of the quantity with engineering notation.
	/// Engineering notation is similar to scientific notation (using exponents of ten) but the exponents are always a multiple of 3.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix, TexOptions};
	/// let x = Qty::new( Num::new( 2.0 ).with_prefix( Prefix::Milli ), &Unit::Ampere );
	///
	/// assert_eq!( x.to_latex_eng( &TexOptions::new() ), r"\qty{2e-3}{\ampere}" );
	/// ```
	#[cfg( feature = "tex" )]
	pub fn to_latex_eng( &self, options: &TexOptions ) -> String {
		if let Prefix::Nothing = self.number.prefix() {
			return self.to_latex_sym( options );
		}

		let mantissa = match options.minimum_decimal_digits {
			Some( x ) => format!( "{:.1$}", self.number.mantissa(), x as usize ),
			None => self.number.mantissa().to_string(),
		};
		format!(
			r"\qty{}{{{}e{}}}{{{}}}",
			options,
			mantissa,
			self.number.prefix().exp(),
			self.unit.to_latex_sym( options )
		)
	}
}

impl PartialEq for Qty {
	/// Compares two `Qty`s for equality. It compares that the numeric value is identical, not the representation.
	/// 1 Mg == 1000 kg == 1 t
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Prefix, Unit};
	/// assert_eq!( Qty::new( 1.1.into(), &Unit::Ampere ), Qty::new( 1.1.into(), &Unit::Ampere ) );
	///
	/// let val_a = Qty::new( Num::new( 1.0 ).with_prefix( Prefix::Mega ), &Unit::Gram );
	/// let val_b = Qty::new( Num::new( 1000.0 ), &Unit::Kilogram );
	/// let val_c = Qty::new( Num::new( 1.0 ), &Unit::Tonne );
	/// assert!( val_a == val_b );
	/// assert!( val_a == val_c );
	/// assert!( val_b == val_c );
	/// ```
	fn eq( &self, other: &Qty ) -> bool {
		if self.phys() != other.phys() {
			return false;
		}

		self.as_f64().eq( &other.as_f64() )
	}
}

impl PartialEq<f64> for Qty {
	/// Compares a `Qty` and a `f64` for equality.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Prefix, Unit};
	/// assert!( Qty::new( 1.1.into(), &Unit::Ampere ) == 1.1 );
	/// assert!( Qty::new( Num::new( 2.0 ).with_prefix( Prefix::Kilo ), &Unit::Second ) == 2e3 );
	/// ```
	fn eq( &self, other: &f64 ) -> bool {
		self.as_f64().eq( other )
	}
}

impl PartialOrd for Qty {
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

impl PartialOrd<f64> for Qty {
	fn partial_cmp( &self, other: &f64 ) -> Option<Ordering> {
		self.as_f64().partial_cmp( other )
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

impl Add for Qty {
	type Output = Self;

	/// The addition operator `+`. The resulting `Qty` will keep the prefix and unit of `self`.
	///
	/// **Note:** Adding two `Qty`s representing different physical quantities results in a **panic**.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) + Qty::new( 0.1.into(), &Unit::Ampere );
	///
	/// assert_eq!( calc_a, Qty::new( 1.1.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) + Qty::new( 4.0.into(), &Unit::Tonne );
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( 4_000_000_008.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn add( self, other: Self ) -> Self::Output {
		let val = self.as_f64() + other.as_f64();

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Add<f64> for Qty {
	type Output = Self;

	/// The addition operator `+`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) + 0.1;
	///
	/// assert_eq!( calc_a, Qty::new( 1.1.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 32.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) + 4.0;
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( 4_000_032.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn add( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() + other;

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Sub for Qty {
	type Output = Self;

	/// The subtraction operator `-`. The resulting `Qty` will keep the prefix and unit of `self`.
	///
	/// **Note:** Subtracting two `Qty`s representing different physical quantities results in a **panic**.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) - Qty::new( 0.1.into(), &Unit::Ampere );
	///
	/// assert_eq!( calc_a, Qty::new( 0.9.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) - Qty::new( 4.0.into(), &Unit::Tonne );
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( -3_999_999_992.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn sub( self, other: Self ) -> Self::Output {
		let val = self.as_f64() - other.as_f64();

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Sub<f64> for Qty {
	type Output = Self;

	/// The subtraction operator `-`. The resulting `Num` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) - 0.1;
	///
	/// assert_eq!( calc_a, Qty::new( 0.9.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) - 4.0;
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( -3_999_992.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn sub( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() - other;

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Mul for Qty {
	type Output = Self;

	/// The multiplication operator `*`. The resulting `Num` will keep the prefix and unit of `self`.
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

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Mul<f64> for Qty {
	type Output = Self;

	/// The multiplication operator `*`. The resulting `Qty` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) * 0.1;
	///
	/// assert_eq!( calc_a, Qty::new( 0.1.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) * 4.0;
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( 32.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn mul( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() * other;

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl MulAssign<f64> for Qty {
	/// The multiplication operator `*=`. `self` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let mut calc_a = Qty::new( 1.0.into(), &Unit::Ampere );
	/// calc_a *= 0.1;
	///
	/// assert_eq!( calc_a, Qty::new( 0.1.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let mut calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram );
	/// calc_b *= 4.0;
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( 32.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn mul_assign( &mut self, rhs: f64 ) {
		self.number *= rhs;
	}
}

impl Div for Qty {
	type Output = Self;

	/// The multiplication operator `/`. The resulting `Qty` will keep the higher prefix of the two parts.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) / Qty::new( 0.1.into(), &Unit::Ampere );
	///
	/// assert_eq!( calc_a, Qty::new( 10.0.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) / Qty::new( 4.0.into(), &Unit::Tonne );
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( 2e-3 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn div( self, other: Self ) -> Self::Output {
		let val = self.as_f64() / other.as_f64();

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Div<f64> for Qty {
	type Output = Self;

	/// The multiplication operator `/`. The resulting `Qty` will keep the prefix.
	///
	/// # Example
	/// ```
	/// # use sinum::{Qty, Num, Unit, Prefix};
	/// let calc_a = Qty::new( 1.0.into(), &Unit::Ampere ) / 0.1;
	///
	/// assert_eq!( calc_a, Qty::new( 10.0.into(), &Unit::Ampere ) );
	/// assert_eq!( calc_a.number().prefix(), Prefix::Nothing );
	/// assert_eq!( calc_a.unit(), &Unit::Ampere );
	///
	/// let calc_b = Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) / 4.0;
	///
	/// assert_eq!( calc_b, Qty::new( Num::new( 2.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ) );
	/// assert_eq!( calc_b.number().prefix(), Prefix::Milli );
	/// ```
	fn div( self, other: f64 ) -> Self::Output {
		let val = self.as_f64() / other;

		Self::new( val.into(), &self.unit.base() )
			.to_unit( &self.unit ).unwrap()
			.to_prefix( self.number.prefix() )
	}
}

impl Neg for Qty {
	type Output = Self;

	fn neg( self ) -> Self::Output {
		let val = -self.as_f64();
		let num = Num::new( val ).to_prefix( self.number.prefix() );

		Self::new( num, &self.unit.base() ).to_unit( &self.unit ).unwrap()
	}
}

impl fmt::Display for Qty {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self.number.prefix() {
			Prefix::Nothing => write!( f, "{} {}", self.number, self.unit.to_string_sym() ),
			_ => write!( f, "{}{}", self.number, self.unit.to_string_sym() ),
		}
	}
}

#[cfg( feature = "tex" )]
impl Latex for Qty {
	/// Return a string that represents this `Qty` as LaTeX string.
	fn to_latex( &self, options: &TexOptions ) -> String {
		self.to_latex_sym( options )
	}
}

#[cfg( feature = "tex" )]
impl LatexSym for Qty {
	/// Return a string that represents this `Qty` as LaTeX command (requiring the usage of the `{siunitx}` package in LaTeX).
	///
	/// # Example
	/// ```
	/// # use sinum::LatexSym;
	/// # use sinum::{Qty, Unit, Num, Prefix, TexOptions};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).to_latex_sym( &TexOptions::none() ), r"\qty{9.9}{\ampere}".to_string() );
	/// assert_eq!(
	///     Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Ampere ).to_latex_sym( &TexOptions::none() ),
	///     r"\qty{9.9}{\milli\ampere}".to_string()
	/// );
	/// ```
	///
	/// # Kilogram
	///
	/// The base unit for mass, the kilogram is a special case, since it already has a prefix (kilo), that has to be taken into account.
	/// ```
	/// # use sinum::LatexSym;
	/// # use sinum::{Qty, Unit, Num, Prefix, TexOptions};
	/// assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\kilogram}".to_string() );
	/// assert_eq!(
	///     Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ),
	///     r"\qty{9.9}{\mega\gram}".to_string()
	/// );
	/// assert_eq!(
	///     Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Kilogram ).to_latex_sym(
	///         &TexOptions::new()
	///             .minimum_decimal_digits( 1 )
	///     ),
	///     r"\qty{9.9}{\gram}".to_string()
	/// );
	/// assert_eq!(
	///     Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Micro ), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ),
	///     r"\qty{9.9}{\milli\gram}".to_string()
	/// );
	/// assert_eq!( Qty::new(
	///     Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Gram ).to_latex_sym( &TexOptions::new() ),
	///     r"\qty{9.9}{\milli\gram}".to_string()
	/// );
	/// assert_eq!(
	///     Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Gram ).to_latex_sym( &TexOptions::new() ),
	///     r"\qty{9.9}{\kilogram}".to_string()
	/// );
	/// ```
	fn to_latex_sym( &self, options: &TexOptions ) -> String {
		let mantissa = match options.minimum_decimal_digits {
			Some( x ) => format!( "{:.1$}", self.number.mantissa(), x as usize ),
			None => self.number.mantissa().to_string(),
		};
		format!(
			r"\qty{}{{{}}}{{{}{}}}",
			options,
			mantissa,
			self.number.prefix().to_latex_sym( options ),
			self.unit.to_latex_sym( options )
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
	fn qty_eq() {
		assert!( Qty::new( 10e3.into(), &Unit::Kilogram ) == Qty::new( 10.0.into(), &Unit::Tonne ) );
	}

	#[test]
	fn siqty_as_f64() {
		// `as_f64()` returns the value with regard to the base unit.
		assert_eq!( Qty::new( 9.9.into(), &Unit::Tonne ).as_f64(), 9.9e3 );
		assert_eq!( Qty::new( Num::new( 8.0 ).with_prefix( Prefix::Milli ), &Unit::Gram ).as_f64(), 8.0e-6 );
	}

	#[test]
	fn siqty_string() {
		assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).to_string(), "9.9 A".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Meter ).to_string(), "9.9 km".to_string() );
		assert_eq!( Qty::new( 9.9.into(), &Unit::Kelvin ).to_string(), "9.9 K".to_string() );
	}

	// The weight/mass is a special case.
	#[test]
	fn siqty_kilogram() {
		assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).as_f64(), 9.9 );
		assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).number(), Num::new( 9.9 ) );
		assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).to_string(), "9.9 kg".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Kilogram ).to_string(), "9.9 Mg".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Kilogram ).to_string(), "9.9 g".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Micro ), &Unit::Kilogram ).to_string(), "9.9 mg".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Gram ).to_string(), "9.9 mg".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Gram ).to_string(), "9.9 kg".to_string() );
	}

	#[cfg( feature = "tex" )]
	#[test]
	fn siqty_latex_kilogram() {
		assert_eq!( Qty::new( 9.9.into(), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\kilogram}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\mega\gram}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\gram}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Micro ), &Unit::Kilogram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\milli\gram}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Gram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\milli\gram}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Gram ).to_latex_sym( &TexOptions::new() ), r"\qty{9.9}{\kilogram}".to_string() );
	}

	#[test]
	fn qty_string_engineering() {
		assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).to_string_eng(), "9.9 A".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Meter ).to_string_eng(), "9.9×10^3 m".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Kelvin ).to_string_eng(), "9.9×10^-3 K".to_string() );
	}

	#[cfg( feature = "tex" )]
	#[test]
	fn qty_latex_engineering() {
		assert_eq!( Qty::new( 9.9.into(), &Unit::Ampere ).to_latex_eng( &TexOptions::new() ), r"\qty{9.9}{\ampere}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Kilo ), &Unit::Meter ).to_latex_eng( &TexOptions::new() ), r"\qty{9.9e3}{\meter}".to_string() );
		assert_eq!( Qty::new( Num::new( 9.9 ).with_prefix( Prefix::Milli ), &Unit::Kelvin ).to_latex_eng( &TexOptions::new() ), r"\qty{9.9e-3}{\kelvin}".to_string() );
	}
}
