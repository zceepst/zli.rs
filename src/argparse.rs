use clap;

#[derive(Debug)]
pub enum Cmd {
    Auto,
    Photon,
	SD,
    None,
}

#[derive(Debug)]
pub struct Args {
    cmd: Cmd,
    arg1: String,
    arg2: String,
}

pub fn arg_parse() -> Args {
	// Argument parser.
	// Parses command line arguments using `clap` crate.
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
