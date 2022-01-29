use std::collections::{HashMap, HashSet};

type LetterCount = HashMap<char, i32>;
type LetterFreq = HashMap<char, f32>;
type PosLetterFreq = Vec<LetterFreq>;

pub fn count_letter(word_list: &HashSet<String>) -> PosLetterFreq {
    let max_size = word_list.iter().map(|x| x.len()).max().unwrap_or(0);

    let mut pos_count: Vec<LetterCount> = (0..max_size).map(|_| LetterCount::new()).collect();
    let mut tot_count: LetterCount = LetterCount::new();

    for word in word_list.iter() {
        for (c, posmap) in word.chars().zip(pos_count.iter_mut()) {
            *tot_count.entry(c).or_insert(0) += 1;
            *posmap.entry(c).or_insert(0) += 1;
        }
    }

    let tot_weight = 0.1;
    let pos_weight = 0.1;

    let tot_amount = word_list.len() as f32;

    pos_count
        .into_iter()
        .map(|posmap| {
            posmap
                .into_iter()
                .map(|(c, pos_score)| {
                    let pos_freq = pos_score as f32 / tot_amount;
                    let total_freq =
                        *tot_count.entry(c).or_insert(0) as f32 / (tot_amount * max_size as f32);
                    let final_freq = pos_freq * pos_weight + total_freq * tot_weight;
                    (c, final_freq)
                })
                .collect()
        })
        .collect()
}

pub fn score_word(word: &str, letter_freq: &PosLetterFreq) -> f32 {
    let char_scores: Vec<(char, f32)> = word
        .chars()
        .zip(letter_freq.iter())
        .map(|(c, local_letter_freq)| {
            let score: f32 = *local_letter_freq.get(&c).unwrap_or(&0.0);
            (c, score)
        })
        .collect();

    let mut letter_map = HashMap::new();
    char_scores.iter().for_each(|(c, score)| {
        let max_score = letter_map.entry(*c).or_insert(0.0);
        *max_score = score.max(*max_score);
    });

    let final_score = letter_map.iter().map(|x| *x.1).sum();
    final_score
}

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
