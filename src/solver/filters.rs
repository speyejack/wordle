use crate::logic::game::*;

#[derive(Debug)]
pub struct FilterCriteria {
    pub pos: Vec<Option<char>>,
    pub nopos: Vec<Vec<char>>,
    pub inc: Vec<char>,
    pub exc: Vec<char>,
    pub size: (usize, usize),
}

pub fn is_viable_word(word: &str, criteria: &FilterCriteria) -> bool {
    let incorrect_size = word.len() < criteria.size.0 || word.len() > criteria.size.1;
    if incorrect_size {
        return false;
    }

    let includes_letters = criteria.inc.iter().all(|c| word.contains(*c));
    if !includes_letters {
        return false;
    }

    let excludes_letters = criteria.exc.iter().all(|c| !word.contains(*c));
    if !excludes_letters {
        return false;
    }

    let includes_positional_letters = criteria
        .pos
        .iter()
        .zip(word.chars())
        .all(|(inc, c)| inc.map(|x| x == c).unwrap_or(true));

    if !includes_positional_letters {
        return false;
    }

    let excludes_positional_letters = criteria
        .nopos
        .iter()
        .zip(word.chars())
        .all(|(exc, c)| exc.iter().all(|x| *x != c));

    if !excludes_positional_letters {
        return false;
    }

    true
}

pub fn determine_filter(matches: &[CharMatch]) -> FilterCriteria {
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
        .map(|x| {
            if let CharAlignment::Misplaced = x.align {
                vec![x.c]
            } else {
                vec![]
            }
        })
        .collect();

    let inc = matches
        .iter()
        .filter_map(|x| match x.align {
            CharAlignment::NotFound => None,
            _ => Some(x.c),
        })
        .collect();
    let exc = matches
        .iter()
        .filter_map(|x| match x.align {
            CharAlignment::NotFound => Some(x.c),
            _ => None,
        })
        .collect();

    FilterCriteria {
        pos,
        nopos,
        inc,
        exc,
        size,
    }
}
