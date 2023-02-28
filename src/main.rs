mod symbols;
mod util;
mod player;
mod tui;
use tui::TuiCtx;
use player::Player;
use symbols::Symbol;
fn main() {
	let mut player = Player::new();
	let map = symbol_map!(Symbol::Player, Symbol::Player);
	let mut tui = TuiCtx::new();
	tui.draw_map(&map);
}
