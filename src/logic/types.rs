use std::fmt::{Display, Debug};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct WordMatch {
	pub word: String,
	pub aligns: Vec<CharAlignment>,
}

impl Debug for WordMatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if f.alternate() {
			write!(f, "WordMatch{{{:?} -> {:?}}}", self.word, self.aligns.iter().map(|x| x.as_char()).collect::<String>())
		} else {
			let match_word: String = self.char_matches().map(|x| {
				match x.align {
					CharAlignment::Exact => x.c,
					_ => x.align.as_char()
				}
			}).collect();

			write!(f, "{}", match_word)
		}
	}
}

impl WordMatch {
	pub fn char_matches(&self) -> impl Iterator<Item=CharMatch> + '_ {
		self.word.chars().zip(self.aligns.iter()).map(|(c, align)| CharMatch {c, align: *align})
	}
}


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CharAlignment {
	NotFound,
	Misplaced,
	Exact,
}

impl CharAlignment {
	pub fn as_char(&self) -> char {
		match self {
			Self::NotFound => '.',
			Self::Misplaced => '-',
			Self::Exact => 'x'
		}
	}
}

impl Debug for CharAlignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_char())
	}
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CharMatch {
	pub c: char,
	pub align: CharAlignment,
}
