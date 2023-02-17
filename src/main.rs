pub mod util;
pub mod tui;

use std::{env, fs, io};
use std::io::BufRead;
use tui::TuiCTX;
use std::path::Path;
fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();
	let lines: Vec<String> = if args.len() > 1 {
		let path = Path::new(&args[1]);
		if !path.exists() {
			return Err(format!("{}: No such file or directory", args[1]));
		}
		let file = fs::File::open(&args[1]).unwrap();
		io::BufReader::new(file)
			.lines()
			.map(|x| x.unwrap())
			.collect()
	} else {
		io::stdin()
			.lines()
			.map(|x| x.unwrap())
			.collect()
	};
	let lines: Vec<String> = lines.into_iter()
		.map(|l| l.replace("\t", "  "))
		.collect();
	let mut tui = TuiCTX::new(lines);
	tui.start();
	while !tui.should_quit() {
		tui.draw();
		tui.handle_events();
	}
	tui.end();
	Ok(())
}
