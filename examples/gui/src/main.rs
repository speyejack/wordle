mod keyboard;
mod style;

use iced::{
    button, container, text_input, Align, Button, Color, Column, Container, Element, Length, Row,
    Sandbox, Settings, Space, Text, TextInput,
};
use jordle::logic::{
    mutator::{NoopMutator, StepProbMutator},
    CharMatch, GuessResult, WordValidation, Wordle,
};
use keyboard::Keyboard;
use style::{Theme, Tile};

fn main() -> iced::Result {
    WordleGui::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum GameVarient {
    Jordle,
    Fuzzle,
}

struct WordleGui<'a> {
    wordle: Wordle<'a>,
    words: Vec<WordRow>,
    reset_button: button::State,
    next_button: button::State,
    current_varient: GameVarient,
    guess_text: String,
    game_state: GameGuiState,
    keyboard: Keyboard,
}

#[derive(Debug, Clone)]
pub enum GameGuiState {
    Running(text_input::State),
    Finished(button::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    KeyboardButton(char),
    TextSubmitted,
    RestartGame(GameVarient),
}

impl Default for WordleGui<'static> {
    fn default() -> Self {
        WordleGui {
            wordle: Wordle::default(),
            current_varient: GameVarient::Jordle,
            words: vec![],
            game_state: GameGuiState::Running(text_input::State::new()),
            guess_text: String::new(),
            keyboard: Keyboard::new(),
            reset_button: button::State::new(),
            next_button: button::State::new(),
        }
    }
}

impl WordleGui<'_> {
    fn restart(&mut self, varient: GameVarient) {
        let mut wordle = Wordle::default();

        wordle.params.mutator = match varient {
            GameVarient::Jordle => Box::new(NoopMutator::default()),
            GameVarient::Fuzzle => Box::new(StepProbMutator::default()),
        };

        *self = WordleGui {
            wordle,
            current_varient: varient,
            ..WordleGui::default()
        }
    }
}

impl Sandbox for WordleGui<'_> {
    type Message = Message;

    fn new() -> Self {
        let wordle = Wordle::default();

        WordleGui {
            wordle,
            current_varient: GameVarient::Jordle,
            ..WordleGui::default()
        }
    }

    fn title(&self) -> String {
        "Jordle".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout

        let title = container::Container::new(Text::new(self.current_varient.to_string()).size(30))
            .style(Theme::Dark);

        let padding = 4;
        let mut column = Column::new()
            .align_items(Align::Center)
            .spacing(padding)
            .height(Length::Fill)
            .push(title)
            .push(Space::new(Length::Fill, Length::FillPortion(1)));

        for wordrow in &mut self.words {
            column = column.push(wordrow.view())
        }

        let guess_text = format!(
            "{:width$}",
            self.guess_text,
            width = self.wordle.params.word_size.0
        );

        column = column.push(WordRow::render(
            guess_text
                .chars()
                .map(|x| {
                    (
                        x,
                        match x {
                            ' ' => Tile::Empty,
                            _ => Tile::Pending,
                        },
                    )
                })
                .collect(),
        ));

        let reset_button = Button::new(&mut self.reset_button, Text::new("Restart"))
            .on_press(Message::RestartGame(self.current_varient));

        let next_button = Button::new(&mut self.next_button, Text::new("Next"))
            .on_press(Message::RestartGame(self.current_varient.next_varient()));

        let mut footer = Row::new().align_items(Align::Center);
        footer = footer
            .push(reset_button)
            .push(Space::new(Length::Units(4), Length::Fill));

        if let GameGuiState::Running(text_state) = &mut self.game_state {
            footer = footer
                .push(
                    TextInput::new(
                        text_state,
                        "Guess...",
                        &self.guess_text,
                        Message::TextChanged,
                    )
                    .on_submit(Message::TextSubmitted),
                )
                .push(Space::new(Length::Units(4), Length::Fill));
        }

        footer = footer.push(next_button).width(Length::Shrink);

        column = column
            .push(Space::new(Length::Fill, Length::FillPortion(1)))
            .push(footer)
            .push(Space::new(Length::Fill, Length::Units(20)))
            .push(self.keyboard.view())
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
                let result = self.wordle.guess(self.guess_text.as_str());

                if let WordValidation::Valid(guess_result, matches) = result {
                    self.keyboard.update(&matches);
                    let row = WordRow::new(matches.char_matches().collect());
                    self.words.push(row);

                    if let GuessResult::Correct = guess_result {
                        self.game_state = GameGuiState::Finished(button::State::new())
                    }
                }
                self.guess_text = String::default();
            }

            Message::KeyboardButton(c) => {
                self.guess_text.push(c);
            }

            Message::RestartGame(varient) => {
                self.restart(varient);
            }
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
        let matches = self.word.iter().map(|x| (x.c, x.align.into())).collect();

        WordRow::render(matches)
    }

    fn render(matches: Vec<(char, Tile)>) -> Element<'static, Message> {
        // We use a column: a simple vertical layout
        let size = 45;
        let padding = 4;

        let mut row = Row::new()
            .align_items(Align::Center)
            .width(Length::Shrink)
            .spacing(padding);

        for (c, tile) in matches {
            let contain = Container::new(Text::new(c).size(30))
                .style(tile)
                .center_x()
                .center_y()
                .width(Length::Units(size))
                .height(Length::Units(size));
            row = row.push(contain);
        }

        row.into()
    }
}

impl GameVarient {
    fn next_varient(&self) -> GameVarient {
        match self {
            GameVarient::Jordle => GameVarient::Fuzzle,
            GameVarient::Fuzzle => GameVarient::Jordle,
        }
    }
}

impl ToString for GameVarient {
    fn to_string(&self) -> String {
        match self {
            GameVarient::Jordle => "Jordle",
            GameVarient::Fuzzle => "Fuzzle",
        }
        .to_string()
    }
}
