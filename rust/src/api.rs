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

pub type PrefixedTextAlias = PrefixedText;
#[frb(mirror(PrefixedTextAlias))]
pub struct _PrefixedTextAlias {
	pub prefix: PrefixAlias,
	pub text: String
}

pub fn generate_text(strategy: PrefixAlias, basic_text: String) -> PrefixedTextAlias {
	strategy.to_prefixed(basic_text)
}