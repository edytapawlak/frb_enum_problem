pub use external_lib::TransformedText;
pub use external_lib::{Prefix, Sufix};
pub use external_lib::{PrefixedText, SufixedText};
use flutter_rust_bridge::frb;

pub type PrefixAlias = Prefix;
#[frb(mirror(PrefixAlias))]
pub enum _PrefixAlias {
    A,
	B,
	C
}

pub type PrefixedTextAlias = PrefixedText;
#[frb(mirror(PrefixedTextAlias))]
pub struct _PrefixedTextAlias {
	pub prefix: PrefixAlias,
	pub text: String
}

pub fn generate_prefixed_text(strategy: PrefixAlias, basic_text: String) -> PrefixedTextAlias {
	strategy.to_prefixed(basic_text)
}

pub type SufixAlias = Sufix;
#[frb(mirror(SufixAlias))]
pub enum _SufixAlias {
    A,
	B,
	C
}

pub type SufixedTextAlias = SufixedText;
#[frb(mirror(SufixedTextAlias))]
pub struct _SufixedTextAlias {
	pub sufix: SufixAlias,
	pub text: String
}

pub fn generate_sufixed_text(strategy: SufixAlias, basic_text: String) -> SufixedTextAlias {
	strategy.to_sufixed(basic_text)
}

#[frb(mirror(TransformedText))]
pub enum _TransformedText {
	Sufixed(SufixedTextAlias),
	Prefixed(PrefixedTextAlias)
}

pub fn transformed_text_from_str(str: String) -> TransformedText {
	str.parse().unwrap()
}
