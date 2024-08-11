//! The SI units.




//=============================================================================
// Crates


use std::fmt;
use std::str::FromStr;

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[cfg( feature = "i18n" )] use fluent_templates::Loader;
#[cfg( feature = "i18n" )] use unic_langid::LanguageIdentifier;

#[cfg( feature = "tex" )]
use crate::{Latex, LatexSym};
#[cfg( feature = "tex" )]
use crate::TexOptions;

#[cfg( feature = "i18n" )] use crate::DisplayLocale;
#[cfg( feature = "i18n" )] use crate::LOCALES;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum UnitError {
	#[error( "Not all units represent the same physical quantity: {}", .0.iter().map( |x| x.to_string() ).collect::<Vec<String>>().join( ", " ) )]
	UnitMismatch( Vec<Unit> ),

	#[error( "Not a valid unit: {0}" )]
	ParseFailure( String ),
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
	Pressure,
	Radiation,
}

// impl PhysicalQuantity {
// 	/// Returns the available units for this `PhysicalQuantity` and the factor to the base unit.
// 	pub(super) fn units( &self ) -> BTreeSet<Unit> {
// 		match self {
// 			Self::Custom => BTreeSet::new(),
// 			Self::Current => BTreeSet::from( [
// 				Unit::Ampere,
// 			] ),
// 			Self::LuminousIntensity => BTreeSet::from( [
// 				Unit::Candela,
// 			] ),
// 			Self::Temperature => BTreeSet::from( [
// 				Unit::Kelvin,
// 			] ),
// 			Self::Mass => BTreeSet::from( [
// 				Unit::Gram,
// 				Unit::Kilogram,
// 				Unit::Tonne,
// 			] ),
// 			Self::Length => BTreeSet::from( [
// 				Unit::Meter,
// 			] ),
// 			Self::Amount => BTreeSet::from( [
// 				Unit::Mole,
// 			] ),
// 			Self::Time => BTreeSet::from( [
// 				Unit::Second,
// 			] ),
// 			Self::Radiation => BTreeSet::from( [
// 				Unit::Sievert,
// 			] ),
// 		}
// 	}
// }

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
	// Additional length units
	AstronomicalUnit,
	Lightyear,
	Parsec,
	//
	Pascal,
	Bar,
	Sievert,
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
			Self::Kilogram | Self::Gram | Self::Tonne => PhysicalQuantity::Mass,
			Self::Meter |
				Self::AstronomicalUnit |
				Self::Lightyear |
				Self::Parsec => PhysicalQuantity::Length,
			Self::Mole =>      PhysicalQuantity::Amount,
			Self::Second =>    PhysicalQuantity::Time,
			Self::Pascal | Self::Bar => PhysicalQuantity::Pressure,
			Self::Sievert =>   PhysicalQuantity::Radiation,
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
				Self::Second |
				Self::Pascal |
				Self::Sievert => 1.0,
			Self::Gram => 1e-3,
			Self::Tonne => 1e3,
			Self::AstronomicalUnit => 149_597_870_700.0,
			Self::Lightyear => 9_460_730_472_580_800.0,
			Self::Parsec => 30.85677581e15,
			Self::Bar => 1e5,
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
			Self::AstronomicalUnit | Self::Lightyear | Self::Parsec => Self::Meter,
			//
			Self::Pascal =>    Self::Pascal,
			Self::Bar =>       Self::Pascal,
			Self::Sievert =>   Self::Sievert,
		}
	}

	/// Returns the symbol representing `self` as unit.
	///
	/// # Example
	/// ```
	/// # use sinum::Unit;
	/// assert_eq!( Unit::Meter.to_string_sym(), "m".to_string() );
	/// assert_eq!( Unit::Second.to_string_sym(), "s".to_string() );
	/// ```
	pub fn to_string_sym( &self ) -> String {
		let res = match self {
			Self::Custom( x ) => x,
			// Base units
			Self::Ampere =>    "A",
			Self::Candela =>   "cd",
			Self::Kelvin =>    "K",
			Self::Kilogram =>  "kg",
			Self::Meter =>     "m",
			Self::Mole =>      "mol",
			Self::Second =>    "s",
			// Additional mass units
			Self::Gram =>      "g",
			Self::Tonne =>     "t",
			// Additional length units
			Self::AstronomicalUnit => "AU",
			Self::Lightyear => "ly",
			Self::Parsec =>    "pc",
			//
			Self::Pascal =>    "Pa",
			Self::Bar =>       "bar",
			Self::Sievert =>   "Sv",
		};

		res.to_string()
	}
}

impl FromStr for Unit {
	type Err = UnitError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let result = match s.to_lowercase().as_str() {
			"ampere" | "a" => Self::Ampere,
			"candela" | "cd" => Self::Candela,
			"kelvin" | "k" => Self::Kelvin,
			"kilogram" | "kg" => Self::Kilogram,
			"meter" | "m" => Self::Meter,
			"mole" | "mol" => Self::Mole,
			"second" | "s" => Self::Second,
			"gram" | "g" => Self::Gram,
			"tonne" | "t" => Self::Tonne,
			"astronomical unit" | "au" => Self::AstronomicalUnit,
			"lightyear" | "ly" => Self::Lightyear,
			"parsec" | "pc" => Self::Parsec,
			"pascal" | "pa" => Self::Pascal,
			"bar" => Self::Bar,
			"sievert" | "sv" => Self::Sievert,
			_ => return Err( UnitError::ParseFailure( s.to_string() ) ),
		};

		Ok( result )
	}
}

impl fmt::Display for Unit {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			Self::Custom( x ) => write!( f, "{}", x ),
			// Base units
			Self::Ampere =>    write!( f, "ampere" ),
			Self::Candela =>   write!( f, "candela" ),
			Self::Kelvin =>    write!( f, "kelvin" ),
			Self::Kilogram =>  write!( f, "kilogram" ),
			Self::Meter =>     write!( f, "meter" ),
			Self::Mole =>      write!( f, "mol" ),
			Self::Second =>    write!( f, "second" ),
			// Additional mass units
			Self::Gram =>      write!( f, "gram" ),
			Self::Tonne =>     write!( f, "tonne" ),
			// Additional length units
			Self::AstronomicalUnit => write!( f, "astronomical unit" ),
			Self::Lightyear => write!( f, "lightyear" ),
			Self::Parsec =>    write!( f, "parsec" ),
			//
			Self::Pascal =>    write!( f, "pascal" ),
			Self::Bar =>       write!( f, "bar" ),
			Self::Sievert =>   write!( f, "sievert" ),
		}
	}
}

#[cfg( feature = "i18n" )]
impl DisplayLocale for Unit {
	/// Representing unit as string, translating the unit into the language specified by `locale`.
	///
	/// # Example
	///
	/// ```
	/// use unic_langid::LanguageIdentifier;
	/// use unic_langid::langid;
	/// use sinum::Unit;
	///
	/// const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
	/// const GERMAN: LanguageIdentifier = langid!( "de-DE" );
	///
	/// assert_eq!( Unit::Ampere.to_string_locale( &US_ENGLISH ), "ampere" );
	/// assert_eq!( Unit::Candela.to_string_locale( &US_ENGLISH ), "candela" );
	/// assert_eq!( Unit::AstronomicalUnit.to_string_locale( &US_ENGLISH ), "astronomical unit" );
	/// assert_eq!( Unit::AstronomicalUnit.to_string_locale( &GERMAN ), "Astronomische Einheit" );
	/// assert_eq!( Unit::Ampere.to_string_locale( &GERMAN ), "Amper" );
	/// assert_eq!( Unit::Candela.to_string_locale( &GERMAN ), "Candela" );
	/// ```
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		match self {
			// Base units
			Self::Ampere =>    LOCALES.lookup( locale, "ampere" ),
			Self::Candela =>   LOCALES.lookup( locale, "candela" ),
			Self::Kelvin =>    LOCALES.lookup( locale, "kelvin" ),
			Self::Kilogram =>  LOCALES.lookup( locale, "kilogram" ),
			Self::Meter =>     LOCALES.lookup( locale, "meter" ),
			Self::Mole =>      LOCALES.lookup( locale, "mol" ),
			Self::Second =>    LOCALES.lookup( locale, "second" ),
			// Additional mass units
			Self::Gram =>      LOCALES.lookup( locale, "gram" ),
			Self::Tonne =>     LOCALES.lookup( locale, "tonne" ),
			// Additional length units
			Self::AstronomicalUnit => LOCALES.lookup( locale, "astronomical_unit" ),
			Self::Lightyear => LOCALES.lookup( locale, "lightyear" ),
			Self::Parsec =>    LOCALES.lookup( locale, "parsec" ),
			//
			Self::Pascal =>    LOCALES.lookup( locale, "pascal" ),
			Self::Bar =>       LOCALES.lookup( locale, "bar" ),
			Self::Sievert =>   LOCALES.lookup( locale, "sievert" ),
			//
			_ => self.to_string(),
		}
	}
}

#[cfg( feature = "tex" )]
impl Latex for Unit {
	/// Return the name of the unit as string. This is identical to `.to_string()`.
	///
	/// # Example
	/// ```
	/// # use sinum::Latex;
	/// # use sinum::{Unit, TexOptions};
	/// assert_eq!( Unit::Meter.to_latex( &TexOptions::none() ), "meter".to_string() );
	/// assert_eq!( Unit::Second.to_latex( &TexOptions::new() ), "second".to_string() );
	/// ```
	fn to_latex( &self, _options: &TexOptions ) -> String {
		self.to_string()
	}
}

#[cfg( feature = "tex" )]
impl LatexSym for Unit {
	/// Return a string that represents this `Unit` as LaTeX command (requiring the usage of the `{siunitx}` package in LaTeX).
	///
	/// # Example
	/// ```
	/// # use sinum::LatexSym;
	/// # use sinum::{Unit, TexOptions};
	/// assert_eq!( Unit::Meter.to_latex_sym( &TexOptions::none() ), r"\meter".to_string() );
	/// assert_eq!( Unit::Second.to_latex_sym( &TexOptions::new() ), r"\second".to_string() );
	/// ```
	fn to_latex_sym( &self, _options: &TexOptions ) -> String {
		match self {
			Self::Custom( x ) => x.clone(),
			// Base units
			Self::Ampere =>    r"\ampere".to_string(),
			Self::Candela =>   r"\candela".to_string(),
			Self::Kelvin =>    r"\kelvin".to_string(),
			Self::Kilogram =>  r"\kilogram".to_string(),
			Self::Meter =>     r"\meter".to_string(),
			Self::Mole =>      r"\mol".to_string(),
			Self::Second =>    r"\second".to_string(),
			// Additional mass units
			Self::Gram =>      r"\gram".to_string(),
			Self::Tonne =>     r"\tonne".to_string(),
			// Additional length units
			Self::AstronomicalUnit => r"\astronomicalunit".to_string(),
			Self::Lightyear => r"\lightyear".to_string(),
			Self::Parsec =>    r"\parsec".to_string(),
			//
			Self::Pascal =>    r"\pascal".to_string(),
			Self::Bar =>       r"\bar".to_string(),
			Self::Sievert =>   r"\sievert".to_string(),
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
	fn print_unit() {
		assert_eq!( Unit::Ampere.to_string(), "ampere".to_string() );
		assert_eq!( Unit::Ampere.to_string_sym(), "A".to_string() );
		assert_eq!( Unit::Candela.to_string(), "candela".to_string() );
		assert_eq!( Unit::Candela.to_string_sym(), "cd".to_string() );
	}
}
