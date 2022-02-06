use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration},
};

use super::mutator::*;
use anyhow::Result;
use rand::{seq::IteratorRandom, Rng};
use super::params::GameParameters;

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
		word_size: (word_size,
					word_size),
        tries: Some(tries),
		time_limit: Some(time_limit),
        guess_wordlist,
        answer_wordlist: answer_wordlist.clone().into_iter().collect(),
		mutator: Box::new(NoopMutator{}),
    })
}

pub fn gen_target_word(answer_wordlist: &HashSet<String>, rng: &mut impl Rng) -> String {
    answer_wordlist.iter().choose(rng).unwrap().to_string()
}

