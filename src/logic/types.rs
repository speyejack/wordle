pub type StringMatch = Vec<CharMatch>;

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

pub fn matches_str(smatch: &StringMatch, other: &str) -> bool {
    smatch
		.iter()
		.zip(other.chars())
        .all(|(cmatch, c)| c == cmatch.c)
}
