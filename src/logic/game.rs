use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

use super::mutator::*;
use anyhow::Result;
use rand::{seq::IteratorRandom, Rng};

#[derive(Debug)]
pub enum CharAlignment {
    NotFound,
    Misplaced,
    Exact,
}

#[derive(Debug)]
pub struct CharMatch {
    pub c: char,
    pub align: CharAlignment,
}

#[derive(Debug)]
pub enum InvalidationReason {
    WrongLength,
    UnknownWord,
}

#[derive(Debug)]
pub enum GuessResult {
    Correct,
    Wrong,
}

#[derive(Debug)]
pub enum WordValidation {
    Invalid(InvalidationReason, String),
    Valid(GuessResult, Vec<CharMatch>),
}

pub struct GameParameters {
    pub word_size: usize,
    pub tries: i32,
    pub guess_wordlist: HashSet<String>,
    pub time_limit: Duration,
}

pub fn load_wordlist(filename: &str, word_size: usize) -> Result<HashSet<String>> {
    let f = File::open(filename)?;
    let file = BufReader::new(f);
    let word_list = file
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| x.len() == word_size)
        .collect::<HashSet<String>>();

    Ok(word_list)
}

pub fn setup_game(answer_wordlist: &HashSet<String>) -> Result<GameParameters> {
    let tries = 30;
    let word_size = 5;

    let time_limit = Duration::from_secs(60 * 60);

    let base_guess_wordlist =
        load_wordlist("/home/jack/Documents/jordle/words/guesses.txt", word_size)?;
    let guess_wordlist = base_guess_wordlist
        .into_iter()
        .chain(answer_wordlist.clone().into_iter())
        .collect::<HashSet<String>>();

    Ok(GameParameters {
        tries,
        word_size,
        time_limit,
        guess_wordlist,
    })
}

pub fn gen_target_word(answer_wordlist: &HashSet<String>, rng: &mut impl Rng) -> String {
    answer_wordlist.iter().choose(rng).unwrap().to_string()
}

pub fn guess_word(
    guessed_word: &str,
    words: &HashSet<String>,
    target_word: &str,
    rng: &mut impl Rng,
    range: (usize, usize),
) -> Result<WordValidation> {
    // println!("Using guessed_word {}", guessed_word);

    if guessed_word.len() < range.0 || guessed_word.len() > range.1 {
        return Ok(WordValidation::Invalid(
            InvalidationReason::WrongLength,
            guessed_word.to_string(),
        ));
    } else if !words.contains(guessed_word) {
        return Ok(WordValidation::Invalid(
            InvalidationReason::UnknownWord,
            guessed_word.to_string(),
        ));
    }

    let matches = match_word(target_word, guessed_word);

    if *target_word == *guessed_word {
        return Ok(WordValidation::Valid(GuessResult::Correct, matches));
    }

    // let mutator = NoopMutator::default();
    let mutator = NoopMutator::default();

    let matches: Vec<CharMatch> = matches
        .into_iter()
        .map(|x| CharMatch {
            align: mutator.mutate(x.align, rng),
            c: x.c,
        })
        .collect();

    Ok(WordValidation::Valid(GuessResult::Wrong, matches))
}

fn match_word(target: &str, guess: &str) -> Vec<CharMatch> {
    target
        .chars()
        .zip(guess.chars())
        .map(|(t, g)| {
            let align = if t == g {
                CharAlignment::Exact
            } else if target.contains(g) {
                CharAlignment::Misplaced
            } else {
                CharAlignment::NotFound
            };

            CharMatch { c: g, align }
        })
        .collect()
}
