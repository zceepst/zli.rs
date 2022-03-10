// if no database exists in current file system
// run SQL script with sqlite3 and create databse
// file with 2 tables: charging and powerhubs

// use json;
use serde_json;
// use serde::Deserialize;

use std::{
	env,
	fs,
	path,
	borrow::Cow
};

// #[derive(Deserialize, Debug)]
struct TrolleyCode {
	product_code: Cow<'static, str>,
	product_type: Cow<'static, str>,
	product: Cow<'static, str>,
	config1: Cow<'static, str>,
	config2: Cow<'static, str>,
	config3: Cow<'static, str>,
	config4: Cow<'static, str>,
	config5: Cow<'static, str>,
	config6: Cow<'static, str>,
	config7: Cow<'static, str>,
	config8: Cow<'static, str>,
	config9: Cow<'static, str>,
	config10: Cow<'static, str>,
	config11: Cow<'static, str>,
	config12: Cow<'static, str>,
	config13: Cow<'static, str>,
}

// #[derive(Deserialize, Debug)]
struct PowerhubCode {
	product_code: Cow<'static, str>,
	product_type: Cow<'static, str>,
	product: Cow<'static, str>,
	config1: Cow<'static, str>,
	config2: Cow<'static, str>,
	config3: Cow<'static, str>,
	config4: Cow<'static, str>,
	config5: Cow<'static, str>,
	config6: Cow<'static, str>,
	config7: Cow<'static, str>,
	config8: Cow<'static, str>,
	config9: Cow<'static, str>,
}

struct Powerhubs {
	powerhubs: 	Vec<PowerhubCode>,
	pcodes:		Vec<Cow<'static, str>>,
}

struct Trolleys {
	trolleys: Vec<TrolleyCode>,
	tcodes: Vec<Cow<'static, str>>,
}

// #[derive(Deserialize, Debug)]
pub struct Products {
	powerhubs: 	Vec<PowerhubCode>,
	pcodes:		Vec<Cow<'static, str>>,
	trolleys: Vec<TrolleyCode>,
	tcodes: Vec<Cow<'static, str>>,
}

// pub fn parse_products() {
// 	let raw = fs::read_to_string("products.json")
// 		.expect("Something went wrong reading 'products.json'");
// 	let parsed = json::parse(&raw[..]); // parse complete slice (&str) of String
// 	println!("{:?}", parsed);
// }

pub fn products() -> Option<serde_json::Value> {
	let raw: String = fs::read_to_string("products.json")
		.expect("Something went wrong reading 'products.json'");
	let products: serde_json::Value = serde_json::from_str(&raw[..])
		.unwrap();
	return Some(products);
}

pub fn create_database() -> u8 {
	return 0;
}
