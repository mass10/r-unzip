//!
//! r-unzip
//!
//! 簡易 unzip ツール
//!

use getopts;

mod application;

/// コンフィギュレーション
struct Configuration {
	options: getopts::Options,

	/// --help: ヘルプ表示
	pub help: bool,

	/// 残りのオプション
	pub free: Vec<String>,
}

impl Configuration {
	/// コンストラクタ
	///
	/// # Returns
	/// 新しいインスタンスを返します。
	pub fn new() -> Configuration {
		let mut options = getopts::Options::new();
		// --help: ヘルプ
		options.opt("h", "help", "Usage.", "", getopts::HasArg::No, getopts::Occur::Optional);

		return Configuration {
			options: options,
			help: false,
			free: Vec::new(),
		};
	}

	/// 使用方法を表示します。
	pub fn usage(&self) {
		eprintln!("{}", self.options.usage(""));
	}

	/// コンフィギュレーション
	pub fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let args: Vec<String> = std::env::args().skip(1).collect();
		let matches = self.options.parse(args)?;

		// --help
		self.help = matches.opt_present("help");

		// 残りのオプション
		self.free = matches.free.clone();

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
		let message = err.to_string();
		if message == "" {
			// 計画された正常終了です。
			return;
		}
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}

	// ヘルプ表示
	if conf.help {
		conf.usage();
		return;
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
