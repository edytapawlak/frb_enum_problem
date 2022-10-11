pub use external_lib::PrefixAdder;
use flutter_rust_bridge::frb;

#[frb(mirror(PrefixAdder))]
pub enum _PrefixAdder {
    A,
	B,
	C
}

pub fn generate_text(strategy: PrefixAdder, basic_text: String) -> String {
	strategy.add(basic_text)
}