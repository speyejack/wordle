use jordle::logic::*;
use jordle::solver::filters::*;
use jordle::solver::scoring::*;
use jordle::solver::*;

use indicatif::ProgressBar;
use std::collections::HashSet;

fn main() {
    let rng = rand::thread_rng();

    let params = GameParameters::default();

	let wordle = Wordle::new_random_game(params, rng);
    repeat_auto_game(wordle);
}

fn repeat_auto_game(mut wordle: Wordle) {
    let played_games = 1_000;

    let mut guess_count = 0;
    let mut fail_count = 0;

    let bar = ProgressBar::new(played_games);
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
    let mut game_words = wordle.params.answer_wordlist.clone().into_iter().collect::<HashSet<_>>();
    let mut guess_count = 0;

    loop {
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
        guess_count += 1;

        match guess_result {
            WordValidation::Valid(result, matches) => {
                if let GuessResult::Correct = result {
                    return guess_count;
                }

                let filter = PosFilterCriteria::from_matches(&matches);
                let new_words: HashSet<String> = game_words
                    .into_iter()
                    .filter(|word| filter.check(word))
                    .collect();

                game_words = new_words;
            }
            WordValidation::Invalid(_, _) => unreachable!(),
        }
    }
}
