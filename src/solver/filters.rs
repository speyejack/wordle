use crate::logic::{CharAlignment, CharMatch};

pub trait FilterCriteria {
    fn from_matches(matches: &[CharMatch]) -> Self;
    fn check(&self, word: &str) -> bool;
}

#[derive(Debug)]
pub struct PosFilterCriteria {
    pub pos: Vec<Option<char>>,
    pub nopos: Vec<Vec<char>>,
    pub inc: Vec<char>,
    pub exc: Vec<char>,
    pub size: (usize, usize),
}

impl FilterCriteria for PosFilterCriteria {
    fn from_matches(matches: &[CharMatch]) -> Self {
        let size = (5, 5);
        let pos = matches
            .iter()
            .map(|x| {
                if let CharAlignment::Exact = x.align {
                    Some(x.c)
                } else {
                    None
                }
            })
            .collect();

        let nopos = matches
            .iter()
            .map(|x| match x.align {
                CharAlignment::Misplaced | CharAlignment::NotFound => vec![x.c],
                _ => vec![],
            })
            .collect();

        let inc: Vec<char> = matches
            .iter()
            .filter_map(|x| match x.align {
                CharAlignment::NotFound => None,
                _ => Some(x.c),
            })
            .collect();

        let exc = matches
            .iter()
            .filter_map(|x| match x.align {
                CharAlignment::NotFound => {
                    if inc.contains(&x.c) {
                        None
                    } else {
                        Some(x.c)
                    }
                }
                _ => None,
            })
            .collect();

        PosFilterCriteria {
            pos,
            nopos,
            inc,
            exc,
            size,
        }
    }

    fn check(&self, word: &str) -> bool {
        let incorrect_size = word.len() < self.size.0 || word.len() > self.size.1;
        if incorrect_size {
            return false;
        }

        let includes_letters = self.inc.iter().all(|c| word.contains(*c));
        if !includes_letters {
            return false;
        }

        let excludes_letters = self.exc.iter().all(|c| !word.contains(*c));
        if !excludes_letters {
            return false;
        }

        let includes_positional_letters = self
            .pos
            .iter()
            .zip(word.chars())
            .all(|(inc, c)| inc.map(|x| x == c).unwrap_or(true));

        if !includes_positional_letters {
            return false;
        }

        let excludes_positional_letters = self
            .nopos
            .iter()
            .zip(word.chars())
            .all(|(exc, c)| exc.iter().all(|x| *x != c));

        if !excludes_positional_letters {
            return false;
        }

        true
    }
}
