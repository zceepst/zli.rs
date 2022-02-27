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

pub fn configure() -> Config {
// read in config parts and assemble config data to modify program behaviour later
    let password = password_prompt();
    let dfuperms = dfu_perms();
    // reads /home dir to parse username to String
    // let output = process::Command::new("ls")
    //     .arg("/home/")
    //     .stdout(process::Stdio::piped())
    //     .output()
    //     .unwrap();
    // let string_out = String::from_utf8(output.stdout).unwrap();
    // let split = string_out.split('\\').collect();
    // let username: &str = String::from(split[0]);
    let username = String::from("pb");
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

// pub fn homepath() -> String {
// 	let output = process::Command::new("ls")
// 		.arg("/home")
// 		.stdout(process::Stdio::piped())
//          .output()
//          .unwrap();
// 	let utf_vec: Vec<u8> = output.stdout;
// 	let n = utf_vec.len() - 1; // -2 = removing '\n' from end, -1 for 0-index mode
// 	let chopped = utf_vec[..n].to_vec();
// 	return String::from_utf8(chopped).unwrap();
// }

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
