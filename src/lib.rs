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

#[cfg( feature = "tex" )]
mod latex;

#[cfg( feature = "tex" )]
pub use crate::latex::{Latex, LatexSym};

#[cfg( feature = "tex" )]
pub use crate::latex::TexOptions;




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
