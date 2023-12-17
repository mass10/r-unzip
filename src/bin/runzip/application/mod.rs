pub struct Unzipper {}

impl Unzipper {
	pub fn new() -> Unzipper {
		Unzipper {}
	}

	pub fn unzip(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		if !path.ends_with(".zip") {
			return Err("Not a zip file.".into());
		}
		
        let path = std::path::Path::new(path);
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
		// let filename = path.file_name().unwrap().to_str().unwrap();

        eprintln!("file_stem: {}", file_stem);

		let mut unzip = zip::ZipArchive::new(std::fs::File::open(path)?)?;
		unzip.extract(file_stem)?;

		return Ok(());
	}
}
