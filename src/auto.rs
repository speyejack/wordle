mod logic;
mod solver;

use logic::game::*;
use solver::filters::*;
use solver::scoring::*;
use solver::*;

use anyhow::Result;
use indicatif::ProgressBar;
use rand::Rng;
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let answer_wordlist = load_wordlist("/home/jack/Documents/jordle/words/answers.txt", 5)?;

    let params = setup_game(&answer_wordlist)?;

    play_auto_game(params, &mut rng, &answer_wordlist)
}

fn play_auto_game(
    params: GameParameters,
    rng: &mut impl Rng,
    answer_wordlist: &HashSet<String>,
) -> Result<()> {
    let played_games = 1_000;

    let mut guess_count = 0;
    let mut fail_count = 0;

    let bar = ProgressBar::new(played_games);
    for _ in 0..played_games {
        let target_word = gen_target_word(answer_wordlist, rng);

        let guesses = auto_game(&params, rng, answer_wordlist, &target_word)?;
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

    Ok(())
}

fn auto_game(
    params: &GameParameters,
    rng: &mut impl Rng,
    answer_wordlist: &HashSet<String>,
    target_word: &str,
) -> Result<i32> {
    let mut game_words = answer_wordlist.clone();
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

        let guess_result = guess_word(
            &word,
            &params.guess_wordlist,
            target_word,
            rng,
            (params.word_size, params.word_size),
        )?;
        guess_count += 1;

        match guess_result {
            WordValidation::Valid(result, matches) => {
                if let GuessResult::Correct = result {
                    return Ok(guess_count);
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
