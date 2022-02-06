
use iced::{container, button, Background, Color};

#[allow(dead_code)]
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

#[allow(dead_code)]
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

impl button::StyleSheet for Tile {

	fn active(&self) -> button::Style {
		let grey_border = Color::from_rgb8(58, 58, 60);
		button::Style {
			text_color: iced::Color::WHITE,
			background: match self {
				Tile::Empty => None,
				Tile::NotEntered => None,
				Tile::NotFound => Some(Background::Color(grey_border)),
				Tile::WrongPlace => Some(Background::Color(Color::from_rgb8(0xb5, 0x9f, 0x3b))),
				Tile::Correct => Some(Background::Color(Color::from_rgb8(0x53, 0x8d, 0x4e))),
			},
			border_radius: 1.0,
			border_width: match self {
				Tile::Empty | Tile::NotEntered => 2.0,
				_ => 0.0,
			},
			border_color: match self {
				Tile::NotEntered => Color::from_rgb8(0x56, 0x57, 0x58),
				_ => grey_border,
			},
			..button::Style::default()
		}
	}
}
