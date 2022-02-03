use web_sys::console;
use anyhow::Result;
use iced::{
    button, Align, Button, Color, Column, Container, Element, Length, Row, Sandbox, Settings,
    Space, Text, TextInput,
};

fn main() -> iced::Result {
	console::log_1(&"Hello using web-sys".into());
    Wordle::run(Settings::default())
}

struct Wordle {
	words: Vec<WordRow>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Sandbox for Wordle {
    type Message = Message;

    fn new() -> Self {
        // Self::default()
		Wordle {
			words: vec![
				WordRow::new("helo ".to_string()),
				WordRow::new("helo ".to_string()),
				WordRow::new("helo ".to_string()),
				WordRow::new("helo ".to_string()),
				WordRow::new("helo ".to_string()),
				WordRow::new("helo ".to_string()),
			]
		}
    }

    fn title(&self) -> String {
        "Jordle".to_string()
    }

    fn view(&mut self) -> Element<Message> {
		console::log_1(&"Getting to view".into());
        // We use a column: a simple vertical layout
        let size = 40;
        let padding = 4;
        let mut base = Column::new()
            .align_items(Align::Center)
            .spacing(padding)
            .push(Space::new(Length::Fill, Length::FillPortion(1)));

		for wordrow in &mut self.words {
			base = base.push(wordrow.view())
		}

        let base = base
			.push(Space::new(Length::Fill, Length::FillPortion(1)));

		Container::new(base)
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.style(style::Theme::Dark)
			.into()
	}

    fn update(&mut self, message: Message) {}

    fn background_color(&self) -> Color {
        Color::from_rgb8(18, 18, 19)
    }
}


struct WordRow {
	word: String,
}


impl WordRow {

    fn new(word: String) -> Self {
        WordRow{word}
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let size = 40;
        let padding = 4;

		let mut row = Row::new()
			.align_items(Align::Center)
			.spacing(padding);

		for col_c in self.word.chars() {
			let contain = Container::new(Text::new(col_c).size(30))
				.style(match col_c {
					'h' => style::Tile::Correct,
					'e' => style::Tile::WrongPlace,
					'l' => style::Tile::NotFound,
					'o' => style::Tile::NotEntered,
					_ => style::Tile::Empty,
				})
				.center_x()
				.center_y()
				.width(Length::Units(size))
				.height(Length::Units(size)) ;
			row = row.push(contain);
		}

		row.into()
	}

    fn update(&mut self, message: Message) {}

    fn background_color(&self) -> Color {
        Color::from_rgb8(18, 18, 19)
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

			container::Style{
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
                border_radius: 0.0,
                border_width: match self {
                    Tile::Empty | Tile::NotEntered => 2.0,
                    _ => 0.0,
                },
                border_color: match self {
                    Tile::NotEntered => Color::from_rgb8(0x56, 0x57, 0x58),
                    _ => grey_border,
                },
				..container::Style::default()
            }
        }
    }
}
