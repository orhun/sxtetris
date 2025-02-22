use anyhow::Result;

use crate::{
	handler::{GameEvent, MainHandler},
	save::Save,
	state::State,
	term::Term,
	ui::ui,
};

pub struct App {
	term: Term,
	handler: MainHandler,
	state: State,
	save: Save,
}

impl App {
	pub fn new() -> Result<Self> {
		let mut save = Save::new();

		save.read()?;

		let term = Term::new()?;
		let handler = MainHandler::new();
		let mut state = State::new(handler.create_sub_handler());

		state.read_save(&save);

		Ok(Self {
			term,
			handler,
			state,
			save,
		})
	}

	pub async fn run(&mut self) -> Result<()> {
		self.term.init()?;

		self.term.draw(|f| {
			ui(f, &self.state);
		})?;

		while let Some(event) = self.handler.recv().await {
			if event == GameEvent::CtrlC {
				break;
			}

			self.state.receive_event(event);

			self.term.draw(|f| {
				ui(f, &self.state);
			})?;

			if !self.state.running {
				break;
			}
		}

		self.save.write(&self.state)?;

		self.term.exit()?;

		Ok(())
	}
}
