use std::fmt;

pub enum WallFlavor {
	Horizontal,
	Vertical
}
pub enum Symbol {
	Player,
	Wall(WallFlavor),
	Newline,
	Space,
	Floor,
}

impl fmt::Display for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Symbol::Player => "@",
			Symbol::Wall(flavor) => match flavor {
				WallFlavor::Horizontal => "-",
				WallFlavor::Vertical   => "|"
			},
			Symbol::Newline => "\r\n",
			Symbol::Space => " ",
			Symbol::Floor => "#"
		})
	}
}


pub struct SymbolMap {
	symbols: Vec<Symbol>
}
impl SymbolMap {
	pub fn new() -> Self {
		Self {
			symbols: Vec::new()
		}
	}
	pub fn push(&mut self, symbol: Symbol) {
		self.symbols.push(symbol);
	}
}
#[macro_export]
macro_rules! symbol_map {
	($($symbol:expr),*) => {
		{
			use crate::symbols::SymbolMap;
			let mut map = SymbolMap::new();
			$(
				map.push($symbol);
			 )*
				map
		}
	}
}

impl fmt::Display for SymbolMap {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut string = String::new();
		for symbol in &self.symbols {
			string += &(format!("{}", symbol));
		}
		write!(f, "{}", string)
	}
}
