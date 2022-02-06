use iced::Length;
use iced::Button;
use iced::Text;
use iced::Row;
use iced::Column;
use iced::Element;
use iced::Align;
use iced::button;
use jordle::logic::types::StringMatch;
use crate::Message;
use crate::CharMatch;
use crate::CharAlignment;
use crate::style;
use crate::style::Tile;


pub struct Keyboard {
	rows: Vec<KeyboardRow>
}

impl Keyboard {
	pub fn new() -> Self {
		Self {
			rows: vec![
				KeyboardRow::new("qwertyuiop"),
				KeyboardRow::new("asdfghjkl"),
				KeyboardRow::new("zxcvbnm"),
			],
		}
	}

	pub fn view(&mut self) -> Element<Message> {

		let padding = 4;
        let mut col = Column::new()
            .align_items(Align::Center)
            .width(Length::Shrink)
            .spacing(padding);

		for row in &mut self.rows {
			col = col.push(row.view())
		}

		col.into()
	}

	pub fn update(&mut self, c: &StringMatch) {
		for tc in c {
			for row in &mut self.rows {
				if row.update(tc) {
					break;
				}
			}
		}
	}
}

struct KeyboardRow {
	row: Vec<(char, Tile, button::State)>
}

impl KeyboardRow {
	fn new(row: &str) -> Self {
		Self{
			row: row.chars().map(|c| (c, Tile::Pending,
									  button::State::new())).collect(),
		}
	}

	fn view(&mut self) -> Element<Message> {
		let padding = 4;
		let size = 50;

        let mut row = Row::new()
            .align_items(Align::Center)
            .width(Length::Shrink)
            .spacing(padding);

        for (c, tile, button_state) in &mut self.row {
            let contain = Button::new(button_state,
									  Text::new(*c)
									  .size(30)
									  .horizontal_alignment(iced::HorizontalAlignment::Center)
			)
				.style(*tile)
				.width(Length::Units(size))
				.height(Length::Units(size));
            row = row.push(contain);
        }

        row.into()
	}

	pub fn update(&mut self, cmatch: &CharMatch) -> bool {
		for (c, tile, _) in &mut self.row {
			if *c == cmatch.c {
				match tile {
					Tile::Correct => {},
					_ => {*tile = cmatch.align.into()}
				}
				return true
			}
		}
		return false
	}
}
