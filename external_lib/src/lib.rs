pub enum PrefixAdder {
	A,
	B,
	C
}

impl PrefixAdder {
	pub fn add(&self, text: String) -> String {
		match self {
			PrefixAdder::A => ["A", &text].join(""),
			PrefixAdder::B => ["B", &text].join(""),
			PrefixAdder::C => ["C", &text].join(""),
		}
	}
}