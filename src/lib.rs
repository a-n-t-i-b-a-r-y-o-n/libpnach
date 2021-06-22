pub mod pnach_code;
pub mod pnach_file;
pub mod raw_code;

#[cfg(test)]
mod tests {
	use crate::pnach_file::PNachFile;
	use crate::pnach_code::PNachCode;
	use crate::raw_code;

	#[test]
	fn basic_1() {
		let output = r#"gametitle=Hello World [123456]
// test code 1
patch=1,EE,12345678,extended,09876543

"#;
		let mut pnach = PNachFile::new("Hello World", "123456");
		pnach.add_code(Some("test code 1"), vec![(0x12345678, 0x09876543)]);
		assert_eq!(pnach.to_string(), output);
	}

	#[test]
	fn parse_1() {
		// Example result
		let sample = PNachCode::new(Some("Master code"), vec![(0x12345678, 0x90ABCDEF)]);
		// Input and parsing
		let input = r#"// Master code
12345678 90ABCDEF"#;
		let parsed= raw_code::parse_raw_codes(input).unwrap();

		assert_eq!(parsed[0], sample);
	}

	#[test]
	fn parse_2() {
		// Example result
		let sample = PNachCode::new(Some("Infinite infinity"), vec![(0x12345678, 0x90ABCDEF), (0x90ABCDEF, 0x12345678)]);
		// Input and parsing
		let input = r#"// Infinite infinity
12345678 90ABCDEF
90ABCDEF 12345678"#;
		let parsed= raw_code::parse_raw_codes(input).unwrap();

		assert_eq!(parsed[0], sample);
	}
}
