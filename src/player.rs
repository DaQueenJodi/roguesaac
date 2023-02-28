use crate::util::Point;
pub struct Player {
	pos: Point
}

impl Player {
	pub fn new() -> Self {
		Self {
			pos: Point::new(0, 0)
		}
	}
}
