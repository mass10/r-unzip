use getopts;

mod application;

fn main() {
	let mut options = getopts::Options::new();
	options.opt("h", "help", "Usage.", "", getopts::HasArg::No, getopts::Occur::Optional);

	let args: Vec<String> = std::env::args().skip(1).collect();
	let result = options.parse(args);
	if result.is_err() {
		println!("Error: {:?}", result);
		return;
	}
	let matches = result.unwrap();

	if matches.free.len() == 0 {
		println!("Error: No path specified.");
		return;
	}

	let path = matches.free.first().unwrap();

	let unzip = application::Unzipper::new();
	let result = unzip.unzip(path);
	if result.is_err() {
		println!("Error: {:?}", result);
		return;
	}
}
