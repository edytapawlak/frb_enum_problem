#[derive(Clone)]
pub enum Prefix {
	A,
	B,
	C
}

impl Prefix {
	pub fn to_prefixed(&self, text: String) -> PrefixedText {
		PrefixedText {prefix: self.clone(), text}
	}
}

#[derive(Clone)]
pub struct PrefixedText {
	pub prefix: Prefix,
	pub text: String
}

impl Prefix {
	pub fn to_str(&self, text: String) -> String {
		match self {
			Prefix::A => ["A", &text].join(""),
			Prefix::B => ["B", &text].join(""),
			Prefix::C => ["C", &text].join(""),
		}
	}
}