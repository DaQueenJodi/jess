use std::cmp;
#[derive(Clone, Copy)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

#[derive(Clone)]
pub struct Point {
	y: usize,
	x: usize,
}

impl Point {
	pub fn new(x: usize, y: usize) -> Self {
		Self {
			x,
			y
		}
	}
	pub fn x(&self) -> usize {
		self.x
	}
	pub fn y(&self) -> usize {
		self.y
	}
	pub fn set_y(&mut self, y: usize) {
		self.y = y;
	}
	pub fn set_x(&mut self, x: usize) {
		self.x = x;
	}
	pub fn move_dir(&mut self, dir: Direction, times: usize) {
		match dir {
			Direction::Up    => self.y -= cmp::min(times, self.y),
			Direction::Down  => self.y += times,
			Direction::Left  => self.x -= cmp::min(times, self.x),
			Direction::Right => self.x += times,
		}
	}
}
