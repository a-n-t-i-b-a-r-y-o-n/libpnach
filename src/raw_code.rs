use crate::pnach_code::PNachCode;
use regex::Regex;
use std::ops::Index;

pub fn parse_raw_codes(input: &str) -> Result<Vec<PNachCode>, Box<dyn std::error::Error>> {
	// Comments only
	let comment_regex = Regex::new(r#"[/#;]+[\s\S]+?"#).expect("Error creating comment regex?");
	// Address *and* value regex, i.e. 2x 32-bit words in hex separated by whitespace
	let words_regex = Regex::new(r#"((?:[0-9]|[A-F]|[a-f]){8})\s+((?:[0-9]|[A-F]|[a-f]){8})"#).expect("Error creating words regex?");
	// Regular expression for a group of 2 columns of 8-digit hex codes w/ optional ascii comment lines
	let code_regex = Regex::new(r#"([/#;]+[\s\S]+?|(\s{2,}))+(((?:[0-9]|[A-F]|[a-f]){8})\s+((?:[0-9]|[A-F]|[a-f]){8})(\s?))+"#).expect("Error creating code regex?");
	// Output vec
	let mut output: Vec<PNachCode> = vec![];
	// Push all captured codes to output vec
	code_regex.captures_iter(input)
		// Iterate chunks of text
		.for_each(|chunk| {
			// Create a new empty PNachCode object
			let mut code = PNachCode::new(None, vec![]);
			// Iterate through lines of this chunk's text (group 0)
			for line in chunk.index(0).lines() {
				// Parse comment line
				if comment_regex.is_match(line) {
					// Use only the top comment line as the title
					if code.title == None {
						// Remove the comment indicator on the line
						let capture = comment_regex.captures_iter(line).nth(0).expect("");
						let indicator = capture.index(0);
						code.title = Some(String::from(line.trim_start_matches(indicator)));
					}
				}
				else {
					// Iterate through code matches
					words_regex.captures_iter(line).for_each(|subgroups| {
						// Get address & value from match groups as strings
						let address_str = subgroups.iter().nth(1)
							.expect("").expect("")
							.as_str();
						let value_str = subgroups.iter().nth(2)
							.expect("").expect("")
							.as_str();
						// Parse address and value from hex
						let value = u32::from_str_radix(value_str, 16).expect("");
						let address = u32::from_str_radix(address_str, 16).expect("");
						// Push codes to list for this PNachCode
						code.content.push(
							(address, value)
						);
					});
				}
			}

			// Push this new PNachCode to the output
			output.push(code)
		});

	Ok(output)
}