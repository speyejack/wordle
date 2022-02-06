use jordle::logic::*;
use jordle::solver::filters::*;
use jordle::solver::scoring::*;
use jordle::solver::*;
use clap::{Parser, Subcommand};

use indicatif::ProgressBar;
use std::collections::HashSet;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[clap(subcommand)]
	command: Commands
}

#[derive(Subcommand)]
enum Commands {
	Trial{
		count: usize,
	},

	Target {
		target: String,
	}
}


fn main() {
	let command = Cli::parse();
    let rng = rand::thread_rng();

    let params = GameParameters::default();

    let wordle = Wordle::new_random_game(params, rng);

	match command.command {
		Commands::Trial{count} => repeat_auto_game(wordle, count),
		Commands::Target{target} => run_auto_game(wordle, target)
	}
}


fn run_auto_game(mut wordle: Wordle, target: String) {
	println!("Searching for word: {}\n", &target);
	wordle.state.target_word = target;

    let mut game_words = wordle
        .params
        .answer_wordlist
        .clone()
        .into_iter()
        .collect::<HashSet<_>>();

    loop {
		let (results, word) = take_guess(&mut wordle, game_words);
		println!("Guessed: {}", word);
		if let Some(new_words) =  results {
			game_words = new_words;
		} else {
			break;
		}
    }

	println!("Solved!")
}

fn repeat_auto_game(mut wordle: Wordle, played_games: usize) {
    let mut guess_count = 0;
    let mut fail_count = 0;

    let bar = ProgressBar::new(played_games.try_into().unwrap());
    for _ in 0..played_games {
        let guesses = auto_game(&mut wordle);
        guess_count += guesses;
        let failed = guesses > 6;
        fail_count += if failed { 1 } else { 0 };

        wordle = wordle.restart();
        bar.inc(1);
    }
    bar.finish();

    println!(
        "Played {} games, with {} avg and {} failures",
        played_games,
        guess_count as f32 / played_games as f32,
        fail_count
    );
}

fn auto_game(wordle: &mut Wordle) -> i32 {
    let mut game_words = wordle
        .params
        .answer_wordlist
        .clone()
        .into_iter()
        .collect::<HashSet<_>>();
    let mut guess_count = 0;

    loop {
		guess_count +=1;
		let (results, _) = take_guess(wordle, game_words);
		if let Some(new_words) =  results {
			game_words = new_words;
		} else {
			break;
		}
    }

	guess_count
}


fn take_guess(wordle: &mut Wordle, game_words: HashSet<String>) -> (Option<HashSet<String>>, String) {
    let counts = count_letter(&game_words);
    let word = game_words
        .iter()
        .map(|x| (x, score_word(x, &counts)))
        .fold((&"".to_string(), 0.0), |acc, item| {
            if acc.1 > item.1 {
                acc
            } else {
                item
            }
        })
        .0
        .to_string();

    let guess_result = wordle.guess(&word);

    match guess_result {
        WordValidation::Valid(result, matches) => {
            if let GuessResult::Correct = result {
                return (None, word);
            }

            let filter = PosFilterCriteria::from_matches(&matches);
            let new_words: HashSet<String> = game_words
                .into_iter()
                .filter(|word| filter.check(word))
                .collect();

            return (Some(new_words), word)
        }
        WordValidation::Invalid(_, _) => unreachable!(),
    }
}
