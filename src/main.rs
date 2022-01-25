use std::{
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
enum CharMatch {
    NotFound,
    Misplaced,
    Exact,
}

enum GuessResult {
    InvalidWord,
    WrongGuess,
    Correct,
}

const MUTATE_PROB: f64 = 0.2;

struct GameParameters {
    word_size: usize,
    tries: i32,
    guess_wordlist: Vec<String>,
    target_word: String,
    time_limit: Duration,
}

fn load_wordlist(filename: &str, word_size: usize) -> Result<Vec<String>> {
    let f = File::open(filename)?;
    let file = BufReader::new(f);
    let word_list = file
        .lines()
        .filter_map(|x| Some(x.unwrap()))
        .filter(|x| x.len() == word_size)
        .collect::<Vec<String>>();

    Ok(word_list)
}

fn setup_game(rng: &mut impl Rng) -> Result<GameParameters> {
    let tries = 30;
    let word_size = 5;

    let time_limit = Duration::from_secs(60 * 60);

    let guess_wordlist = load_wordlist("/home/jack/Downloads/words.txt", word_size)?;
    let answer_wordlist =
        load_wordlist("/home/jack/Documents/jordle/words/answers.txt", word_size)?;

    let target_word = answer_wordlist.iter().choose(rng).unwrap().to_string();

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
        let guess_result = guess_word(&params.guess_wordlist, &params.target_word, &mut rng)?;
        match guess_result {
            GuessResult::Correct => {
                break;
            }
            GuessResult::CheckStatus => {}
            GuessResult::WrongGuess => try_number += 1,
            GuessResult::InvalidWord => {}
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

fn guess_word(
    words: &Vec<String>,
    target_word: &String,
    rng: &mut impl Rng,
) -> Result<GuessResult> {
    let mut raw_input = String::new();
    stdin().read_line(&mut raw_input)?;
    let guessed_word = raw_input.trim().to_string();

    if target_word.len() != guessed_word.len() {
        println!("Invalid length, try again.");
        return Ok(GuessResult::InvalidWord);
    } else if !words.contains(&guessed_word) {
        println!("Invalid word, try again.");
        return Ok(GuessResult::InvalidWord);
    } else if *target_word == guessed_word {
        println!("You guessed it!");
        return Ok(GuessResult::Correct);
    }

    let matches: Vec<CharMatch> = match_word(&target_word, &guessed_word)
        .into_iter()
        .map(|x| mutate_match(x, rng))
        .collect();

    guessed_word
        .chars()
        .zip(matches.iter())
        .for_each(|(t, m)| print_char(t, m));

    print!(" - ");

    Ok(GuessResult::WrongGuess)
}

fn mutate_match(cmatch: CharMatch, rng: &mut impl Rng) -> CharMatch {
    let prob = match cmatch {
        CharMatch::Misplaced => MUTATE_PROB * 2.0,
        _ => MUTATE_PROB,
    };

    let should_mutate = rng.gen_bool(prob);

    if should_mutate {
        match cmatch {
            CharMatch::Exact => CharMatch::Misplaced,
            CharMatch::NotFound => CharMatch::Misplaced,
            CharMatch::Misplaced => {
                if rng.gen_bool(0.5) {
                    CharMatch::Exact
                } else {
                    CharMatch::NotFound
                }
            }
        }
    } else {
        cmatch
    }
}

fn print_char(c: char, cmatch: &CharMatch) {
    match *cmatch {
        CharMatch::Exact => print!("{}", c.fg::<Black>().bg::<Green>()),
        CharMatch::Misplaced => print!("{}", c.fg::<Black>().bg::<BrightBlue>()),
        CharMatch::NotFound => print!("{}", c.fg::<White>().bg::<BrightBlack>()),
    }
}

fn match_word(target: &str, guess: &str) -> Vec<CharMatch> {
    target
        .chars()
        .zip(guess.chars())
        .map(|(t, g)| {
            if t == g {
                CharMatch::Exact
            } else if target.contains(g) {
                CharMatch::Misplaced
            } else {
                CharMatch::NotFound
            }
        })
        .collect()
}
