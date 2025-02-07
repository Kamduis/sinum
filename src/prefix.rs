//! The SI unit prefixes.




//=============================================================================
// Crates


use std::fmt;
use std::str::FromStr;

#[cfg( feature = "i18n" )] use fluent_templates::Loader;
use thiserror::Error;
#[cfg( feature = "i18n" )] use unic_langid::LanguageIdentifier;

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};

#[cfg( feature = "i18n" )] use crate::DisplayLocale;
#[cfg( feature = "tex" )] use crate::{Latex, LatexSym};
#[cfg( all( feature = "i18n", feature = "tex" ) )] use crate::LatexLocale;
#[cfg( feature = "tex" )] use crate::TexOptions;
#[cfg( feature = "i18n" )] use crate::LOCALES;




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
	Quecto,
	Ronto,
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
	Ronna,
	Quetta,
}

impl Prefix {
	/// Larges exponent representable by `Self`.
	pub const MAX_EXP: i8 = 30;

	/// Smalles exponent representable by `Self`.
	pub const MIN_EXP: i8 = -30;

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
			Self::Quecto => 1e-30,
			Self::Ronto => 1e-27,
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
			Self::Ronna => 1e27,
			Self::Quetta => 1e30,
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
			Self::Quecto => -30,
			Self::Ronto =>  -27,
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
			Self::Ronna =>   27,
			Self::Quetta =>  30,
		}
	}

	/// Returns `self` as symbol string. While `to_string()` returns the name of the unit prefix, this returns the prexif letter as it is written in front of the unit symbol.
	pub fn to_string_sym( &self ) -> String {
		let res = match self {
			Self::Quecto =>  "q",
			Self::Ronto =>   "r",
			Self::Yocto =>   "y",
			Self::Zepto =>   "z",
			Self::Atto =>    "a",
			Self::Femto =>   "f",
			Self::Pico =>    "p",
			Self::Nano =>    "n",
			Self::Micro =>   "µ",
			Self::Milli =>   "m",
			Self::Centi =>   "c",
			Self::Deci =>    "d",
			Self::Nothing => "",
			Self::Deca =>    "da",
			Self::Hecto =>   "h",
			Self::Kilo =>    "k",
			Self::Mega =>    "M",
			Self::Giga =>    "G",
			Self::Tera =>    "T",
			Self::Peta =>    "P",
			Self::Exa =>     "E",
			Self::Zetta =>   "Z",
			Self::Yotta =>   "Y",
			Self::Ronna =>   "R",
			Self::Quetta =>  "Q",
		};

		res.to_string()
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
			-30 => Self::Quecto,
			-27 => Self::Ronto,
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
			27  => Self::Ronna,
			30  => Self::Quetta,
			_ => return Err( PrefixError::TryFromExp( item ) ),
		};

		Ok( res )
	}
}

impl FromStr for Prefix {
	type Err = PrefixError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let result = match s.to_lowercase().as_str() {
			"quecto"  => Self::Quecto,
			"ronto"   => Self::Ronto,
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
			"ronna"   => Self::Ronna,
			"quetta"  => Self::Quetta,
			_ => return Err( PrefixError::TryFromStr( s.to_string() ) ),
		};

		Ok( result )
	}
}

impl fmt::Display for Prefix {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		let res = match self {
			Self::Quecto =>  "quecto",
			Self::Ronto =>   "ronto",
			Self::Yocto =>   "yocto",
			Self::Zepto =>   "zepto",
			Self::Atto =>    "atto",
			Self::Femto =>   "femto",
			Self::Pico =>    "pico",
			Self::Nano =>    "nano",
			Self::Micro =>   "micro",
			Self::Milli =>   "milli",
			Self::Centi =>   "centi",
			Self::Deci =>    "deci",
			Self::Nothing => "",
			Self::Deca =>    "deca",
			Self::Hecto =>   "hecto",
			Self::Kilo =>    "kilo",
			Self::Mega =>    "mega",
			Self::Giga =>    "giga",
			Self::Tera =>    "tera",
			Self::Peta =>    "peta",
			Self::Exa =>     "exa",
			Self::Zetta =>   "zetta",
			Self::Yotta =>   "yotta",
			Self::Ronna =>   "ronna",
			Self::Quetta =>  "quetta",
		};

		write!( f, "{}", res )
	}
}

#[cfg( feature = "i18n" )]
impl DisplayLocale for Prefix {
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		match self {
			Self::Quecto =>  LOCALES.lookup( locale, "quecto" ),
			Self::Ronto =>   LOCALES.lookup( locale, "ronto" ),
			Self::Yocto =>   LOCALES.lookup( locale, "yocto" ),
			Self::Zepto =>   LOCALES.lookup( locale, "zepto" ),
			Self::Atto =>    LOCALES.lookup( locale, "atto" ),
			Self::Femto =>   LOCALES.lookup( locale, "femto" ),
			Self::Pico =>    LOCALES.lookup( locale, "pico" ),
			Self::Nano =>    LOCALES.lookup( locale, "nano" ),
			Self::Micro =>   LOCALES.lookup( locale, "micro" ),
			Self::Milli =>   LOCALES.lookup( locale, "milli" ),
			Self::Centi =>   LOCALES.lookup( locale, "centi" ),
			Self::Deci =>    LOCALES.lookup( locale, "deci" ),
			Self::Nothing => "".to_string(),
			Self::Deca =>    LOCALES.lookup( locale, "deca" ),
			Self::Hecto =>   LOCALES.lookup( locale, "hecto" ),
			Self::Kilo =>    LOCALES.lookup( locale, "kilo" ),
			Self::Mega =>    LOCALES.lookup( locale, "mega" ),
			Self::Giga =>    LOCALES.lookup( locale, "giga" ),
			Self::Tera =>    LOCALES.lookup( locale, "tera" ),
			Self::Peta =>    LOCALES.lookup( locale, "peta" ),
			Self::Exa =>     LOCALES.lookup( locale, "exa" ),
			Self::Zetta =>   LOCALES.lookup( locale, "zetta" ),
			Self::Yotta =>   LOCALES.lookup( locale, "yotta" ),
			Self::Ronna =>   LOCALES.lookup( locale, "ronna" ),
			Self::Quetta =>  LOCALES.lookup( locale, "quetta" ),
		}
	}
}

#[cfg( feature = "tex" )]
impl Latex for Prefix {}

#[cfg( all( feature = "i18n", feature = "tex" ) )]
impl LatexLocale for Prefix {}

#[cfg( feature = "tex" )]
impl LatexSym for Prefix {
	/// Return a string that represents this `Prefix` as LaTeX command (requiring the usage of the `{siunitx}` package in LaTeX).
	///
	/// # Example
	/// ```
	/// # use sinum::LatexSym;
	/// # use sinum::{Prefix, TexOptions};
	/// assert_eq!( Prefix::Femto.to_latex_sym( &TexOptions::none() ), r"\femto".to_string() );
	/// assert_eq!( Prefix::Nothing.to_latex_sym( &TexOptions::none() ), "".to_string() );
	/// assert_eq!( Prefix::Giga.to_latex_sym( &TexOptions::none() ), r"\giga".to_string() );
	/// ```
	fn to_latex_sym( &self, _options: &TexOptions ) -> String {
		match self {
			Self::Quecto =>  r"\quecto".to_string(),
			Self::Ronto =>   r"\ronto".to_string(),
			Self::Yocto =>   r"\yocto".to_string(),
			Self::Zepto =>   r"\zepto".to_string(),
			Self::Atto =>    r"\atto".to_string(),
			Self::Femto =>   r"\femto".to_string(),
			Self::Pico =>    r"\pico".to_string(),
			Self::Nano =>    r"\nano".to_string(),
			Self::Micro =>   r"\micro".to_string(),
			Self::Milli =>   r"\milli".to_string(),
			Self::Centi =>   r"\centi".to_string(),
			Self::Deci =>    r"\deca".to_string(),
			Self::Nothing => "".to_string(),
			Self::Deca =>    r"\deca".to_string(),
			Self::Hecto =>   r"\hecto".to_string(),
			Self::Kilo =>    r"\kilo".to_string(),
			Self::Mega =>    r"\mega".to_string(),
			Self::Giga =>    r"\giga".to_string(),
			Self::Tera =>    r"\tera".to_string(),
			Self::Peta =>    r"\peta".to_string(),
			Self::Exa =>     r"\exa".to_string(),
			Self::Zetta =>   r"\zetta".to_string(),
			Self::Yotta =>   r"\yotta".to_string(),
			Self::Ronna =>   r"\ronna".to_string(),
			Self::Quetta =>  r"\quetta".to_string(),
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
		assert_eq!( Prefix::Peta.to_string(), "peta".to_string() );
		assert_eq!( Prefix::Peta.to_string_sym(), "P".to_string() );
		assert_eq!( Prefix::Femto.to_string(), "femto".to_string() );
		assert_eq!( Prefix::Femto.to_string_sym(), "f".to_string() );
	}
}
