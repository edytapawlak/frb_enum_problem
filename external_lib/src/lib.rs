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

impl ToString for Prefix {
    fn to_string(&self) -> String {
        match self {
            Prefix::A => "A",
            Prefix::B => "B",
            Prefix::C => "C",
        }.to_string()
    }
}

#[derive(Clone)]
pub struct PrefixedText {
	pub prefix: Prefix,
	pub text: String
}

impl PrefixedText {
	pub fn to_str(&self) -> String {
		[self.prefix.to_string(), self.text.clone()].join("")
	}
}