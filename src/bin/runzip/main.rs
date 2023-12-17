//!
//! r-unzip
//!
//! 簡易 unzip ツール
//!

use getopts;

mod application;

struct Configuration {
	pub help: bool,
	pub free: Vec<String>,
}

impl Configuration {
	pub fn new() -> Configuration {
		return Configuration { help: false, free: Vec::new() };
	}

	/// コンフィギュレーション
	pub fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let mut options = getopts::Options::new();

		options.opt("h", "help", "Usage.", "", getopts::HasArg::No, getopts::Occur::Optional);

		let args: Vec<String> = std::env::args().skip(1).collect();
		let result = options.parse(args);
		if result.is_err() {
			let err = result.err().unwrap();
			eprintln!("Error: {}", err);
			eprintln!("{}", options.usage(""));
			std::process::exit(1);
		}

		let matches = result.unwrap();

		self.free = matches.free.clone();

		if matches.opt_present("help") {
			println!("{}", options.usage(""));
			self.help = true;
			return Ok(());
		}

		return Ok(());
	}
}

/// Rust アプリケーションのエントリーポイント
fn main() {
	// コンフィギュレーション
	let mut conf = Configuration::new();
	let result = conf.configure();
	if result.is_err() {
		let err = result.err().unwrap();
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}

	// 残りのオプション
	let free = &conf.free;
	if free.len() == 0 {
		eprintln!("Error: No input file.");
		std::process::exit(1);
	}

	let path = &free[0];

	// unzip 実行
	let unzip = application::Unzipper::new();
	let result = unzip.unzip(path);
	if result.is_err() {
		println!("Error: {:?}", result);
		return;
	}
}
