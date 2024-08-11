//! Provides quantities representing numbers combined with the SI prefix and unit system.




//=============================================================================
// Crates


use std::fmt;

#[cfg( feature = "i18n" )] use unic_langid::LanguageIdentifier;

#[cfg( feature = "i18n" )] use crate::DisplayLocale;




//=============================================================================
// Traits


/// Providing conversion into LaTeX code.
///
/// This Trait is only available, if the **`tex`** feature has been enabled.
#[cfg( feature = "tex" )]
pub trait Latex {
	/// Converts the entity into a LaTeX-string.
	fn to_latex( &self, options: &TexOptions ) -> String;
}


/// Providing conversion into LaTeX code to print symbols instead of text. This is mostly implemented to print out prefixes and units like `\kilo\meter` or `\milli\ampere` (using the LaTeX package `{siunitx}` instead of words.
///
/// This Trait is only available, if the **`tex`** feature has been enabled.
#[cfg( feature = "tex" )]
pub trait LatexSym: Latex {
	/// Converts the entity into a LaTeX-string displaying symbols instead of written units.
	fn to_latex_sym( &self, options: &TexOptions ) -> String;
}


/// Providing a localized `.to_latex()`: `.to_latex_locale()`.
///
/// This Trait is only available, if the both, the **`i18n`** and the **`tex`** features have been enabled.
#[cfg( all( feature = "i18n", feature = "tex" ) )]
pub trait LatexLocale: DisplayLocale + Latex {
	/// Returns the localized LaTeX string representation of `self`.
	///
	/// The standard implementation ignores `locale` and returns the same string as `.to_latex()`.
	fn to_latex_locale( &self, _locale: &LanguageIdentifier, options: &TexOptions ) -> String {
		self.to_latex( options )
	}
}




//=============================================================================
// Structs


/// Representing options to LaTeX commands generated by `to_latex`.
#[derive( PartialEq, Default, Debug )]
pub struct TexOptions {
	pub drop_zero_decimal: Option<bool>,
	pub minimum_decimal_digits: Option<u8>,
}

impl TexOptions {
	// Create a new `TexOptions` without an option active. Is identical to `none()`.
	pub fn new() -> Self {
		Self::default()
	}

	// Create a new `Options` without an option active.
	pub fn none() -> Self {
		Self::default()
	}

	pub fn drop_zero_decimal( mut self, sw: bool ) -> Self {
		self.drop_zero_decimal = Some( sw );
		self
	}

	pub fn minimum_decimal_digits( mut self, digits: u8 ) -> Self {
		self.minimum_decimal_digits = Some( digits );
		self
	}
}

impl fmt::Display for TexOptions {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self.drop_zero_decimal {
			Some( x ) if x => write!( f, "[drop-zero-decimal]" ),
			_ => write!( f, "" ),
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn builder_test() {
		let opts = TexOptions {
			drop_zero_decimal: Some( true ),
			..Default::default()
		};
		let opts_from_builder = TexOptions::new().drop_zero_decimal( true );
		assert_eq!( opts, opts_from_builder );
	}

	#[test]
	fn options_to_string() {
		assert_eq!( TexOptions::default().to_string(), "".to_string() );
		assert_eq!(
			TexOptions::new()
				.drop_zero_decimal( true )
				.to_string(),
			"[drop-zero-decimal]".to_string()
		);
	}
}
