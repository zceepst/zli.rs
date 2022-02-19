use std::env; // command line argument invocation
use std::fs; // file to string parsing
use std::process::{Command, Stdio}; // run shell commands
use std::io::Write; // write to files

extern crate clap;
use clap::{Arg, App};
extern crate rpassword;
use rpassword::read_password;

mod menu;

#[allow(dead_code)]
enum Args {
    Auto,
    Photon,
    SD,
    None,
}

#[allow(dead_code)]
struct RunConf {
    name: String,
    pass: String,
    dfu: bool,
    cmd: Args,
    spec: String,
    frmw: String,
}

struct Config {
    name: String,
    pass: String,
    dfu: bool,
}

fn main() {
    // let _config = configure();
    // parse_args();
    // menu::auto();
    // test_print(config);
    // capture_command_output(String::from("pwd"), String::from("."));
        let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::new("file")
                 .short('f')
                 .long("file")
                 .takes_value(true)
                 .help("A cool file"))
        .arg(Arg::new("num")
                 .short('n')
                 .long("number")
                 .takes_value(true)
                 .help("Five less than your favorite number"))
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}

fn parse_args() {
    let args: Vec<String> = env::args().collect();
    // println!("{}", args.len())
    if args.len() == 1 || args.len() > 5 {
        // arguments out of bound
        menu::error();
        menu::help();
    } else {
        // args in bound, proceed to decompose
        let arg1 = &args[1];
        match arg1.as_str() {
            "auto" => {println!("auto");},
            "photon" => {println!("photon");},
            "sd" => {println!("sd");},
            _ => {menu::error();},
        }
    }
}

#[allow(dead_code)]
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
