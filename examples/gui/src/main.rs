use std::{collections::HashSet, time::Duration};
use rand::prelude::*;
use jordle::logic;

use iced::{
    text_input, Align, Color, Column, Container, Element, Length, Row, Sandbox,
    Settings, Space, Text, TextInput, button, Button,
};
use logic::game::{gen_target_word, GameParameters, guess_word, CharMatch, WordValidation, GuessResult, CharAlignment};

fn main() -> iced::Result {
    Wordle::run(Settings::default())
}

struct Wordle {
    words: Vec<WordRow>,
    guess_text: String,
    target_word: String,
    params: GameParameters,
	rng: ThreadRng,
	game_state: GameState,
}

#[derive(Debug, Clone)]
pub enum GameState {
	Running(text_input::State),
	Finished(button::State)
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    TextSubmitted,
    RestartGame,
}

impl Sandbox for Wordle {
    type Message = Message;

    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let answer_wordlist = include_str!("../../../words/answers.txt");

        let guess_wordlist = include_str!("../../../words/guesses.txt")
            .lines()
            .filter(|x| x.len() == 5)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

		let target_word = answer_wordlist.lines().choose(&mut rng).unwrap().to_string();

        let params = GameParameters {
            tries: 10,
            word_size: 5,
            time_limit: Duration::from_secs(60 * 60),
            guess_wordlist,
        };

        Wordle {
            words: vec![],
            game_state: GameState::Running(text_input::State::new()),
            guess_text: String::new(),
			target_word,
			params,
			rng,
        }
    }

    fn title(&self) -> String {
        "Jordle".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout

        let size = 40;
        let padding = 4;
        let mut column = Column::new()
            .align_items(Align::Center)
            .spacing(padding)
            .height(Length::Fill)
            .push(Space::new(Length::Fill, Length::FillPortion(1)));

        for wordrow in &mut self.words {
            column = column.push(wordrow.view())
        }

		let footer: Element<Message> = match &mut self.game_state {
			GameState::Running(text_state) => {
				TextInput::new(
					text_state,
					"Guess...",
					&self.guess_text,
					Message::TextChanged,
				)
					.on_submit(Message::TextSubmitted)
					.width(Length::Shrink)
					.into()
			}
			GameState::Finished(state) => {
				Button::new(state, Text::new("Restart"))
					.on_press(Message::RestartGame)
					.into()
			}
		};

		column = column
            .push(Space::new(Length::Fill, Length::FillPortion(1)))
			.push(footer)
            .push(Space::new(Length::Fill, Length::Units(40)));

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Theme::Dark)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextChanged(string) => {
                self.guess_text = string.to_lowercase().trim().to_string();
            }
            Message::TextSubmitted => {
				let text = &self.guess_text;
				let result = guess_word(&text,
										&self.params.guess_wordlist,
										&self.target_word, &mut self.rng, (5,5));

				if let Ok(WordValidation::Valid(guess_result,
												matches)) = result {
					let row = WordRow::new(matches);
					self.words.push(row);

					if let GuessResult::Correct = guess_result {
						self.game_state = GameState::Finished(button::State::new())
					}
					// self.running = match guess_result {
					// 	GuessResult::Correct => false,
					// 	GuessResult::Wrong => true,
					// }
				}
                self.guess_text = String::default();
            }
			Message::RestartGame => {}
        }
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(18, 18, 19)
    }
}

struct WordRow {
    word: Vec<CharMatch>,
}

impl WordRow {
    fn new(word: Vec<CharMatch>) -> Self {
        WordRow { word }
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let size = 45;
        let padding = 4;

        let mut row = Row::new()
            .align_items(Align::Center)
            .width(Length::Shrink)
            .spacing(padding);

        for char_match in &self.word {
			let contain =
				Container::new(Text::new(char_match.c).size(30))
				.style(match &char_match.align {
                    CharAlignment::Exact => style::Tile::Correct,
                    CharAlignment::Misplaced => style::Tile::WrongPlace,
                    CharAlignment::NotFound  => style::Tile::NotFound,
                })
                .center_x()
                .center_y()
                .width(Length::Units(size))
                .height(Length::Units(size));
            row = row.push(contain);
        }

        row.into()
    }
}

mod style {
    use iced::{container, Background, Color};

    pub enum Theme {
        Light,
        Dark,
    }

    impl container::StyleSheet for Theme {
        fn style(&self) -> container::Style {
            let color = match *self {
                Theme::Light => Color::WHITE,
                Theme::Dark => Color::from_rgb8(0x12, 0x12, 0x13),
            };

            container::Style {
                background: Some(Background::Color(color)),

                ..container::Style::default()
            }
        }
    }

    pub enum Tile {
        Empty,
        NotEntered,
        NotFound,
        WrongPlace,
        Correct,
    }

    impl container::StyleSheet for Tile {
        fn style(&self) -> container::Style {
            let grey_border = Color::from_rgb8(58, 58, 60);
            container::Style {
                text_color: Some(Color::WHITE),
                background: match self {
                    Tile::Empty => None,
                    Tile::NotEntered => None,
                    Tile::NotFound => Some(Background::Color(grey_border)),
                    Tile::WrongPlace => Some(Background::Color(Color::from_rgb8(0xb5, 0x9f, 0x3b))),
                    Tile::Correct => Some(Background::Color(Color::from_rgb8(0x53, 0x8d, 0x4e))),
                },
                border_radius: 1.0,
                border_width: 100.0,
                // border_width: match self {
                //     Tile::Empty | Tile::NotEntered => 2000.0,
                //     _ => 1000.0,
                // },
                border_color: match self {
                    Tile::NotEntered => Color::from_rgb8(0x56, 0x57, 0x58),
                    _ => grey_border,
                },
                ..container::Style::default()
            }
        }
    }
}