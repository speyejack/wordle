mod solver;
use crate::solver::*;

use std::{
    collections::HashSet,
    fs::File,
    io::stdin,
    io::{BufRead, BufReader},
	time::{Duration, Instant}
};

use anyhow::Result;
use indicatif::ProgressBar;
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

#[derive(Debug)]
enum InvalidationReason {
    WrongLength,
    UnknownWord,
}

#[derive(Debug)]
enum GuessResult {
	Correct,
	Wrong,
}

#[derive(Debug)]
enum WordValidation {
    Invalid(InvalidationReason, String),
	Valid(GuessResult, Vec<CharMatch>)
}

const MUTATE_PROB: f64 = 0.2;

struct GameParameters {
    word_size: usize,
    tries: i32,
    guess_wordlist: HashSet<String>,
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

fn setup_game(rng: &mut impl Rng, answer_wordlist: &HashSet<String>) -> Result<GameParameters> {
    let tries = 30;
    let word_size = 5;

    let time_limit = Duration::from_secs(60 * 60);


    let base_guess_wordlist = load_wordlist("/home/jack/Documents/jordle/words/guesses.txt", word_size)?;
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

fn main() -> Result<()> {

    let mut rng = rand::thread_rng();

    let answer_wordlist =
        load_wordlist("/home/jack/Documents/jordle/words/answers.txt", 5)?;

    let params = setup_game(&mut rng, &answer_wordlist)?;

	// play_regular_game(params, &mut rng)
	play_auto_game(params, &mut rng, &answer_wordlist)
}

fn gen_target_word(answer_wordlist: &HashSet<String>, rng: &mut impl Rng) -> String {
	answer_wordlist.iter().choose(rng).unwrap().to_string()
}

fn play_auto_game(params: GameParameters, rng: &mut impl Rng, answer_wordlist: &HashSet<String>) -> Result<()> {
	let mut params = params;
	println!("Starting games");
	let mut guess_count = 0;
	let played_games = 1_000_000;
	let mut fail_count = 0;
	let bar = ProgressBar::new(played_games);

	for _ in 0..played_games {
		let target_word = gen_target_word(answer_wordlist, rng);

		let guesses = auto_game(&params, rng, answer_wordlist)?;
		guess_count += guesses;
		let failed = guesses > 6;
		fail_count += if failed {1} else {0};

		bar.inc(1);
	}
	bar.finish();

	println!("Played {} games, with {} avg and {} failures",
			 played_games,
			 guess_count as f32 / played_games as f32, fail_count); 

	Ok(())
}

fn auto_game(params: &GameParameters, rng: &mut impl Rng, answer_wordlist: &HashSet<String>) -> Result<i32> {
	let mut game_words = answer_wordlist.clone();
	let mut guess_count = 0;

	loop {
		let target_word = gen_target_word(answer_wordlist, rng);
		let counts = count_letter(&game_words);
		let word = game_words.iter()
			.map(|x| (x,score_word(x,&counts)))
			.fold((&"".to_string(),0.0),|acc, item| if acc.1 > item.1 {acc} else {item})
			.0.to_string();

		let guess_result = guess_word(&word, &params.guess_wordlist, &target_word, rng, (params.word_size,params.word_size))?;
		guess_count += 1;

		match guess_result {
			WordValidation::Valid(result, matches) => {
				if let GuessResult::Correct = result {
					return Ok(guess_count);
				}

				let filter = determine_filter(&matches);
				let new_words: HashSet<String> = game_words.into_iter().filter(|word| is_viable_word(word, &filter)).collect();
				
				game_words = new_words;

			}
			WordValidation::Invalid(_,_) => unreachable!(),
		}

	}
}

fn determine_filter(matches: &Vec<CharMatch>) -> FilterCriteria {
	let size = (5,5);
	let pos =  matches.iter().map(|x| if let CharAlignment::Exact = x.align {Some(x.c)} else {None}).collect();
	let nopos =  matches.iter().map(|x| if let CharAlignment::Misplaced = x.align {vec![x.c]} else {vec![]}).collect();

	let inc = matches.iter().filter_map(|x| match x.align {CharAlignment::NotFound => None, _ => Some(x.c)}).collect();
	let exc = matches.iter().filter_map(|x| match x.align {CharAlignment::NotFound => Some(x.c), _ => None}).collect();

	FilterCriteria{
		pos,nopos,inc,exc,size
	}
}

fn play_regular_game(params: GameParameters, rng: &mut impl Rng, answer_wordlist: &HashSet<String>)  -> Result<()>{
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
			WordValidation::Valid(guess,matches) => {

				matches.iter()
					.for_each(|m| print_char(m));

				print!(" - ");

				match guess {
					GuessResult::Correct => {
						println!("You guessed it!");
						break;
					},
					GuessResult::Wrong => {try_number += 1},
				}
			},
			WordValidation::Invalid(reason,_) => {
				match reason {
					InvalidationReason::WrongLength => {
						println!("Invalid length, try again.");
					},
					InvalidationReason::UnknownWord => {
						println!("Invalid word, try again.");
					}
				}
			}
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

fn guess_word(
	guessed_word: &String,
    words: &HashSet<String>,
    target_word: &str,
    rng: &mut impl Rng,
    range: (usize, usize),
) -> Result<WordValidation> {
	// println!("Using guessed_word {}", guessed_word);

    if guessed_word.len() < range.0 || guessed_word.len() > range.1 {
        return Ok(WordValidation::Invalid(InvalidationReason::WrongLength, guessed_word.to_string()));
    } else if !words.contains(guessed_word) {
        return Ok(WordValidation::Invalid(InvalidationReason::UnknownWord, guessed_word.to_string()));
    }

	let matches= match_word(target_word, &guessed_word);

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
