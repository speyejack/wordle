pub mod filters;
pub mod scoring;

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
    let pos_weight = 1.0;

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

