use anyhow::Result;
use iced::{
    button, Align, Button, Color, Column, Container, Element, Length, Row, Sandbox, Settings,
    Space, Text, TextInput,
};

fn main() -> iced::Result {
    Wordle::run(Settings::default())
}

#[derive(Default)]
struct Wordle {
    // The counter value
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Sandbox for Wordle {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Jordle")
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let size = 40;
        let padding = 4;
        let mut base = Column::new()
            .align_items(Align::Center)
            .spacing(padding)
            .push(Space::new(Length::Fill, Length::FillPortion(1)));
        let string = "helo ";
        for _row_i in 0..6 {
            let mut row = Row::new().align_items(Align::Center).spacing(padding);
            for col_c in string.chars() {
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
                    .height(Length::Units(size));
                row = row.push(contain);
            }
            base = base.push(row);
        }

        base.push(Space::new(Length::Fill, Length::FillPortion(1)))
            .into()
    }

    fn update(&mut self, message: Message) {}

    fn background_color(&self) -> Color {
        Color::from_rgb8(18, 18, 19)
    }
}

mod style {
    use iced::{container, Background, Color};

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
