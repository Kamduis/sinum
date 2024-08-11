// Replace crate links with internal links when creating documentation with `cargo`.
//! [`Num`]: crate::Num
//! [`Prefix`]: crate::Prefix
//! [`Qty`]: crate::Qty
//! [`serde`]: serde
// File links are not supported by rustdoc.
//! [LICENSE-APACHE]: https://github.com/Kamduis/sinum/blob/master/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/Kamduis/sinum/blob/master/LICENSE-MIT
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!( "../README.md" )]




//=============================================================================
// Modules


#[cfg( feature = "i18n" )] use std::fmt;

#[cfg( feature = "i18n" )] use unic_langid::LanguageIdentifier;

mod prefix;
pub use crate::prefix::PrefixError;
pub use crate::prefix::Prefix;

mod number;
pub use crate::number::Num;

mod unit;
use crate::unit::PhysicalQuantity;
pub use crate::unit::UnitError;
pub use crate::unit::Unit;

mod quantity;
pub use crate::quantity::Qty;

#[cfg( feature = "tex" )] mod latex;
#[cfg( feature = "tex" )] pub use crate::latex::{Latex, LatexSym};
#[cfg( all( feature = "i18n", feature = "tex" ) )] pub use crate::latex::LatexLocale;
#[cfg( feature = "tex" )] pub use crate::latex::TexOptions;




//=============================================================================
// Traits


/// Providing a localized `.to_string()`: `.to_string_locale()`.
///
/// This Trait is only available, if the **`i18n`** feature has been enabled.
#[cfg( feature = "i18n" )]
pub trait DisplayLocale: fmt::Display {
	/// Returns the localized string representation of `self`.
	///
	/// The standard implementation ignores `locale` and returns the same string as `.to_string()`.
	#[allow( unused_variables )]
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		self.to_string()
	}
}




//=============================================================================
// Internationalization


#[cfg( feature = "i18n" )]
fluent_templates::static_loader! {
	static LOCALES = {
		// The directory of localisations and fluent resources.
		locales: "./locales",

		// The language to falback on if something is not present.
		fallback_language: "en-US",
	};
}
