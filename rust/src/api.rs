pub use external_lib::TransformedText;
pub use external_lib::{Prefix, Sufix};
pub use external_lib::{PrefixedText, SufixedText};
use flutter_rust_bridge::frb;

#[frb(mirror(Prefix))]
pub enum _Prefix{
    A,
	B,
	C
}

#[frb(mirror(PrefixedText))]
pub struct _PrefixedText{
	pub prefix: Prefix,
	pub text: String
}

pub fn generate_prefixed_text(strategy: Prefix, basic_text: String) -> PrefixedText{
	strategy.to_prefixed(basic_text)
}

#[frb(mirror(Sufix))]
pub enum _Sufix{
    A,
	B,
	C
}

#[frb(mirror(SufixedText))]
pub struct _SufixedText{
	pub sufix: Sufix,
	pub text: String
}

pub fn generate_sufixed_text(strategy: Sufix, basic_text: String) -> SufixedText {
	strategy.to_sufixed(basic_text)
}

#[frb(mirror(TransformedText))]
pub enum _TransformedText {
	Sufixed(SufixedText),
	Prefixed(PrefixedText)
}

pub fn transformed_text_from_str(str: String) -> TransformedText {
	str.parse().unwrap()
}
