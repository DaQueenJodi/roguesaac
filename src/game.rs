use crate::player::Player;
pub struct GameCtx {
	pub player: Player
}

impl GameCtx {
	pub fn new(player: Player) -> Self {
		Self {
			player
		}
	}
}
