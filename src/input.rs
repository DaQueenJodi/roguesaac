use std::time::Duration;
use crossterm::event::{
	self,
	Event,
	KeyEvent,
	KeyCode::Char
};
use crate::game::GameCtx;
pub fn handle_key_event(key_event: KeyEvent, game_ctx: &mut GameCtx) {
	match key_event.code {
		Char(c) => {
			match c {
				'h' => game_ctx.player.move_left(),
				'j' => game_ctx.player.move_down(),
				'k' => game_ctx.player.move_up(),
				'l' => game_ctx.player.move_right(),
				 _ => {}
			}
		},
		_ => {}
	}
}

pub fn handle_event(game_ctx: &mut GameCtx) -> crossterm::Result<()> {
	if event::poll(Duration::from_millis(100))? {
		match event::read()? {
			Event::Key(key_event) => handle_key_event(key_event, game_ctx),
			_ => {}
		}
	}
	Ok(())
}

