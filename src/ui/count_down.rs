use ratatui::{
	layout::Constraint,
	style::{Color, Style},
	text::Line,
	widgets::Clear,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::utils::{centered_rect, rounded_block};
use crate::state::State;

pub fn count_down(f: &mut Frame, state: &State) {
	let area =
		centered_rect(f.size(), Constraint::Length(16), Constraint::Length(10));

	f.render_widget(Clear, area);

	let block = rounded_block(Some("PAUSED"));

	let block_inner = block.inner(area);

	f.render_widget(block, area);

	let text = BigText::builder()
		.pixel_size(PixelSize::Full)
		.lines([Line::raw(state.count_down.to_string())])
		.style(Style::new().fg(Color::White))
		.build()
		.unwrap();

	let center = centered_rect(
		block_inner,
		Constraint::Length(6),
		Constraint::Length(7),
	);

	f.render_widget(text, center);
}
