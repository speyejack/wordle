use std::{io::{BufRead, BufReader}, fs::File, io::stdin, time::{Duration, Instant}};

use anyhow::Result;
use rand::{seq::IteratorRandom, Rng};
use owo_colors::OwoColorize;
use owo_colors::colors::*;

#[derive(Debug)]
enum CharMatch {
	NotFound,
	Misplaced,
	Exact
}

enum GuessResult {
	InvalidWord,
	CheckStatus,
	WrongGuess,
	Correct,
}

struct GameParameters {
	word_size: usize,
	tries: i32,
	word_list: Vec<String>,
	target_word: String,
	time_limit: Duration,
}

fn setup_game(rng: &mut impl Rng) -> Result<GameParameters> {

	let tries = 15;
	let word_size = 6;

	let time_limit = Duration::from_secs(600);

	let f = File::open("/home/jack/Downloads/words.txt")?;
	let file = BufReader::new(f);
	let word_list = file.lines()
		.filter_map(|x| Some(x.unwrap()))
		.filter(|x| x.len() == word_size)
		.collect::<Vec<String>>();

	let target_word = word_list.iter().choose(rng).unwrap().to_string();

	Ok(GameParameters{tries,word_size,time_limit, word_list, target_word})
}

fn main() -> Result<()> {
	let mut rng = rand::thread_rng();

	let params = setup_game(&mut rng)?;

	// println!("Shhhhh ;) : {}",target_word);
	println!("You have {} tries and {} seconds to guess a {} letter my word!",
			 params.tries, params.time_limit.as_secs(), params.word_size);

	let start_time = Instant::now();
	let mut try_number = 0;
	while try_number < params.tries {
		let guess_result = guess_word(&params.word_list, &params.target_word)?;
		match guess_result {
			GuessResult::Correct => {break;}
			GuessResult::CheckStatus => {}
			GuessResult::WrongGuess => {try_number += 1}
			GuessResult::InvalidWord => {try_number += 1}
		}

		let current_dur = start_time.elapsed();
		if current_dur >= params.time_limit {
			println!("Out of time :(");
			break;
		}
		let diff_dur = params.time_limit - current_dur;

		println!("You have {} tries and {} seconds left before your final guess.", params.tries - try_number, diff_dur.as_secs())

	}

	if try_number >= params.tries {
		println!("Sorry you ran out of guesses.");
	}
	println!("The word was: {}", params.target_word);

	Ok(())
}

fn guess_word(words: &Vec<String>, target_word: &String) -> Result<GuessResult> {
		let mut raw_input = String::new();
		stdin().read_line(&mut raw_input)?;
		let guessed_word = raw_input.trim().to_string();

		if !words.contains(&guessed_word) {
			println!("Invalid word/length, try again.");
			return Ok(GuessResult::InvalidWord);
		}

		let matches = match_word(&target_word, &guessed_word);

		guessed_word.chars()
			.zip(matches.iter())
			.for_each(|(t,m)| print_char(t,m));
		println!();

		if matches.iter().fold(true, |prev, x| match x {CharMatch::Exact => true, _ => false} && prev) {
			println!("You guessed it!");
			Ok(GuessResult::Correct)
		} else {
			Ok(GuessResult::WrongGuess)
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
	target.chars().zip(guess.chars()).map(|(t,g)| {
		if t == g {
			CharMatch::Exact
		} else if target.contains(g) {
			CharMatch::Misplaced
		} else {
			CharMatch::NotFound
		}
	}).collect()
}
