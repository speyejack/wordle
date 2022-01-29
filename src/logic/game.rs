use super::solver::*;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

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

const MUTATE_PROB: f64 = 0.2;

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

pub fn setup_game(rng: &mut impl Rng, answer_wordlist: &HashSet<String>) -> Result<GameParameters> {
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

    let matches: Vec<CharMatch> = matches
        .into_iter()
        // .map(|x| mutate_match(x, rng))
        .collect();

    Ok(WordValidation::Valid(GuessResult::Wrong, matches))
}

fn mutate_match(cmatch: CharAlignment, rng: &mut impl Rng) -> CharAlignment {
    let prob = match cmatch {
        CharAlignment::Misplaced => MUTATE_PROB * 2.0,
        _ => MUTATE_PROB,
    };

    let should_mutate = rng.gen_bool(prob);

    if should_mutate {
        match cmatch {
            CharAlignment::Exact => CharAlignment::Misplaced,
            CharAlignment::NotFound => CharAlignment::Misplaced,
            CharAlignment::Misplaced => {
                if rng.gen_bool(0.5) {
                    CharAlignment::Exact
                } else {
                    CharAlignment::NotFound
                }
            }
        }
    } else {
        cmatch
    }
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
