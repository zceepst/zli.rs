/* menu module for zli */

pub fn error() {
    println!("Command not found\n");
}

// command line argument help print
pub fn help() {
    println!("zli   A command line product setup tool

Usage:  zli <command>
Help:   zli help <command>

Commands:
    auto    'One-shot' product setup
    photon  Configure photon device
    sd      Provision SD card");
}

// auto command help menu
pub fn auto() {
    println!("'One-shot' product setup
Usage: zli auto [options]

Global Options:
    -v, --verbose   Increases how much logging to display
    -q, --quiet     Decreases how much logging to display

Options:
    --trolley       Charging trolley/cabinet product
    --powerhub      Powerhub charging product

Examples:
    zli -v --trolley 'CHRGT-TB-40-K-OV'     Configures photon and SD card
                                            for trolley with verbose logs
    zli --powerhub 'HUB4D-L100-2M2U-UK'     Configure and provision a
                                            powerhub photon and SD card");
}

// photon command help menu
pub fn photon() {
    println!("Configure photon device
Usage: zli photon [options]

Global Options:
    -v, --verbose   Increases how much logging to display
    -q, --quiet     Decreases how much logging to display

Options:
    --
");
}

pub fn sd() {
	println!("Wrong!");
}
