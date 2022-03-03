
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WordMatch<'a> {
	pub word: &'a str,
	pub alignments: Vec<CharAlignment>,
}

impl<'a> WordMatch<'a> {

	pub fn char_matches(&'a self) -> impl Iterator<Item=CharMatch> + 'a {
		self.word.chars().zip(self.alignments.iter()).map(|(c, align)| CharMatch {c, align: *align})
	}
}

pub fn matches_str(smatch: &WordMatch, other: &str) -> bool {
	smatch.word == other
}
