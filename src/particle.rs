// Run particle shell commands as a process with captured output

use std::process;
use std::thread;
use std::time;

// maps product code to default (latest) firmware version
pub fn default_firmware(code: String) -> String {
    return String::from("2.2.0");
}

fn usb_state(state: &str) {
    let out = process::Command::new("particle")
        .arg("usb")
        .arg(state)
        .arg("-v")
        .stdout(process::Stdio::piped())
        .output()
        .unwrap()
        .stdout;
    println!("Usb state: {:?}", String::from_utf8(out).unwrap());
}

// bin/bin/photon-os-<version>/photon-<part>@<version>.bin vector for each parts
pub fn device_os_paths(version: &str) -> Vec<String> {
    let mut base: String = String::from("/home/pb/Documents/fun/rusty/zli/bin/photon-os-");
    base.push_str(version);
    base.push('/');

    // boot done separately due to +lto tag
    let mut vec: Vec<String> = vec![
        base.clone(),
        base.clone(),
        base.clone(),
        base.clone()
    ]; // copy contents of base into each element
    let parts = vec![
        "photon-system-part1@",
        "photon-system-part2@",
        "photon-tinker@",
        "photon-bootloader@"
    ];
    let format = vec![".bin", ".bin", ".bin", "+lto.bin"];

    for i in 0..4 {
        vec[i].push_str(parts[i]);
        vec[i].push_str(version);
        vec[i].push_str(format[i]);
    }

    return vec;
}

fn flash_part(part: &str, interface: Vec<&str>) {
    println!("Flashing:\n{:?}", part);
    let out = process::Command::new("particle")
        .arg("flash")
        .args(interface)
        .arg(part)
        .arg("-v")
        .stdout(process::Stdio::piped())
        .output()
        .unwrap()
        .stdout;
    println!("{:?}", String::from_utf8(out).unwrap())
}

pub fn flash_device_os(version: &str) {
    let paths: Vec<String> = device_os_paths(version);

    usb_state("dfu"); // device dfu mode
    println!("\n---0\n");

    flash_part(&paths[0][..], vec!["--usb"]);     // system part 1
    println!("\n---1\n");

    flash_part(&paths[1][..], vec!["--usb"]);     // system part 2
    println!("\n---2\n");

    flash_part(&paths[2][..], vec!["--usb"]);     // tinker
    println!("\n---3\n");

    let pause = time::Duration::from_millis(2000); // 2 seconds pause
    thread::sleep(pause); // necessary pre-serial state pause

    usb_state("start-listening"); // device serial mode
    println!("\n---4\n");

    flash_part(&paths[3][..], vec!["--serial", "--yes"]);  // bootloader
    println!("\n---5\n");
}
