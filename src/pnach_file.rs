// PNach File - A cheat file for use with the emulator PCSX2
use std::fmt::Write;
use crate::pnach_code::PNachCode;

pub struct PNachFile {
	pub game_title:     String,			// Corresponding game's title
	pub game_crc:       String,			// CRC for corresponding game
	pub codes:          Vec<PNachCode>,	// List of PNach-format codes in file
}

impl ToString for PNachFile {
	fn to_string(&self) -> String {
		let header = format!("gametitle={} [{}]", &self.game_title, &self.game_crc);
		let mut body = String::new();
		&self.codes.iter()
			.for_each(|code| {
				write!(&mut body, "{}", &code.to_string())
					.expect("// Unable to write PNachCode!")
			});
		format!("{}\n{}", header, body)
	}
}

impl PNachFile {
	/// PNach factory
	pub fn new(game_title: &str, game_crc: &str) -> PNachFile {
		let game_title = String::from(game_title);
		let game_crc = String::from(game_crc);
		Self {
			game_title,
			game_crc,
			codes: vec![]
		}
	}
	/// Add individual code to the PNach file
	pub fn add_code(&mut self, title: Option<&str>, codes: Vec<(u32, u32)>) {
		self.codes.push(PNachCode::new(title, codes))
	}

}