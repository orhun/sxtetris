use ratatui::{
	layout::{Constraint, Flex, Layout, Rect},
	style::{Color, Style},
	text::Line,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::{board::board, utils::rounded_block};
use crate::state::State;

pub fn sidebar(
	f: &mut Frame,
	rect: Rect,
	state: &State,
	cell_height: u16,
	cell_width: u16,
) {
	let sidebar_blcok = rounded_block(None);

	let sidebar_area = sidebar_blcok.inner(rect);

	f.render_widget(sidebar_blcok, rect);

	let vertical_chunks =
		Layout::vertical([Constraint::Ratio(1, 4); 4]).areas::<4>(sidebar_area);

	let chunks_0 = Layout::vertical([
		Constraint::Length(4),
		Constraint::Length(cell_height * (state.preview_board.rows as u16) + 1),
	])
	.spacing(1)
	.flex(Flex::Center)
	.areas::<2>(vertical_chunks[0]);

	render_text(f, "NEXT".to_string(), chunks_0[0], Color::Blue);
	board(f, chunks_0[1], state, cell_height, cell_width, false);

	let chunks_1 = create_text_chunks(vertical_chunks[1]);
	render_text(f, "SCORE".to_string(), chunks_1[0], Color::Blue);
	render_text(f, state.score.to_string(), chunks_1[1], Color::White);

	let chunks_2 = create_text_chunks(vertical_chunks[2]);
	render_text(f, "LEVEL".to_string(), chunks_2[0], Color::Blue);
	render_text(f, state.level.to_string(), chunks_2[1], Color::White);

	let chunks_3 = create_text_chunks(vertical_chunks[3]);
	render_text(f, "LINES".to_string(), chunks_3[0], Color::Blue);
	render_text(f, state.lines.to_string(), chunks_3[1], Color::White);
}

fn create_text_chunks(rect: Rect) -> [Rect; 2] {
	Layout::vertical([Constraint::Length(4), Constraint::Length(4)])
		.spacing(1)
		.flex(Flex::Center)
		.areas::<2>(rect)
}

fn render_text(f: &mut Frame, text: String, rect: Rect, color: Color) {
	let area = Layout::horizontal([Constraint::Length(text.len() as u16 * 4)])
		.flex(Flex::Center)
		.areas::<1>(rect)[0];

	let title = BigText::builder()
		.pixel_size(PixelSize::Quadrant)
		.lines([Line::raw(text).style(Style::new().fg(color))])
		.build()
		.unwrap();

	f.render_widget(title, area);
}
