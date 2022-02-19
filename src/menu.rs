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
    zli auto -v --trolley CHRGT-TB-40-K-OV	Configures photon and SD for a trolley with verbose logs
    zli auto --powerhub HUB4D-L100-2M2U-UK	Configure and provision a powerhub photon and SD card");
}

// photon command help menu
pub fn photon() {
    println!("Configure photon device
Usage: zli photon [options] [file]

Global Options:
    -v, --verbose   Increases how much logging to display
    -q, --quiet     Decreases how much logging to display

Example:
	zli photon zioxi-powerhub4-v526.bin		Flash powerhub binary file to photon target");
}

pub fn sd() {
	println!("Provision SD card
Usage: zli sd [options] [product code]

Global Options:
    -v, --verbose   Increases how much logging to display
    -q, --quiet     Decreases how much logging to display

Example:
	zli sd HUB4D-L100-2M2U-UK				Generate config files then provisions files to SD card");
}
