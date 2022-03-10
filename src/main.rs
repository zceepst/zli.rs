use std::process; // run shell commands
use serde_json;
mod config;
mod argparse;
mod particle;
mod database;
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
    let data: serde_json::Value = database::products().unwrap();
    println!("{:?}", data["tcodes"][0]);
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

#[allow(dead_code)]
fn shell_command(cmd: String, args: String) -> String {
    let output = process::Command::new(cmd)
        .arg(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();
    return String::from_utf8(output.stdout).unwrap();
}
