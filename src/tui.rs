use crossterm::{
	terminal,
	cursor,
	style::Print,
	QueueableCommand,
	ExecutableCommand
};
use std::io::{
	self,
	BufWriter,
	Write
};
use crate::symbols::SymbolMap;
type BuffStdout = std::io::BufWriter<std::io::Stdout>;
pub struct TuiCtx {
	cols: u32,
	rows: u32,
	stdout: BuffStdout
}

impl TuiCtx {
	pub fn new() -> Self {
		let (cols, rows) = terminal::size().unwrap();
		Self {
			cols: cols as u32,
			rows: rows as u32,
			stdout: BufWriter::new(io::stdout())
		}
	}
	pub fn enter_raw(&mut self) { // uwu
		terminal::enable_raw_mode().unwrap();
		self.stdout.execute(cursor::Hide).unwrap();
	}
	pub fn exit_raw(&mut self) {
		terminal::disable_raw_mode().unwrap();
		self.stdout.execute(cursor::Show).unwrap();
	}
	fn clear(&mut self) {
		self.stdout
			.queue(terminal::Clear(terminal::ClearType::All)).unwrap()
			.queue(cursor::MoveTo(0, 0)).unwrap();
	}
	fn refresh(&mut self) {
		self.stdout.flush().unwrap();
	}
	pub fn draw_map(&mut self, map: &SymbolMap) {
		self.clear();
		self.stdout.queue(Print(format!("{}", map))).unwrap();
		self.refresh();
	}
}
