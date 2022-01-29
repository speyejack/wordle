use std::collections::HashMap;
use super::PosLetterFreq;

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
