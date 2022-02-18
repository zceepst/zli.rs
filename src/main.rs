use std::env; // command line argument invocation
use std::fs; // file to string parsing
use std::process::{Command, Stdio}; // run shell commands

extern crate rpassword;
use rpassword::read_password; // hide password input
use std::io::Write; // write to files

pub struct Config {
    name: String,
    pass: String,
    dfu: bool,
}

pub struct Args {
    photon: bool,
    sd: bool,
    auto: bool,

}

fn main() {
    let _config = configure();
    // test_print(config);
    capture_command_output(String::from("pwd"), String::from("."));
}

fn _args() {
    let args: Vec<String> = env::args().collect();

}

fn capture_command_output(cmd: String, args: String) {
    let output = Command::new(cmd)
        .arg(args)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("Command output:\n{}", stdout);
}

#[allow(dead_code)]
fn test_print(t1: Config) {
    println!("Username: {}", t1.name);
    println!("Password: {}", t1.pass);
    println!("DFU permissions: {}", t1.dfu);
}

// read in config parts and assemble config data to modify program behaviour later
fn configure() -> Config {
    let password = password_prompt();
    let dfuperms = parse_dfu_perms();
    let username = String::from("pb");
    let config = Config { name: username, pass: password, dfu: dfuperms };
    return config;
}

// prompt user for password, hidden input from rpassword
fn password_prompt() -> String {
    println!("Password:");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    return password;
}

// parse DFU permissions file from `dfuperms` text config file
fn parse_dfu_perms() -> bool {
    let string = fs::read_to_string("dfuperms")
        .expect("Something went wrong reading the dfuperms file");
    let val: i32 = string.parse().unwrap();
    let out: bool;
    if val == 0 {
        out = false;
    } else {
        out = true;
    }
    return out;
}
