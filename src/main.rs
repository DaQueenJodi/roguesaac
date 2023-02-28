mod symbols;
mod util;
mod player;
mod tui;
mod room;
mod input;
mod game;
use room::Room;
use tui::TuiCtx;
use player::Player;
use game::GameCtx;
use symbols::Symbol;
fn main() {
	let room = Room {
		size_y: 10,
		size_x: 10,
	};
	let player = Player::new(2, 4);
	let mut game_ctx = GameCtx::new(player);
	let mut tui = TuiCtx::new();
	tui.enter_raw();
	loop {
		input::handle_event(&mut game_ctx).unwrap();
		let map = room.to_symbol_map(&game_ctx.player);
		tui.draw_map(&map);
	}
	tui.exit_raw();
}
