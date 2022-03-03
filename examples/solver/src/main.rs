use clap::{ArgEnum, Parser, Subcommand};
use jordle::{
    logic::{mutator::NoopMutator, params::GameParameters, *},
    solver::{
        entropy::{EntropySolver, GlobalFilteredEntropySolver, GlobalShrinkingEntropySolver},
        positional::PositionalSolver,
        solvers::{Solver, SolverWordList},
    },
};

use indicatif::{ProgressBar, ProgressStyle};

type WordleSolver<'a> = Box<dyn jordle::solver::solvers::Solver<'a> + 'a>;

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
    ShrinkingEntropy,
    FilteredEntropy,
}

impl<'a> SelectedSolver {
    fn create_solver(&self, wordlist: &SolverWordList<'a>) -> Box<dyn Solver<'a> + 'a> {
        match self {
            Self::Positional => Box::new(PositionalSolver::new(wordlist)),
            Self::Entropy => Box::new(EntropySolver::new(wordlist)),
            Self::ShrinkingEntropy => Box::new(GlobalShrinkingEntropySolver::new(wordlist)),
            Self::FilteredEntropy => Box::new(GlobalFilteredEntropySolver::new(wordlist)),
        }
    }
}

fn create_simple_game_params() -> GameParameters<'static> {
    let word_size = (5, 5);
    let filter_words =
        |x: &'static str| Some(x).filter(|x| x.len() >= word_size.0 && x.len() <= word_size.1);

    let word_str = include_str!("../../../words/sample.txt");
    // let word_str = include_str!("../../../words/repeats.txt");
    // let word_str = include_str!("../../../words/super_simple.txt");

    let answer_wordlist: Vec<&str> = word_str.lines().filter_map(filter_words).collect();

    let guess_wordlist = answer_wordlist.iter().map(|x| *x).collect();

    GameParameters {
        guess_wordlist,
        answer_wordlist,
        word_size,

        mutator: Box::new(NoopMutator {}),
        tries: Some(6),
        time_limit: None,
    }
}

fn main() {
    let command = Cli::parse();
    let rng = rand::thread_rng();

    // let params = create_simple_game_params();
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

    let game_words: SolverWordList = wordle.params.answer_wordlist.iter().map(|x| *x).collect();

    let mut solver: WordleSolver = solver.create_solver(&game_words);

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
    let words = wordle.params.answer_wordlist.clone();
    let mut solver = solver.create_solver(&words);

    let bar = ProgressBar::new(played_games.try_into().unwrap());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}/{eta_precise}] {bar:cyan/blue} {pos:>7}/{len:7} {msg}"),
    );

    for _ in 0..played_games {
        let guesses = auto_game(&mut wordle, &mut solver);
        guess_count += guesses;
        let failed = guesses > 6;
        fail_count += if failed { 1 } else { 0 };

        wordle = wordle.restart();
        solver.reload_wordlist(&words);
        bar.set_message(format!("Failed: {}", fail_count));
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
    let target_words: Vec<&str> = wordle.params.answer_wordlist.clone();
    let played_games = target_words.len() as u64;
    let bar = ProgressBar::new(played_games);

    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed_precise}/{eta_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}",
    ));

    let guessing_words = wordle.params.answer_wordlist.clone();
    let mut solver = solver.create_solver(&guessing_words);
    let mut failed_words = Vec::new();

    println!("Game loaded, beginning trial");

    for target_word in target_words.into_iter() {
        wordle = wordle.restart_with_target(target_word.to_string());

        let guesses = auto_game(&mut wordle, &mut solver);
        guess_count += guesses;
        let failed = guesses > 6;
        if failed {
            failed_words.push((guesses, target_word));
        }

        solver.reload_wordlist(&guessing_words);
        bar.set_message(format!(
            "Failed: {} ({:?})",
            failed_words.len(),
            failed_words.last()
        ));
        bar.inc(1);
    }
    bar.finish();

    dbg!(&failed_words);

    println!(
        "Played {} games, with {} avg and {} failures",
        played_games,
        guess_count as f32 / played_games as f32,
        failed_words.len()
    );
}

fn auto_game<'a>(wordle: &'a mut Wordle, solver: &mut WordleSolver<'a>) -> i32 {
    let mut guess_count = 0;

    loop {
        guess_count += 1;
        let (running, _) = take_guess(wordle, solver);

        if !running {
            break;
        }
    }

    guess_count
}

fn take_guess<'a>(wordle: &'a mut Wordle, solver: &mut WordleSolver<'a>) -> (bool, String) {
    let guess_word = solver.guess().expect(&format!(
        "Failed to find guess with word {}",
        &wordle.state.target_word
    ));

    let guess_result = wordle.guess(&guess_word);

    match guess_result {
        WordValidation::Valid(result, matches) => {
            if let GuessResult::Correct = result {
                return (false, guess_word);
            }


            solver.narrow_words(&matches);

            return (true, guess_word);
        }
        WordValidation::Invalid(_, _) => {
            println!("Guessed an invalid word: {}", &guess_word);
            unreachable!()
        }
    }
}
