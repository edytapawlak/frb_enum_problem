pub use external_lib::Prefix;
pub use external_lib::PrefixedText;
use flutter_rust_bridge::frb;

#[frb(mirror(Prefix))]
pub enum _Prefix {
    A,
	B,
	C
}

#[frb(mirror(PrefixedText))]
pub struct _PrefixedText {
	pub prefix: Prefix,
	pub text: String
}

pub fn generate_text(strategy: Prefix, basic_text: String) -> PrefixedText {
	strategy.to_prefixed(basic_text)
}