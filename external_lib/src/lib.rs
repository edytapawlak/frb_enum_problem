use std::{str::FromStr, fmt::Error};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Sufix {
	A,
	B,
	C
}

impl Sufix {
	pub fn to_sufixed(&self, text: String) -> SufixedText {
		SufixedText {sufix: self.clone(), text}
	}
}

impl ToString for Sufix {
    fn to_string(&self) -> String {
        match self {
            Sufix::A => "A",
            Sufix::B => "B",
            Sufix::C => "C",
        }.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct PrefixedText {
	pub prefix: Prefix,
	pub text: String
}

impl PrefixedText {
	pub fn to_str(&self) -> String {
		[self.prefix.to_string(), self.text.clone()].join("")
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub struct SufixedText {
	pub sufix: Sufix,
	pub text: String
}

impl SufixedText {
	pub fn to_str(&self) -> String {
		[self.text.clone(), self.sufix.to_string()].join("")
	}
}

#[derive(Debug, Clone)]
pub enum TransformedText {
	Sufixed(SufixedText),
	Prefixed(PrefixedText)
}

impl FromStr for TransformedText {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut chars = s.chars();
        Ok(match chars.next().unwrap() {
			'A' => TransformedText::Prefixed(PrefixedText { prefix: Prefix::A, text: chars.collect() }),
			'B' => TransformedText::Prefixed(PrefixedText { prefix: Prefix::B, text: chars.collect() }),
			'C' => TransformedText::Prefixed(PrefixedText { prefix: Prefix::C, text: chars.collect() }),
			other => {
				match chars.clone().last().unwrap() {
					'A' => TransformedText::Sufixed(SufixedText { sufix: Sufix::A, text: [other.to_string(), chars.collect()].join("") }),
					'B' => TransformedText::Sufixed(SufixedText { sufix: Sufix::B, text: [other.to_string(), chars.collect()].join("") }),
					'C' => TransformedText::Sufixed(SufixedText { sufix: Sufix::C, text: [other.to_string(), chars.collect()].join("") }),
					 _ => todo!()
				}
			}
		})
    }
}