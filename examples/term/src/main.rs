use jordle::logic::*;

use std::io::stdin;
use std::time::Duration;

use anyhow::Result;
use owo_colors::colors::*;
use owo_colors::OwoColorize;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let params = GameParameters::default();

    let wordle = Wordle::new_random_game(params, &mut rng);
    play_regular_game(wordle)
}

fn play_regular_game(mut wordle: Wordle) -> Result<()> {
    let params = &wordle.params;
    let time_limit = params
        .time_limit
        .clone()
        .unwrap_or(Duration::from_secs(60 * 60));
    let attempt_limit = params.tries.unwrap_or(100);
    let start_time = wordle.state.start_time;

    println!(
        "You have {} tries and {} seconds to guess a {} letter word!",
        attempt_limit,
        time_limit.as_secs(),
        params.word_size.0
    );

    let mut attempt_number = 0;

    while attempt_number < attempt_limit {
        let guessed_word = get_user_guess()?;
        let guess_result = wordle.guess(guessed_word.as_str());

        match guess_result {
            WordValidation::Valid(guess, matches) => {
                matches.char_matches().for_each(print_char);

                print!(" - ");

                match guess {
                    GuessResult::Correct => {
                        println!("You guessed it!");
                        break;
                    }
                    GuessResult::Wrong => attempt_number += 1,
                }
            }
            WordValidation::Invalid(reason, _) => match reason {
                InvalidationReason::WrongLength => {
                    println!("Invalid length, try again.");
                }
                InvalidationReason::UnknownWord => {
                    println!("Invalid word, try again.");
                }
                InvalidationReason::RepeatWord => {
                    println!("Repeated guess, try again.");
                }
            },
        }

        let current_dur = start_time.map(|x| x.elapsed()).unwrap_or(Duration::ZERO);

        if current_dur >= time_limit {
            println!("Out of time :(");
            break;
        }
        let diff_dur = time_limit - current_dur;

        println!(
            "You have {} tries and {} seconds left before your final guess.",
            attempt_limit - attempt_number,
            diff_dur.as_secs()
        )
    }

    if attempt_number >= attempt_limit {
        println!("Sorry you ran out of guesses.");
    }

    println!("The word was: {}", &wordle.state.target_word);

    Ok(())
}

fn get_user_guess() -> Result<String> {
    let mut raw_input = String::new();
    stdin().read_line(&mut raw_input)?;
    let guessed_word = raw_input.trim().to_string();

    Ok(guessed_word)
}

fn print_char(cmatch: CharMatch) {
    let (c, calign) = (cmatch.c, &cmatch.align);
    match *calign {
        CharAlignment::Exact => print!("{}", c.fg::<Black>().bg::<Green>()),
        CharAlignment::Misplaced => print!("{}", c.fg::<Black>().bg::<BrightBlue>()),
        CharAlignment::NotFound => print!("{}", c.fg::<White>().bg::<BrightBlack>()),
    }
}
