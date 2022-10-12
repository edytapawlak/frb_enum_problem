pub use external_lib::Prefix;
pub use external_lib::PrefixedText;
use flutter_rust_bridge::frb;

pub type PrefixAlias = Prefix;
#[frb(mirror(PrefixAlias))]
pub enum _PrefixAlias {
    A,
	B,
	C
}

#[frb(mirror(PrefixedText))]
pub struct _PrefixedText {
	pub prefix: PrefixAlias,
	pub text: String
}

pub fn generate_text(strategy: PrefixAlias, basic_text: String) -> PrefixedText {
	strategy.to_prefixed(basic_text)
}