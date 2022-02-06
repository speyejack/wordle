
pub type StringMatch = Vec<CharMatch>;

#[derive(Debug, Copy, Clone)]
pub enum CharAlignment {
    NotFound,
    Misplaced,
    Exact,
}

#[derive(Debug, Copy, Clone)]
pub struct CharMatch {
    pub c: char,
    pub align: CharAlignment,
}

fn matches_str(smatch: &StringMatch, other: &str) -> bool {
	smatch.iter().zip(other.chars()).all(|(cmatch, c)| c == cmatch.c)
}
