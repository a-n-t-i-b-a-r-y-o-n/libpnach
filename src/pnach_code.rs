// PNach Code - A line or group of lines in a PNach File
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct PNachCode {
	pub title:      Option<String>,		// Title of specific code
	pub content:    Vec<(u32, u32)>,	// Individual address-value pairs in code
}

impl ToString for PNachCode {
	fn to_string(&self) -> String {
		let title = match &self.title {
			None => String::new(),
			Some(t) => format!("// {}", t)
		};
		let mut body = String::new();
		&self.content.iter()
			.for_each(|(address, value)| {
				write!(&mut body, "patch=1,EE,{:08X},extended,{:08X}\n", address, value)
					.expect("// Unable to print PNachCode contents!")
			});
		format!("{}\n{}\n", title, body)
	}
}

impl PNachCode {
	pub fn new(title: Option<&str>, codes: Vec<(u32, u32)>) -> Self {
		// Double-check that the given title isn't an empty string, or allocate String
		let code_title: Option<String> = match title {
			None => None,
			Some(text) => {
				if text.is_empty() { None }
				else { Some(String::from(text)) }
			}
		};

		Self {
			title:		code_title,
			content:	codes,
		}
	}
}