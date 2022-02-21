// use std::env; // command line argument invocation
use std::fs; // file to string parsing
use std::process; // run shell commands
use std::path; // directory tree utils
use std::io::Write; // write to files

extern crate clap;
extern crate rpassword;
use rpassword::read_password;

mod menu; // import help menus (depreciated)

#[allow(dead_code)]
enum Cmd {
    Auto,
    Photon,
    SD,
    None,
}

struct Args {
    cmd: Cmd,
    arg: String,
}

#[allow(dead_code)]
struct RunConf {
    name: String,
    pass: String,
    dfu: bool,
    cmd: Cmd,
    spec: String,
    frmw: String,
}

struct Config {
    name: String,
    pass: String,
    dfu: bool,
}

fn main() {
    let args: Args = arg_parse(); // user arguments parsed
    println!("Output:\n{}", args.arg);
    // let _config = configure();
    // parse_args();
    // menu::auto();
    // test_print(config);
    // capture_command_output(String::from("pwd"), String::from("."));
}

fn arg_parse() -> Args {
    let matches = clap::Command::new("zli")
        .about("Production automation tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            clap::Command::new("auto")
                .about("One-shot product setup, photon & sd card")
                .args(&[
                    clap::arg!(<PRODUCT_CODE> "Product code"),
                    clap::arg!([SERIAL_ID] "Device serial ID e.g. zcts100054")
                ])
                .arg_required_else_help(true),
        )
        .subcommand(
            clap::Command::new("photon")
                .about("Photon firmware and device-OS flash")
                .arg(clap::arg!(<FIRMWARE> "Firmware binary"))
                .arg_required_else_help(true),
        )
        .subcommand(
            clap::Command::new("sd")
                .about("Generate config and providion sd card")
                .args(&[
                    clap::arg!(<PRODUCT_CODE> "Product code"),
                    clap::arg!([SERIAL_ID] "Device serial ID e.g. zcts100054")
                ])
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("auto", sub_matches)) => {
            let argy = sub_matches.value_of("PRODUCT_CODE").expect("required");
            println!(
                "Automatic configuration of product: {}",
                &argy
            );
            return Args{ cmd: Cmd::Auto, arg: String::from(argy) };
        }
        Some(("photon", sub_matches)) => {
            let argy = sub_matches.value_of("FIRMWARE").expect("required");
            println!(
                "Photon firmware and device-OS flash version: {}",
                &argy
            );
            return Args{ cmd: Cmd::Photon, arg: String::from(argy) };
        }
        Some(("sd", sub_matches)) => {
            let argy = sub_matches.value_of("PRODUCT_CODE").expect("required");
            println!(
                "Config generation and sd card provisioning for: {}",
                &argy
            );
            return Args{ cmd: Cmd::SD, arg: String::from(argy) };
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
            return Args{ cmd: Cmd::None, arg: String::from("None") };
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

#[allow(dead_code)]
fn capture_command_output(cmd: String, args: String) {
    let output = process::Command::new(cmd)
        .arg(args)
        .stdout(process::Stdio::piped())
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
