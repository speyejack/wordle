mod logic;
use logic::solver::*;
use logic::game::*;

use std::{
    collections::HashSet,
    fs::File,
    io::stdin,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};

use anyhow::Result;
use indicatif::ProgressBar;
use owo_colors::colors::*;
use owo_colors::OwoColorize;
use rand::{seq::IteratorRandom, Rng};


fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let answer_wordlist = load_wordlist("/home/jack/Documents/jordle/words/answers.txt", 5)?;

    let params = setup_game(&mut rng, &answer_wordlist)?;

    play_regular_game(params, &mut rng, &answer_wordlist)
}

fn play_regular_game(
    params: GameParameters,
    rng: &mut impl Rng,
    answer_wordlist: &HashSet<String>,
) -> Result<()> {
    // println!("Shhhhh ;) : {}", &params.target_word);
    println!(
        "You have {} tries and {} seconds to guess a {} letter word!",
        params.tries,
        params.time_limit.as_secs(),
        params.word_size
    );

    let target_word = gen_target_word(answer_wordlist, rng);

    let start_time = Instant::now();
    let mut try_number = 0;

    while try_number < params.tries {
        let guess_result = guess_user_word(
            &params.guess_wordlist,
            &target_word,
            rng,
            (params.word_size, params.word_size),
        )?;

        match guess_result {
            WordValidation::Valid(guess, matches) => {
                matches.iter().for_each(print_char);

                print!(" - ");

                match guess {
                    GuessResult::Correct => {
                        println!("You guessed it!");
                        break;
                    }
                    GuessResult::Wrong => try_number += 1,
                }
            }
            WordValidation::Invalid(reason, _) => match reason {
                InvalidationReason::WrongLength => {
                    println!("Invalid length, try again.");
                }
                InvalidationReason::UnknownWord => {
                    println!("Invalid word, try again.");
                }
            },
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
    println!("The word was: {}", &target_word);

    Ok(())
}

fn get_user_guess() -> Result<String> {
    let mut raw_input = String::new();
    stdin().read_line(&mut raw_input)?;
    let guessed_word = raw_input.trim().to_string();

    Ok(guessed_word)
}

fn guess_user_word(
    words: &HashSet<String>,
    target_word: &str,
    rng: &mut impl Rng,
    range: (usize, usize),
) -> Result<WordValidation> {
    let guessed_word = get_user_guess()?;
    guess_word(&guessed_word, words, target_word, rng, range)
}

fn print_char(cmatch: &CharMatch) {
    let (c, calign) = (cmatch.c, &cmatch.align);
    match *calign {
        CharAlignment::Exact => print!("{}", c.fg::<Black>().bg::<Green>()),
        CharAlignment::Misplaced => print!("{}", c.fg::<Black>().bg::<BrightBlue>()),
        CharAlignment::NotFound => print!("{}", c.fg::<White>().bg::<BrightBlack>()),
    }
}
