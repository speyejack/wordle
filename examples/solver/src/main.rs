use clap::{Parser, Subcommand};
use jordle::{logic::*, solver::positional::PositionalSolver};

use indicatif::ProgressBar;
use std::collections::HashSet;
// use jordle::solver::positional::Pos

type WordleSolver = Box<dyn jordle::solver::solvers::Solver>;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Trial { count: usize },

    Target { target: String },
}

fn main() {
    let command = Cli::parse();
    let rng = rand::thread_rng();

    let params = GameParameters::default();

    let wordle = Wordle::new_random_game(params, rng);

    match command.command {
        Commands::Trial { count } => repeat_auto_game(wordle, count),
        Commands::Target { target } => run_auto_game(wordle, target),
    }
}

fn run_auto_game(mut wordle: Wordle, target: String) {
    println!("Searching for word: {}\n", &target);
    wordle.state.target_word = target;

    let game_words = wordle
        .params
        .answer_wordlist
        .clone()
        .into_iter()
        .collect::<HashSet<_>>();

	let mut solver: WordleSolver =  Box::new(PositionalSolver::load_wordlist(game_words));

    loop {
        let (running, word) = take_guess(&mut wordle, &mut solver);
        println!("Guessed: {}", word);
		if !running {
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
    let game_words = wordle
        .params
        .answer_wordlist
        .clone()
        .into_iter()
        .collect::<HashSet<_>>();

	let mut solver: WordleSolver =  Box::new(PositionalSolver::load_wordlist(game_words));

    let mut guess_count = 0;

    loop {
        guess_count += 1;
        let (running, _) = take_guess(wordle, &mut solver);
		if !running {
			break;
		}
    }

    guess_count
}

fn take_guess(
    wordle: &mut Wordle,
    solver: &mut WordleSolver,
) -> (bool, String) {

	let guess_word = solver.guess();

    let guess_result = wordle.guess(&guess_word);

    match guess_result {
        WordValidation::Valid(result, matches) => {
            if let GuessResult::Correct = result {
                return (false, guess_word);
            }

			solver.narrow_words(&matches);

            return (true, guess_word);
        }
        WordValidation::Invalid(_, _) => unreachable!(),
    }
}
