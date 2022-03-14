use std::{clone, process}; // run shell commands
use serde_json;
mod config;
mod argparse;
mod particle;
mod products;
use rusqlite::{
    params,
    Connection,
    Result
};

#[allow(dead_code)]
#[derive(Debug)]
struct RunConfig {
    system: config::Config,
    program: argparse::Args,
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    // String::from("HUB4T-L150-4M-EU")
    products::main_config_generator(
        String::from("CHRGC-LS-30-K-OV")
    );
    // let data: serde_json::Value = products::products().unwrap();
    // println!("{:#?}", data);

    // let tcodes: Vec<String> = products::unpack_json(data, "pcodes");
    // println!("{:#?}", tcodes);
}

// fn main() {
//     let program: argparse::Args = argparse::arg_parse();
//     let system: config::Config = config::configure();
//     let config: RunConfig = RunConfig{ system, program }; // runtime config
//     // println!("Run Config Structure:\n{:?}", config);
//     match config.program.cmd {
//         argparse::Cmd::Auto => {
//             let _placeholder = config.program.arg1;

//         },
//         argparse::Cmd::Photon => {},
//         argparse::Cmd::SD => {},
//         argparse::Cmd::None => {},
//     }
// }

// fn isin_test(testvec: Vec<String>, item: &str) {
//     let testvec = vec![
//         String::from("no"),
//         String::from("no"),
//         String::from("no"),
//         String::from("no"),
//         String::from("no"),
//     ];
//     // let test: bool = products::isin(String::from("yes"), testvec);
//     let test: bool = testvec.iter().any(|e| e == &String::from("no"));
//     println!("{:?}", test);
// }

#[allow(dead_code)]
fn shell_command(cmd: String, args: String) -> String {
    let output = process::Command::new(cmd)
        .arg(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();
    return String::from_utf8(output.stdout).unwrap();
}
