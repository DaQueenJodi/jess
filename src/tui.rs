use std::io::{self, Write};
use std::cmp;
use crossterm::{
	event::{
		self,
		Event,
		KeyEvent,
		KeyCode,
	},
	style::Print,
	QueueableCommand,
	terminal,
	cursor,
};
use crate::util::{Point, Direction};

// used for 'w' and 'e' movements
const DELIMINATORS: &'static [char] = &[
	' ',
];


pub struct TuiCTX {
	stdout:      io::Stdout,
	cursor:      Point,
	lines:       Vec<String>,
	should_quit: bool,
	scrollx:     usize,
	scrolly:     usize,
	cols:        usize,
	rows:        usize,
}

impl TuiCTX {
	pub fn new(lines: Vec<String>) -> Self {
		let (cols, rows) = terminal::size().unwrap();
		Self {
			lines,
			stdout: io::stdout(),
			cursor: Point::new(0, 0),
			should_quit: false,
			scrollx: 0,
			scrolly: 0,
			cols: cols.into(),
			rows: rows.into(),
		}
	}
	fn refresh(&mut self) {
		self.stdout.flush().unwrap();
	}
	pub fn start(&mut self) {
		terminal::enable_raw_mode().unwrap();
		self.clear();
		self.refresh();
	}
	pub fn end(&mut self) {
		self.clear();
		terminal::disable_raw_mode().unwrap();
		self.stdout.queue(cursor::Show).unwrap();
	}
	pub fn should_quit(&self) -> bool {
		self.should_quit
	}
	pub fn get_cursor(&self) -> Point {
		self.cursor.clone()
	}
	pub fn set_cursor(&mut self, point: Point) {
		self.cursor = point;
	}
	pub fn move_cursor(&mut self, dir: Direction, times: usize) {
		let mut max_times: i32 = match dir {
			Direction::Up    => cmp::min(times, self.cursor.y()) as i32,
			Direction::Down  => cmp::min(times, self.lines.len()
																	 - 1
																	 - self.cursor.y()) as i32,
			Direction::Left  => cmp::min(times, self.cursor.x()) as i32,
			Direction::Right => {
				let line = &self.lines[self.cursor.y()];
				cmp::min(times, line.len()
								 - 1
								 - self.cursor.x()) as i32
			}
		};
		if max_times < 0 { max_times = 0 }
		self.cursor.move_dir(dir, max_times as usize);
		let line = self.current_line();
		let mut line_len: i32 = line.len() as i32 - 1;
		if line_len < 0 { line_len = 0 };
		let new_x = cmp::min(self.cursor.x(), line_len as usize);
		let first_char = line.chars().position(|c| !c.is_whitespace());
		match first_char {
			Some(f) => self.cursor.set_x(cmp::min(
					cmp::max(f, new_x),
					line_len as usize
					)),
			None    => {
				self.move_cursor(dir, 1);
				self.move_cursor(Direction::Left, self.cursor.x());
			}
		}
	}
	fn current_line(&self) -> &str {
		&self.lines[self.cursor.y()]
	}
	fn handle_key_events(&mut self, event: KeyEvent) {
		match event.code {
			KeyCode::Char(c) => {
				match c {
					'q' => self.should_quit = true,
					'h' => self.move_cursor(Direction::Left,  1),
					'j' => self.move_cursor(Direction::Down,  1),
					'k' => self.move_cursor(Direction::Up,    1),
					'l' => self.move_cursor(Direction::Right, 1),
					'w' => {
						let line = self.current_line();
						let times = line.
							chars()
							.skip(self.cursor.x())
							.position(|c| DELIMINATORS.contains(&c))
							.unwrap_or(line.len() - 1);
						self.move_cursor(Direction::Right, times + 1);
					}
					// TODO: change how this acts if you're already on a deliminator
					'b' => {
						let line = self.current_line();
						let times = line
							.chars()
							.rev()
							.take(self.cursor.x())
							.position(|c| DELIMINATORS.contains(&c))
							.unwrap_or(self.cursor.x() + 1);
						self.move_cursor(Direction::Left, times - 1);
					}
					'a' => self.cursor.set_x(self.current_line().len() - 1),
					'_' | '0' => self.move_cursor(Direction::Left, self.cursor.x()),
					_ => (),
				}
			}
			_ => ()
		};
	}
	pub fn handle_events(&mut self) {
		match event::read().unwrap() {
			Event::Key(keyevent) => self.handle_key_events(keyevent),
			_ => (),
		};
		let mut scrolly: i32 = self.cursor.y() -
		if scrolly < 0 { scrolly = 0 }

		self.scrolly = scrolly as usize;
	}
	fn clear(&mut self) {
		self.stdout
			.queue(terminal::Clear(terminal::ClearType::All)).unwrap()
			.queue(cursor::MoveTo(0, 0)).unwrap();
	}
	pub fn draw(&mut self) {
		// hide cursor to prevent screen tearing
		self.stdout.queue(cursor::Hide).unwrap();
		self.clear();
		let rows_max = cmp::min(self.rows, self.lines.len()) as usize;
		for line in &self.lines[self.scrolly..rows_max + self.scrolly - 1] {
			self.stdout
				.queue(Print(line)).unwrap()
				.queue(Print("\r\n")).unwrap();
		}
		let cursor_x = self.cursor.x() - self.scrollx;
		let cursor_y = self.cursor.y() - self.scrolly;
		self.stdout
			.queue(cursor::MoveTo(cursor_x as u16, 
														cursor_y as u16)).unwrap()
			.queue(cursor::Show).unwrap()
			.flush().unwrap();
	}
}
