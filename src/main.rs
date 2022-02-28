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

fn main() {
    let program: argparse::Args = argparse::arg_parse();
    let system: config::Config = config::configure();
    let config: RunConfig = RunConfig{ system, program }; // runtime config
    // println!("Run Config Structure:\n{:?}", config);

    match config.program.cmd {
        argparse::Cmd::Auto => {
           let _placeholder = config.program.arg1;
        },
        argparse::Cmd::Photon => {},
        argparse::Cmd::SD => {},
        argparse::Cmd::None => {},
    }

}
// particle::flash_device_os("2.2.0");

#[allow(dead_code)]
fn shell_command(cmd: String, args: String) -> String {
    let output = process::Command::new(cmd)
        .arg(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();
    return String::from_utf8(output.stdout).unwrap();
}
