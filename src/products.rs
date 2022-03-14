use serde_json;
use std::fs;

pub fn products() -> Option<serde_json::Value> {
	let raw: String = fs::read_to_string("products.json")
		.expect("Something went wrong reading 'products.json'");
	let products: serde_json::Value = serde_json::from_str(&raw[..])
		.unwrap();

	return Some(products);
}

pub fn unpack_json_to_vecstring(json_value: &serde_json::Value, key: &str) -> Vec<String> {
	let mut out: Vec<String> = Vec::new();
	let codes = json_value[key]
		.as_array()
		.unwrap();
	let scope = codes.len();
	for i in 0..scope {
		out.push(
			String::from(
				codes[i]
					.as_str()
					.unwrap()
			)
		);
	}
	return out;
}

pub fn is_trolley_code(code: &String) -> Option<bool> {
	if &code[0..3] == "CHR" || &code[0..3] == "NET" {
		return Some(true);
	} else if &code[0..3] == "HUB" || &code[0..3] == "HT4" {
		return Some(false);
	} else {
		return None;
	}
}

pub fn is_stored_product(code: &String, key: &str) -> bool {
	let data: serde_json::Value = products().unwrap();
	let tcodes: Vec<String> = unpack_json_to_vecstring(&data, key);
	let result: bool = tcodes
		.iter()
		.any(|e| e == code);
	return result;
}

// search and retrieve known product config from code
// and family
pub fn search_for_config(code: &String, data: serde_json::Value, code_key: &str, product_key: &str) {
	let tcodes: Vec<String> = unpack_json_to_vecstring(&data, code_key);
	let trolleys = &data[product_key]
		.as_array()
		.unwrap()
		.to_vec();

	let mut index = 0;
	let scope = tcodes.len();

	for i in 0..scope {
		if code != &tcodes[i] {
			index += 1;
		} else if code == &tcodes[i] {
			break;
		}
	}
	// return trolleys[..].to_vec()[index];
	println!("{:#?}", trolleys.to_vec());
}

// read json product codes
// determine if trolley/powerhub input code
// determine if product code => config vals are stored/known
// if known, search for product code config object in data
//
pub fn main_config_generator(code: String) {
    let data: serde_json::Value = products().unwrap();
	let is_trolley = is_trolley_code(&code).unwrap();
	if is_trolley {
		let stored: bool = is_stored_product(&code, "tcodes");
		if stored {
			// println!("{:#?}", 0);
			search_for_config(&code, data, r#"tcodes"#, r#"trolleys"#);
		} else {
			println!("Not stored, need insertion/new config def. functionality!");
		}
		// println!("Stored: {:#?}", stored);
		// println!("Trolley");
	} else {
		let stored: bool = is_stored_product(&code, "pcodes");
		println!("Stored: {:#?}", stored);
		println!("Powerhub");
	}
}

// equivalent to Julia's: `in(collection, element)::Bool`
// vec.iter().any(|e| e == &String::from("no")) -> bool
