use crate::symbols::{SymbolMap, Symbol, WallFlavor};
use crate::player::Player;
pub struct Room {
	pub size_y: u32,
	pub size_x: u32,
}

impl Room {
	pub fn to_symbol_map(&self, player: &Player) -> SymbolMap {
		let mut map = SymbolMap::new();
		for y in 0..self.size_y {
			for x in 0..self.size_x {
				if y == player.y() && x == player.x() {
					map.push(Symbol::Player);
				}
				else if x == 0 || x == self.size_x - 1 {
						map.push(Symbol::Wall(WallFlavor::Vertical));
				}
				else if y == 0 || y == self.size_y - 1{
					if x != 0 {
						map.push(Symbol::Wall(WallFlavor::Horizontal));
					}
				}
				else {
					map.push(Symbol::Floor);
				}
			}
			map.push(Symbol::Newline);
		}
		map
	}
}
