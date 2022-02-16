use clap::{Parser, Subcommand, ArgEnum};
use jordle::{logic::*, solver::{positional::PositionalSolver, solvers::{SolverWordList, Solver}, entropy::EntropySolver}};

use indicatif::ProgressBar;
use std::collections::HashSet;
// use jordle::solver::positional::Pos

type WordleSolver<'a> = Box<dyn jordle::solver::solvers::Solver + 'a>;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(default_value_t = SelectedSolver::Positional, arg_enum)]
	solver: SelectedSolver,
}

#[derive(Subcommand)]
enum Commands {
    Trial {},
    Sample { count: usize },
    Target { target: String },
}

#[derive(ArgEnum, Clone)]
enum SelectedSolver {
	Positional,
	Entropy,
}

impl<'a> SelectedSolver {
	fn create_solver(&self, wordlist: SolverWordList<'a>) -> Box<dyn Solver + 'a> {
		match self {
			Positional => Box::new(PositionalSolver::load_wordlist(wordlist)),
			Entropy => Box::new(EntropySolver::load_wordlist(wordlist)),
		}
	}
}

fn main() {
    let command = Cli::parse();
    let rng = rand::thread_rng();

    let params = GameParameters::default();

    let wordle = Wordle::new_random_game(params, rng);
	let solver = command.solver;

    match command.command {
        Commands::Trial {} => trial_solver(wordle, solver),
        Commands::Sample { count } => repeat_auto_game(wordle, count, solver),
        Commands::Target { target } => run_auto_game(wordle, target, solver),
    }
}

fn run_auto_game(mut wordle: Wordle, target: String, solver: SelectedSolver) {
    println!("Searching for word: {}\n", &target);
    wordle.state.target_word = target;

    let game_words: HashSet<&str> = wordle
        .params
        .answer_wordlist
        .iter()
        .map(|x| *x)
        .collect();

	let mut solver: WordleSolver =  solver.create_solver(game_words);

    loop {
        let (running, word) = take_guess(&mut wordle, &mut solver);
        println!("Guessed: {}", word);
		if !running {
			break;
		}
    }

    println!("Solved!")
}

fn repeat_auto_game(mut wordle: Wordle, played_games: usize, solver: SelectedSolver) {
    let mut guess_count = 0;
    let mut fail_count = 0;

    let bar = ProgressBar::new(played_games.try_into().unwrap());
    for _ in 0..played_games {
        let guesses = auto_game(&mut wordle, &solver);
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

fn trial_solver(mut wordle: Wordle, solver: SelectedSolver) {
    let mut guess_count = 0;
    let mut fail_count = 0;
	let guess_words = wordle.params.answer_wordlist.clone();

	let played_games = guess_words.len() as u64;
    let bar = ProgressBar::new(played_games);
	for word in guess_words.into_iter() {
		wordle = wordle.restart();
		wordle.state.target_word = word.to_string();

        let guesses = auto_game(&mut wordle, &solver);
        guess_count += guesses;
        let failed = guesses > 6;
        fail_count += if failed { 1 } else { 0 };

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

fn auto_game(wordle: &mut Wordle, solver: &SelectedSolver) -> i32 {
    let game_words = wordle
        .params
        .answer_wordlist
        .iter()
        .map(|x| *x)
        .collect::<SolverWordList>();

	let mut solver: WordleSolver = solver.create_solver(game_words);

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

	let guess_word = solver.guess().expect(&format!("Failed to find guess with word {}", &wordle.state.target_word));

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
