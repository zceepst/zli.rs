use std::process; // run shell commands

mod config;
mod argparse;
mod particle;

#[allow(dead_code)]
#[derive(Debug)]
struct RunConfig {
    system: config::Config,
    program: argparse::Args,
}

use rusqlite;

fn main() {
    let conn = rusqlite::Connection::open("test.db").unwrap();

    conn.execute("CREATE TABLE product (
        code TEXT NOT NULL,
        type TEXT NOT NULL,
        product TEXT NOT NULL,
        config1 TEXT NOT NULL,
        config2 TEXT NOT NULL,
        config3 TEXT NOT NULL,
        config4 INTEGER NOT NULL,
        config5 TEXT NOT NULL,
        config6 TEXT NOT NULL,
        config7 TEXT NOT NULL,
        config8 INTEGER NOT NULL,
        config9 INTEGER NOT NULL,
        config10 FLOAT NOT NULL,
        config11 FLOAT NOT NULL,
        config12 FLOAT NOT NULL
    ", []).unwrap();

    let _code = String::from("CHRGT-TB-16-K-OV");
    let _type = String::from("Charging Trolley");
    let _prod = String::from("Tablet");
    let c1 = String::from("W");
    let c2 = String::from("D");
    let c3 = String::from("E");
    let c4 = 4;
    let c5 = String::from("TTB16KOV");
    let c6 = String::from("C");
    let c7 = String::from("E");
    let c8 = 16;
    let c9 = 1;
    let c10 = 0.011;
    let c11 = 0.064;
    let c12 = 0.000001;

    conn.execute("INSERT INTO person (code, type, product, )", &[_code, _type, _prod]);
}

// fn main() {
//     let program: argparse::Args = argparse::arg_parse();
//     let system: config::Config = config::configure();
//     let config: RunConfig = RunConfig{ system, program }; // runtime config
//     // println!("Run Config Structure:\n{:?}", config);

//     match config.program.cmd {
//         argparse::Cmd::Auto => {
//            let _placeholder = config.program.arg1;
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
