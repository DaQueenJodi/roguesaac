use crate::util::Point;
pub struct Player {
	pos: Point
}

impl Player {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			pos: Point::new(x, y)
		}
	}
	pub fn x(&self) -> u32 {
		self.pos.x
	}
	pub fn y(&self) -> u32 {
		self.pos.y
	}
	pub fn move_left(&mut self) {
		self.pos.x -= 1;
	}
	pub fn move_down(&mut self) {
		self.pos.y += 1;
	}
	pub fn move_up(&mut self) {
		self.pos.y -= 1;
	}
	pub fn move_right(&mut self) {
		self.pos.x += 1;
	}
}
