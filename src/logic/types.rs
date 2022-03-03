
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WordMatch {
	pub word: String,
	pub aligns: Vec<CharAlignment>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CharAlignment {
    NotFound,
    Misplaced,
    Exact,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CharMatch {
    pub c: char,
    pub align: CharAlignment,
}

pub fn matches_str(smatch: &WordMatch, other: &str) -> bool {
	smatch.word == other
}

impl WordMatch {
	pub fn char_matches(&self) -> impl Iterator<Item=CharMatch> + '_ {
		self.word.chars().zip(self.aligns.iter()).map(|(c, align)| CharMatch {c, align: *align})
	}
}
