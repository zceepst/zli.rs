use std::process; // run shell commands

mod config;
mod argparse;
mod particle;

#[allow(dead_code)]
#[derive(Debug)]
struct RunConf {
    system: config::Config,
    program: argparse::Args,
}

fn main() {
    particle::flash_device_os("2.2.0");

    // let program: argparse::Args = argparse::arg_parse(); // user arguments parsed
    // let system: config::Config = config::configure();
    // let config: RunConf = RunConf{ system, program };
    // println!("Run Config Structure:\n{:?}", config);

    // let home = config::username();
    // println!("Home dir ls result:\n{:?}", home);
}

#[allow(dead_code)]
fn shell_command(cmd: String, args: String) -> String {
    let output = process::Command::new(cmd)
        .arg(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();
    return String::from_utf8(output.stdout).unwrap();
}
