// use std::env; // command line argument invocation
use std::fs; // file to string parsing
use std::process; // run shell commands
use std::path; // directory tree utils
use std::io::Write; // write to files
use std::env;

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
    arg1: String,
    arg2: String,
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
    particle_usb("dfu");
    // let out = process::Command::new("particle")
    //     .arg("usb")
    //     .arg("dfu")
    //     .arg("-v")
    //     .stdout(process::Stdio::piped())
    //     .output()
    //     .unwrap();
    // let stdout = String::from_utf8(out.stdout).unwrap();
    // println!("Command output:\n{}", stdout);

    // let args: Args = arg_parse(); // user arguments parsed
    // println!("Output:\n{:?}\n{:?}", args.arg1, args.arg2);
    // let out = process::Command::new("pwd")
    //     .arg(".")
    //     .stdout(process::Stdio::piped())
    //     .output()
    //     .unwrap();
    // let stdout = String::from_utf8(out.stdout).unwrap();
    // println!("Command output:\n{}", stdout);

    // particle_device_state("reset");
    // particle_device_state("dfu");
    // flash_device_os("--usb", "photon-system-part1","2.1.0");
    // flash_device_os("--usb", "photon-system-part2","2.1.0");
    // flash_device_os("--usb", "photon-tinker","2.1.0");
    // particle_device_state("reset");
    // particle_device_state("start-listening --yes");
    // let _config = configure();
    // parse_args();
    // menu::auto();
    // test_print(config);
    // capture_command_output(String::from("pwd"), String::
}

fn particle_usb(state: &str) {
    let out = process::Command::new("particle")
        .arg("usb")
        .arg(state)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();
}

fn particle_device_state(state: &str) {
    let mut command: String = String::from("particle usb ");
    command.push_str(state);
    process::Command::new(command);
}

// builds: particle flash <flag> bin/bin/photon-os-<version>/photon-<part>@<version>.bin
fn build_flash_command(flag: &str, part: &str, version: &str) -> String {
    let mut command: String = String::from("particle flash ");
    command.push_str(flag);
    command.push(' ');
    command.push_str("./bin/bin/photon-os-");
    command.push_str(version);
    command.push('/');
    command.push_str(part);
    command.push('@');
    command.push_str(version);
    command.push_str(".bin");
    return command;
}

fn flash_device_os(flag: &str, part: &str, version: &str) {
    let cmd = build_flash_command(flag, part, version);
    let output = process::Command::new(cmd)
        .spawn()
        .unwrap();
    // let stdout = String::from_utf8(output.stdout).unwrap();
    // println!("Command output:\n{}", stdout);
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
            let code = sub_matches.value_of("PRODUCT_CODE").expect("required");
            let serial = sub_matches.value_of("SERIAL_ID").expect("required");
            println!(
                "Automatic configuration of product:\n{:?}\n{:?}",
                &code, &serial
            );
            return Args{
                cmd: Cmd::Auto,
                arg1: String::from(code),
                arg2: String::from(serial)
            };
        }
        Some(("photon", sub_matches)) => {
            let argy = sub_matches.value_of("FIRMWARE").expect("required");
            println!(
                "Photon firmware and device-OS flash version: {}",
                &argy
            );
            return Args{ cmd: Cmd::Photon, arg1: String::from(argy), arg2: String::from("None") };
        }
        Some(("sd", sub_matches)) => {
            let code = sub_matches.value_of("PRODUCT_CODE").expect("required");
            let serial = sub_matches.value_of("SERIAL_ID").expect("required");
            println!(
                "Config generation and sd card provisioning for:\n{:?}\n{:?}",
                &code, &serial
            );
            return Args{
                cmd: Cmd::Auto,
                arg1: String::from(code),
                arg2: String::from(serial)
            };
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
            return Args{
                cmd: Cmd::None,
                arg1: String::from("None"),
                arg2: String::from("None")
            };
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
