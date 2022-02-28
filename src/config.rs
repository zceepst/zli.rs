// Read and produce runtime config

use rpassword::read_password;
use std::fs; // file to string parsing
use std::io;
use std::io::Write;
use std::process;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    name: String,
    pass: String,
    dfu: bool,
}

// read in config parts and assemble config data to modify program behaviour later
pub fn configure() -> Config {
    let password = password_prompt();
    let dfuperms = dfu_perms();
    let username = username();
    return Config { name: username, pass: password, dfu: dfuperms };
}

// prompt user for password, hidden input from rpassword
pub fn password_prompt() -> String {
    println!("Password:");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    return password;
}

// parse DFU permissions file from `dfuperms` text config file
pub fn dfu_perms() -> bool {
    let string = fs::read_to_string("dfuperms")
        .expect("Something went wrong reading the dfuperms file");
    let val: i32 = string.parse().unwrap();
    return if val == 0 { false } else { true };
}

// get system username (unix) from home directory
pub fn username() -> String {
	let vec = process::Command::new("ls")
		.arg("/home")
		.stdout(process::Stdio::piped())
        .output()
        .unwrap()
		.stdout;
	let chopped = vec[..(vec.len() - 1)].to_vec(); // index-away '\n' terminating chars
	return String::from_utf8(chopped).unwrap();
}
