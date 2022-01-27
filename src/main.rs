use std::{
    collections::HashSet,
    fs::File,
    io::stdin,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};

use anyhow::Result;
use owo_colors::colors::*;
use owo_colors::OwoColorize;
use rand::{seq::IteratorRandom, Rng};

#[derive(Debug)]
enum CharAlignment {
    NotFound,
    Misplaced,
    Exact,
}

#[derive(Debug)]
struct CharMatch {
	c: char,
	align: CharAlignment,
}

enum InvalidationReason {
    WrongLength,
    UnknownWord,
}

enum GuessResult {
	Correct,
	Wrong,
}

enum WordValidation {
    Invalid(InvalidationReason, String),
	Valid(GuessResult, Vec<CharMatch>)
}

const MUTATE_PROB: f64 = 0.2;

struct GameParameters {
    word_size: usize,
    tries: i32,
    guess_wordlist: HashSet<String>,
    target_word: String,
    time_limit: Duration,
}

fn load_wordlist(filename: &str, word_size: usize) -> Result<HashSet<String>> {
    let f = File::open(filename)?;
    let file = BufReader::new(f);
    let word_list = file
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| x.len() == word_size)
        .collect::<HashSet<String>>();

    Ok(word_list)
}

fn setup_game(rng: &mut impl Rng) -> Result<GameParameters> {
    let tries = 30;
    let word_size = 5;

    let time_limit = Duration::from_secs(60 * 60);

    let answer_wordlist =
        load_wordlist("/home/jack/Documents/jordle/words/answers.txt", word_size)?;

    let target_word = answer_wordlist.iter().choose(rng).unwrap().to_string();

    let base_guess_wordlist = load_wordlist("/home/jack/Downloads/words.txt", word_size)?;
    let guess_wordlist = base_guess_wordlist
        .into_iter()
        .chain(answer_wordlist.into_iter())
        .collect::<HashSet<String>>();

    Ok(GameParameters {
        tries,
        word_size,
        time_limit,
        guess_wordlist,
        target_word,
    })
}

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let params = setup_game(&mut rng)?;

    // println!("Shhhhh ;) : {}", &params.target_word);
    println!(
        "You have {} tries and {} seconds to guess a {} letter word!",
        params.tries,
        params.time_limit.as_secs(),
        params.word_size
    );

    let start_time = Instant::now();
    let mut try_number = 0;

    while try_number < params.tries {
        let guess_result = guess_word(
            &params.guess_wordlist,
            &params.target_word,
            &mut rng,
            (params.word_size, params.word_size),
        )?;

		match guess_result {
			WordValidation::Valid(guess,_) => {
				match guess {
					GuessResult::Correct => {break;},
					GuessResult::Wrong => {try_number += 1},
				}
			},
			WordValidation::Invalid(_,_) => {}
		}

        let current_dur = start_time.elapsed();
        if current_dur >= params.time_limit {
            println!("Out of time :(");
            break;
        }
        let diff_dur = params.time_limit - current_dur;

        println!(
            "You have {} tries and {} seconds left before your final guess.",
            params.tries - try_number,
            diff_dur.as_secs()
        )
    }

    if try_number >= params.tries {
        println!("Sorry you ran out of guesses.");
    }
    println!("The word was: {}", params.target_word);

    Ok(())
}

fn get_user_guess() -> Result<String> {
    let mut raw_input = String::new();
    stdin().read_line(&mut raw_input)?;
    let guessed_word = raw_input.trim().to_string();

	Ok(guessed_word)
}

fn guess_word(
    words: &HashSet<String>,
    target_word: &str,
    rng: &mut impl Rng,
    range: (usize, usize),
) -> Result<WordValidation> {
	let guessed_word = get_user_guess()?;

    if guessed_word.len() < range.0 || guessed_word.len() > range.1 {
        println!("Invalid length, try again.");
        return Ok(WordValidation::Invalid(InvalidationReason::WrongLength, guessed_word));
    } else if !words.contains(&guessed_word) {
        println!("Invalid word, try again.");
        return Ok(WordValidation::Invalid(InvalidationReason::UnknownWord, guessed_word));
    }

	let matches= match_word(target_word, &guessed_word);

	if *target_word == guessed_word {
        println!("You guessed it!");
        return Ok(WordValidation::Valid(GuessResult::Correct, matches));
    }

    let matches: Vec<CharMatch> = matches
        .into_iter()
        // .map(|x| mutate_match(x, rng))
        .collect();

        matches.iter()
        .for_each(|m| print_char(m));

    print!(" - ");

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

fn print_char(cmatch: &CharMatch) {
	let (c, calign) = (cmatch.c, &cmatch.align);
    match *calign {
        CharAlignment::Exact => print!("{}", c.fg::<Black>().bg::<Green>()),
        CharAlignment::Misplaced => print!("{}", c.fg::<Black>().bg::<BrightBlue>()),
        CharAlignment::NotFound => print!("{}", c.fg::<White>().bg::<BrightBlack>()),
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

			CharMatch { c: g, align}
        })
        .collect()
}
